use crate::Error;

use clap::{Parser, ValueEnum};
use clap_verbosity::Verbosity;
use disksize::DiskSize;
use output::SetupTestOutputBuilder;
use tame_index::{index::FileLock, KrateName};

mod disksize;
mod output;

#[derive(Debug, Parser, Default, ValueEnum, Clone)]
enum SelectVersion {
    #[default]
    Latest,
    Highest,
    HighestNormal,
    Earlist,
    None,
}

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Setup {
    #[clap(flatten)]
    logging: Verbosity,
    /// Do not replace the existing registry if it exists
    #[clap(default_value = "false", short = 'r', long)]
    no_replace: bool,
    /// Add dependencies based on specific version
    #[clap(short, long, default_value = "latest")]
    dependencies: SelectVersion,
    /// The name of the crate
    crate_: String,
}

impl Setup {
    pub fn run(&self) -> Result<String, Error> {
        log::info!(
            "Setting up local registry and adding crate: {}",
            self.crate_
        );
        let lock = FileLock::unlocked();

        let combo_index = crate::get_remote_combo_index()?;
        let crate_name = KrateName::crates_io(&self.crate_)?;

        let index_crate = combo_index.krate(crate_name, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        let mut output = SetupTestOutputBuilder::new(index_crate.clone(), "tests/registry_new");

        output.initialise_local_registry(self.no_replace)?;

        output.insert_crate(&index_crate)?;

        match self.dependencies {
            SelectVersion::Latest => {
                log::debug!("Adding dependencies for most recent version");
                output.add_dependency_crates(
                    index_crate.most_recent_version().dependencies(),
                    &combo_index,
                )?;
            }
            SelectVersion::Earlist => {
                log::debug!("Adding dependencies for earliest version");
                output.add_dependency_crates(
                    index_crate.earliest_version().dependencies(),
                    &combo_index,
                )?;
            }
            SelectVersion::Highest => {
                log::debug!("Adding dependencies for highest version");
                output.add_dependency_crates(
                    index_crate.highest_version().dependencies(),
                    &combo_index,
                )?;
            }
            SelectVersion::HighestNormal => {
                log::debug!("Attempting to add dependencies for highest normal version");
                let opt_index_version = index_crate.highest_normal_version();
                if let Some(index_version) = opt_index_version {
                    output.add_dependency_crates(index_version.dependencies(), &combo_index)?;
                } else {
                    log::warn!("No normal version found for crate: {}", self.crate_);
                };
            }
            SelectVersion::None => (),
        };

        let final_output = output.finalize()?;

        Ok(final_output.to_string())
    }
}
