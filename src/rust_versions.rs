use std::fmt::Display;

use crate::{Error, HEADER, LINE_CHAR};

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use semver::{Version, VersionReq};
use smol_str::SmolStr;
use tame_index::{IndexKrate, KrateName, SparseIndex};

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

        let mut output = RustVersionOutput::new(&index_crate);

        output.set_rust_version(&index_crate)?;

        output.set_minimum_rust_version_required(&index_crate, &index)?;

        Ok(output.to_string())
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

#[derive(Debug, Default)]
struct RustVersionOutput {
    header: String,
    rust_version: Option<String>,
    minimum_required_rust: Option<String>,
}

impl RustVersionOutput {
    fn new(index_crate: &IndexKrate) -> Self {
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
            header,
            ..Default::default()
        }
    }

    fn set_rust_version(&mut self, index_crate: &IndexKrate) -> Result<(), Error> {
        let mut rust_version = String::from("    Most recent version: ");
        rust_version.push_str(
            index_crate
                .most_recent_version()
                .version
                .to_string()
                .as_str(),
        );
        rust_version.push_str(" (Rust version: ");
        let rv = if let Some(rv) = &index_crate.most_recent_version().rust_version {
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

    fn set_minimum_rust_version_required(
        &mut self,
        index_crate: &IndexKrate,
        index: &SparseIndex,
    ) -> Result<(), Error> {
        let mut rust_versions = vec![];

        let deps = index_crate.most_recent_version().dependencies();
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
        minimum_required_rust.push_str(minimum_rust.unwrap().to_string().as_str());

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
