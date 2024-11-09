use crate::Error;

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use semver::{Version, VersionReq};
use smol_str::SmolStr;
use tame_index::{KrateName, SparseIndex};

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

        let index = crate::get_sparce_index()?;
        let index_crate = crate::get_index_crate(&index, KrateName::crates_io(&self.crate_)?)?;

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

        output = format!(
            "{}   {}\n",
            output,
            format!(
                "Most recent version: {} (Rust version: {})",
                index_crate.most_recent_version().version,
                if let Some(rv) = &index_crate.most_recent_version().rust_version {
                    rv.to_string()
                } else {
                    "not specified".to_string()
                }
            )
            .blue()
            .bold()
        );

        let mut rust_versions = vec![];

        let deps = index_crate.most_recent_version().dependencies();
        for dep in deps {
            let rust_version =
                get_rust_version(&index, dep.crate_name(), dep.version_requirement())?;
            rust_versions.push(rust_version.clone());
            output = format!(
                "{}    {}   {}  {:?}\n",
                output,
                dep.crate_name(),
                dep.version_requirement(),
                rust_version,
            );
        }

        let minimum_rust = rust_versions
            .iter()
            .filter_map(|rv_opt| rv_opt.as_ref())
            .max();
        output = format!("{}    Minimum Rust version: {:?}", output, minimum_rust);

        let any_none = rust_versions.iter().any(|rv| rv.is_none());
        if any_none {
            output = format!(
                "{} ({})",
                output,
                "WARNING: Some dependencies do not specify a Rust version".yellow(),
            );
        }
        output = format!("{}\n", output);

        Ok(output)
    }
}

fn get_rust_version(
    index: &SparseIndex,
    name: &str,
    version_reference: VersionReq,
) -> Result<Option<SmolStr>, Error> {
    let crate_name = KrateName::crates_io(name)?;

    let index_crate = crate::get_index_crate(index, crate_name)?;

    for version in index_crate.versions {
        if version_reference.matches(&Version::parse(&version.version)?) {
            return Ok(version.rust_version);
        }
    }

    Ok(None)
}

// Forestry - single dependency with rust_version specified
// walkdir - No dependency has rust_version specified

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
