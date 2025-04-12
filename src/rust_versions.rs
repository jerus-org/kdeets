use std::fmt::Display;

use crate::{Error, HEADER, LINE_CHAR};

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use semver::{Version, VersionReq};
use smol_str::SmolStr;
use tame_index::{
    IndexKrate, KrateName,
    index::{ComboIndex, FileLock},
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
        let lock = FileLock::unlocked();
        let index = crate::get_remote_combo_index()?;
        let index_crate = index.krate(KrateName::crates_io(&self.crate_)?, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        let mut output = RustVersionOutput::new(index_crate);

        output.set_rust_version()?;

        output.set_minimum_rust_version_required(&index)?;

        Ok(output.to_string())
    }
}

fn get_rust_version(
    index: &ComboIndex,
    name: &str,
    version_reference: VersionReq,
) -> Result<Option<SmolStr>, Error> {
    let crate_name = KrateName::crates_io(name)?;
    let lock = FileLock::unlocked();
    let index_crate = index.krate(crate_name, true, &lock)?;

    let Some(index_crate) = index_crate else {
        return Err(Error::CrateNotFoundOnIndex);
    };

    for version in index_crate.versions {
        if version_reference.matches(&Version::parse(&version.version)?) {
            return Ok(version.rust_version);
        }
    }

    Ok(None)
}

#[derive(Debug)]
struct RustVersionOutput {
    index_crate: IndexKrate,
    header: String,
    rust_version: Option<String>,
    minimum_required_rust: Option<String>,
}

impl RustVersionOutput {
    fn new(index_crate: IndexKrate) -> Self {
        let mut header = String::from("\n  ");
        header.push_str(HEADER);
        header.push(' ');
        header.push_str(index_crate.name().cyan().to_string().as_str());
        header.push('.');
        header.push_str("\n  ");
        let mut i = 0;
        while i < HEADER.len() + 2 + index_crate.name().len() {
            header.push(LINE_CHAR);
            i += 1;
        }
        header.push('\n');

        Self {
            index_crate,
            header,
            rust_version: None,
            minimum_required_rust: None,
        }
    }

    fn set_rust_version(&mut self) -> Result<(), Error> {
        let mut rust_version = String::from("    Most recent version: ");
        rust_version.push_str(
            self.index_crate
                .most_recent_version()
                .version
                .to_string()
                .as_str(),
        );
        rust_version.push_str(" (Rust version: ");
        let rv = if let Some(rv) = &self.index_crate.most_recent_version().rust_version {
            rv.to_string()
        } else {
            "not specified".to_string()
        }
        .blue()
        .bold()
        .to_string();
        rust_version.push_str(&rv);
        rust_version.push_str(")\n");

        self.rust_version = Some(rust_version);

        Ok(())
    }

    fn set_minimum_rust_version_required(&mut self, index: &ComboIndex) -> Result<(), Error> {
        let mut rust_versions = vec![];

        let deps = self.index_crate.most_recent_version().dependencies();
        for dep in deps {
            let rust_version =
                get_rust_version(index, dep.crate_name(), dep.version_requirement())?;
            rust_versions.push(rust_version.clone());
            log::debug!(
                "    {}   {}  {:?}\n",
                dep.crate_name(),
                dep.version_requirement(),
                rust_version,
            );
        }

        let minimum_rust = rust_versions
            .iter()
            .filter_map(|rv_opt| rv_opt.as_ref())
            .max();

        let mut minimum_required_rust = String::from("    Minimum Rust version: ");
        if let Some(minimum_rust) = minimum_rust {
            minimum_required_rust.push_str(minimum_rust.to_string().as_str());
        } else {
            minimum_required_rust.push_str("not specified");
        }

        if rust_versions.iter().any(|rv| rv.is_none()) {
            minimum_required_rust.push_str(" (");
            minimum_required_rust.push_str(
                " (WARNING: Some dependencies do not specify a Rust version)"
                    .yellow()
                    .to_string()
                    .as_str(),
            );
            minimum_required_rust.push(')');
        }
        minimum_required_rust.push('\n');

        self.minimum_required_rust = Some(minimum_required_rust);

        Ok(())
    }
}

impl Display for RustVersionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        if let Some(rust_version) = &self.rust_version {
            write!(f, "{}", rust_version)?;
        }
        if let Some(minimum_required_rust) = &self.minimum_required_rust {
            write!(f, "{}", minimum_required_rust)?;
        }
        Ok(())
    }
}

// Forestry - single dependency with rust_version specified
// walkdir - No dependency has rust_version specified

#[cfg(test)]
mod tests {

    use super::*;

    use crate::rust_versions::RustVersions;
    use clap::Parser;

    #[test]
    fn test_rust_versions_new() {
        let rust_versions = RustVersions::parse_from(["program", "test-crate"]);
        assert_eq!(rust_versions.crate_, "test-crate");
    }

    #[test]
    fn test_rust_versions_empty_crate() {
        let result = RustVersions::try_parse_from(["program"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_rust_versions_with_verbose() {
        let rust_versions = RustVersions::parse_from(["program", "-v", "test-crate"]);
        assert_eq!(rust_versions.crate_, "test-crate");
    }

    #[test]
    fn test_rust_versions_with_quiet() {
        let rust_versions = RustVersions::parse_from(["program", "-q", "test-crate"]);
        assert_eq!(rust_versions.crate_, "test-crate");
    }

    #[test]
    fn test_rust_versions_with_multiple_flags() {
        let rust_versions = RustVersions::parse_from(["program", "-vv", "test-crate"]);
        assert_eq!(rust_versions.crate_, "test-crate");
    }

    #[test]
    fn test_add_crate_and_set_header() {
        let expected = "\n  Crate versions for \u{1b}[38;5;6mforestry\u{1b}[0m.\n  🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶\n";

        let (_temp_dir, registry) = crate::tests::get_temp_local_registry();
        let lock = FileLock::unlocked();
        let index = crate::tests::get_test_index(&registry).unwrap();
        let index_crate = index
            .krate(KrateName::crates_io("forestry").unwrap(), true, &lock)
            .unwrap()
            .unwrap();

        let output = RustVersionOutput::new(index_crate);

        assert_eq!(output.to_string(), expected);
    }

    #[test]
    fn test_set_rust_version_output_with_specified_version() {
        let expected = "\n  Crate versions for \u{1b}[38;5;6mforestry\u{1b}[0m.\n  🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶\n    Most recent version: 1.4.1 (Rust version: \u{1b}[38;5;4;1mnot specified\u{1b}[0m)\n";

        let (_temp_dir, registry) = crate::tests::get_temp_local_registry();
        let lock = FileLock::unlocked();
        let index = crate::tests::get_test_index(&registry).unwrap();
        let index_crate = index
            .krate(KrateName::crates_io("forestry").unwrap(), true, &lock)
            .unwrap()
            .unwrap();

        let mut output = RustVersionOutput::new(index_crate);
        output.set_rust_version().unwrap();

        assert_eq!(output.to_string(), expected);
    }

    #[test]
    fn test_set_rust_version_output_with_minimum_rust() {
        let expected = "\n  Crate versions for \u{1b}[38;5;6mforestry\u{1b}[0m.\n  🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶\n    Minimum Rust version: not specified (\u{1b}[38;5;3m (WARNING: Some dependencies do not specify a Rust version)\u{1b}[0m)\n";

        let (_temp_dir, registry) = crate::tests::get_temp_local_registry();
        let lock = FileLock::unlocked();
        let index = crate::tests::get_test_index(&registry).unwrap();
        let index_crate = index
            .krate(KrateName::crates_io("forestry").unwrap(), true, &lock)
            .unwrap()
            .unwrap();

        let mut output = RustVersionOutput::new(index_crate);

        output.set_minimum_rust_version_required(&index).unwrap();

        assert_eq!(output.to_string(), expected);
    }

    #[test]
    fn test_set_rust_version_output_with_specified_version_and_minimum_rust() {
        let expected = "\n  Crate versions for \u{1b}[38;5;6mforestry\u{1b}[0m.\n  🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶🭶\n    Most recent version: 1.4.1 (Rust version: \u{1b}[38;5;4;1mnot specified\u{1b}[0m)\n    Minimum Rust version: not specified (\u{1b}[38;5;3m (WARNING: Some dependencies do not specify a Rust version)\u{1b}[0m)\n";

        let (_temp_dir, registry) = crate::tests::get_temp_local_registry();
        let lock = FileLock::unlocked();
        let index = crate::tests::get_test_index(&registry).unwrap();
        let index_crate = index
            .krate(KrateName::crates_io("forestry").unwrap(), true, &lock)
            .unwrap()
            .unwrap();

        let mut output = RustVersionOutput::new(index_crate);
        output.set_rust_version().unwrap();
        output.set_minimum_rust_version_required(&index).unwrap();

        assert_eq!(output.to_string(), expected);
    }
}
