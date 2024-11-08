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
pub struct RustVersions {
    #[clap(flatten)]
    logging: Verbosity,
    /// The name of the crate
    crate_: String,
}

impl RustVersions {
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

        Ok(output)
    }
}

#[cfg(test)]
mod tests {

    use colorful::Colorful;
    use rstest::fixture;

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
}
