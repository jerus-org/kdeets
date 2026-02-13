use crate::Error;

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use smol_str::SmolStr;
use tame_index::{IndexKrate, KrateName, index::FileLock};

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct CrateVersions {
    #[clap(flatten)]
    logging: Verbosity,
    /// The name of the crate
    crate_: String,
    /// Display bare version number without text for recent, highest normal, higest or earliest version.
    #[clap(short = 'b', long = "bare")]
    bare: bool,
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
    /// List key values (equivalent to `-entr`)
    #[clap(short = 'k', long = "key")]
    key: bool,
    /// List all versions and key values (equivalent to `-entrl`)
    #[clap(short = 'a', long = "all")]
    all: bool,

    #[clap(skip)]
    output: String,
}

impl CrateVersions {
    pub fn run(&mut self, no_colour: bool) -> Result<String, Error> {
        log::info!("Getting details for crate: {}", self.crate_);
        let lock = FileLock::unlocked();
        let index = crate::get_remote_combo_index()?;
        let index_crate = index.krate(KrateName::crates_io(&self.crate_)?, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        if self.bare {
            self.output = if self.recent {
                index_crate.most_recent_version().version.to_string()
            } else if self.highest {
                index_crate.highest_version().version.to_string()
            } else if self.normal {
                index_crate
                    .highest_normal_version()
                    .unwrap_or_else(|| index_crate.highest_version())
                    .version
                    .to_string()
            } else {
                index_crate.earliest_version().version.to_string()
            }
        } else {
            self.append_header(no_colour, index_crate.name());

            if self.earliest | self.all | self.key {
                let description = "Earliest version";
                let version = &index_crate.earliest_version().version;
                let colour = TextColour::None;
                self.append_specific_version(description, version, colour);
            };

            if self.normal | self.all | self.key {
                let description = "Highest normal version";
                let version = &index_crate
                    .highest_normal_version()
                    .unwrap_or_else(|| index_crate.highest_version())
                    .version;
                let colour = if no_colour {
                    TextColour::None
                } else {
                    TextColour::Blue
                };
                self.append_specific_version(description, version, colour);
            };

            if self.highest | self.all | self.key {
                let description = "Highest version";
                let version = &index_crate.highest_version().version;
                let colour = if no_colour {
                    TextColour::None
                } else {
                    TextColour::Green
                };
                self.append_specific_version(description, version, colour);
            };

            if self.recent | self.all | self.key {
                let description = "Most recent version";
                let version = &index_crate.most_recent_version().version;
                let colour = if no_colour {
                    TextColour::None
                } else {
                    TextColour::Yellow
                };
                self.append_specific_version(description, version, colour);
            };

            if self.list | self.all {
                self.append_list(index_crate, no_colour);
            }
        };

        Ok(self.output.to_string())
    }

    fn append_header(&mut self, no_colour: bool, crate_name: &str) {
        let output = format!(
            "\n {}",
            if no_colour {
                format!("Crate versions for {crate_name}.")
            } else {
                format!("Crate versions for {}.", crate_name.cyan())
                    .bold()
                    .to_string()
            }
        );

        let mut i = 0;
        let mut line = String::from(" ");

        while i < 20 + crate_name.len() {
            line.push('🭶');
            i += 1;
        }

        self.output = format!("{output}\n{line}\n");
    }

    fn append_specific_version(
        &mut self,
        description: &str,
        version: &SmolStr,
        colour: TextColour,
    ) {
        let addition = format!("   {description}: {version}\n");
        let addition = colour.paint(addition);
        self.output = format!("{}{}", self.output, addition)
    }

    fn append_list(&mut self, index_crate: IndexKrate, no_colour: bool) {
        const BASE_HEADER: &str = " Yanked  Version ";

        let mut header = BASE_HEADER.to_string();

        let rows = index_crate
            .versions
            .iter()
            .map(|x| {
                format!(
                    "   {}     {}",
                    match (x.yanked, no_colour) {
                        (true, true) => "Yes".to_string(),
                        (false, true) => " No".to_string(),
                        (true, false) => "Yes".red().to_string(),
                        (false, false) => " No".green().to_string(),
                    },
                    x.version
                )
            })
            .collect::<Vec<String>>();

        log::debug!("Rows: {rows:#?}!");

        let max_row = &rows
            .iter()
            .map(|x| {
                log::debug!("Line: `{}`, len: `{}`!", x, x.chars().count(),);
                x.len() - 12
            })
            .max()
            .unwrap_or(BASE_HEADER.len());
        log::debug!("Max row length: {max_row}!");

        while header.len() < *max_row {
            header = format!("{header} ");
        }
        log::debug!("Output: {}!", self.output);
        log::debug!("Header: {header}!");

        let rows = format!("   {}\n", rows.join("\n   "));

        self.output = format!(
            "{}   {}\n{}",
            self.output,
            if no_colour {
                header.to_string()
            } else {
                header.underlined().to_string()
            },
            rows
        );
    }
}

enum TextColour {
    None,
    Blue,
    Green,
    Yellow,
}

impl TextColour {
    fn paint(&self, text: String) -> String {
        match self {
            TextColour::None => text,
            TextColour::Blue => text.blue().to_string(),
            TextColour::Green => text.green().to_string(),
            TextColour::Yellow => text.yellow().to_string(),
        }
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

        format!("{output}\n{line}\n")
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

        let mut crate_versions = CrateVersions {
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

        let result = crate_versions.run(false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_normal() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &highest_normal());

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            normal: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_top() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &highest());

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            highest: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_recent() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &recent());

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            recent: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_list() {
        let name = "some_crate";
        let expected = format!("{}{}", header(name), &list());

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            list: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
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

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            all: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
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

        let mut crate_versions = CrateVersions {
            crate_: "some_crate".to_string(),
            key: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_run_invalid_crate() {
        let mut crate_versions = CrateVersions {
            crate_: "some_non-existing_crate".to_string(),
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_invalid_crate_earliest() {
        let mut crate_versions = CrateVersions {
            crate_: "sdc_apis".to_string(),
            earliest: true,
            ..Default::default()
        };

        let result = crate_versions.run(false);
        assert!(result.is_ok());
    }
}
