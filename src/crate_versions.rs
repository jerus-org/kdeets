use crate::Error;

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use tame_index::{
    external::{
        http::{request::Parts, Response},
        reqwest::blocking::ClientBuilder,
    },
    index::FileLock,
    IndexLocation, IndexUrl, KrateName, SparseIndex,
};

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct CrateVersions {
    #[clap(flatten)]
    logging: Verbosity,
    /// The name of the crate
    crate_: String,
    /// First version ever published. May be yanked.
    #[clap(short = 'e', long = "earliest")]
    earliest: bool,
    /// Returns crate version with the highest version number according to semver, but excludes pre-release and yanked versions.
    #[clap(short = 'n', long = "normal")]
    normal: bool,
    /// The highest version as per semantic versioning specification
    #[clap(short = 't', long = "top")]
    highest: bool,
    /// The last release by date, even if it’s yanked or less than highest version.
    #[clap(short = 'r', long = "recent")]
    recent: bool,
    /// List all versions of the crate
    #[clap(short = 'l', long = "list")]
    list: bool,
    /// List key values (equivalent to -entr)
    #[clap(short = 'k', long = "key")]
    key: bool,
    /// List all versions and key values (equivalent to -entrl)
    #[clap(short = 'a', long = "all")]
    all: bool,
}

impl CrateVersions {
    pub fn run(&self) -> Result<String, Error> {
        log::info!("Getting details for crate: {}", self.crate_);

        let crate_name = KrateName::crates_io(&self.crate_)?;

        let il = IndexLocation::new(IndexUrl::CratesIoSparse);
        let index = SparseIndex::new(il)?;

        let lock = FileLock::unlocked();
        let req = index.make_remote_request(crate_name, None, &lock)?;

        log::debug!("Constructed remote request: {:?}!", req);

        let (
            Parts {
                method,
                uri,
                version,
                headers,
                ..
            },
            _,
        ) = req.into_parts();

        let builder = ClientBuilder::new();
        let builder = builder.tls_built_in_root_certs(true);
        let client = builder.build()?;

        let mut req = client.request(method, uri.to_string());
        req = req.version(version);
        req = req.headers(headers);
        log::info!("Remote request for reqwest: {:#?}!", req);

        let resp = client.execute(req.build()?)?;
        log::info!("Response: {:#?}!", resp);

        let mut builder = Response::builder()
            .status(resp.status())
            .version(resp.version());

        builder
            .headers_mut()
            .unwrap()
            .extend(resp.headers().iter().map(|(k, v)| (k.clone(), v.clone())));

        let body = resp.bytes().unwrap();
        let response = builder.body(body.to_vec())?;

        let Some(index_crate) = index.parse_remote_response(crate_name, response, false, &lock)?
        else {
            return Err(Error::CrateNotFoundonIndex);
        };

        let mut output = format!(
            "\n {}",
            format!("Crate versions for {}.", index_crate.name().cyan()).bold()
        );

        let mut i = 0;
        let mut line = String::from(" ");

        while i < 20 + index_crate.name().len() {
            line.push('🭶');
            i += 1;
        }

        output = format!("{}\n{}\n", output, line);

        if self.earliest | self.all | self.key {
            output = format!(
                "{}   Earliest version: {}\n",
                output,
                index_crate.earliest_version().version
            );
        };

        if self.normal | self.all | self.key {
            output = format!(
                "{}   {}\n",
                output,
                format!(
                    "Highest normal version: {}",
                    index_crate
                        .highest_normal_version()
                        .unwrap_or_else(|| index_crate.highest_version())
                        .version
                )
                .blue()
            );
        };

        if self.highest | self.all | self.key {
            output = format!(
                "{}   {}\n",
                output,
                format!("Highest version: {}", index_crate.highest_version().version).green()
            );
        };

        if self.recent | self.all | self.key {
            output = format!(
                "{}   {}\n",
                output,
                format!(
                    "Most recent version: {}",
                    index_crate.most_recent_version().version
                )
                .yellow()
            );
        };

        if self.list | self.all {
            const BASE_HEADER: &str = " Yanked  Version ";

            let mut header = BASE_HEADER.to_string();

            let rows = index_crate
                .versions
                .iter()
                .map(|x| {
                    format!(
                        "   {}     {}",
                        if x.yanked { "Yes".red() } else { " No".green() },
                        x.version
                    )
                })
                .collect::<Vec<String>>();

            log::debug!("Rows: {:#?}!", rows);

            let max_row = &rows
                .iter()
                .map(|x| {
                    log::debug!("Line: `{}`, len: `{}`!", x, x.chars().count(),);
                    x.len() - 12
                })
                .max()
                .unwrap_or(BASE_HEADER.len());
            log::debug!("Max row length: {}!", max_row);

            while header.len() < *max_row {
                header = format!("{} ", header);
            }
            log::debug!("Output: {}!", output);
            log::debug!("Header: {}!", header);

            let rows = format!("   {}\n", rows.join("\n   "));

            output = format!("{}   {}\n{}", output, header.underlined(), rows);
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {

    use colorful::Colorful;
    use rstest::fixture;

    use crate::crate_versions::CrateVersions;

    #[fixture]
    fn header(#[default("some_crate")] name: &str) -> String {
        let output = format!(
            "\n {}",
            format!("Crate versions for {}.", name.cyan()).bold()
        );

        let mut i = 0;
        let mut line = String::from(" ");

        while i < 20 + name.len() {
            line.push('🭶');
            i += 1;
        }

        format!("{}\n{}\n", output, line)
    }

    #[fixture]
    fn earliest() -> String {
        "   Earliest version: 0.1.0\n".to_string()
    }

    #[fixture]
    fn highest_normal() -> String {
        format!("   {}\n", "Highest normal version: 0.2.1".blue())
    }

    #[fixture]
    fn highest() -> String {
        format!("   {}\n", "Highest version: 0.2.1".green())
    }

    #[fixture]
    fn recent() -> String {
        format!("   {}\n", "Most recent version: 0.2.1".yellow())
    }

    #[fixture]
    fn list() -> String {
        "   \u{1b}[4m Yanked  Version \u{1b}[0m\n      \u{1b}[38;5;2m No\u{1b}[0m     0.1.0\n      \u{1b}[38;5;2m No\u{1b}[0m     0.1.1\n      \u{1b}[38;5;2m No\u{1b}[0m     0.1.3\n      \u{1b}[38;5;2m No\u{1b}[0m     0.2.1\n".to_string()
    }

    #[test]
    fn test_run_earliest() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &earliest());

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            earliest: true,
            ..Default::default()
        };

        assert_eq!(crate_versions.crate_, "some_crate".to_string());
        assert!(crate_versions.earliest);
        assert!(!crate_versions.normal);
        assert!(!crate_versions.highest);
        assert!(!crate_versions.recent);
        assert!(!crate_versions.list);
        assert!(!crate_versions.all);
        assert!(!crate_versions.key);

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_normal() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &highest_normal());

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            normal: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_top() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &highest());

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            highest: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_recent() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &recent());

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            recent: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_list() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &list());

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            list: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_all() {
        let name = "some_crate";
        let expected = format!(
            "{}{}{}{}{}{}",
            header(name),
            &earliest(),
            &highest_normal(),
            &highest(),
            &recent(),
            &list()
        );

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            all: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_key() {
        let name = "some_crate";
        let expected = format!(
            "{}{}{}{}{}",
            header(name),
            &earliest(),
            &highest_normal(),
            &highest(),
            &recent(),
        );

        let crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            key: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_invalid_crate() {
        let crate_versions = CrateVersions {
            crate_: "some_non-existing_crate".to_string(),
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_err());
    }

    #[test]
    fn test_run_invalid_crate_earliest() {
        let crate_versions = CrateVersions {
            crate_: "sdc_apis".to_string(),
            earliest: true,
            ..Default::default()
        };

        let result = crate_versions.run();
        assert!(result.is_ok());
    }
}
