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
    #[clap(
        short,
        long,
        help = "Add dependencies based on specific version\n",
        default_value = "latest"
    )]
    dependencies: SelectVersion,
    /// The location for the local registry
    #[clap(short, long, default_value = "tests/local_registry")]
    location: String,
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

        let registry = if self.location.is_empty() {
            "tests/local_registry"
        } else {
            &self.location
        };
        log::debug!("Creating registry at {}", registry);
        let mut output = SetupTestOutputBuilder::new(index_crate.clone(), registry);

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_setup_run_with_latest_dependencies() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();

        let setup = Setup {
            crate_: String::from("forestry"),
            dependencies: SelectVersion::Latest,
            location: location.to_string(),
            ..Default::default()
        };
        let result = setup.run();
        println!("Result: {:?}", result);
        assert!(result.is_ok());
        assert!(Path::new("tests/registry_new").exists());
    }

    #[test]
    fn test_setup_run_with_earliest_dependencies() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();
        let setup = Setup {
            crate_: String::from("forestry"),
            dependencies: SelectVersion::Earlist,
            location: location.to_string(),
            ..Default::default()
        };
        let result = setup.run();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_run_with_highest_normal_dependencies() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();
        let setup = Setup {
            crate_: String::from("forestry"),
            dependencies: SelectVersion::HighestNormal,
            location: location.to_string(),
            ..Default::default()
        };
        let result = setup.run();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_run_with_no_dependencies() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();
        let setup = Setup {
            crate_: String::from("forestry"),
            dependencies: SelectVersion::None,
            location: location.to_string(),
            ..Default::default()
        };
        let result = setup.run();
        assert!(result.is_ok());
    }

    #[test]
    fn test_setup_run_nonexistent_crate() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();
        let setup = Setup {
            crate_: String::from("nonexistent_crate_12345"),
            dependencies: SelectVersion::Latest,
            location: location.to_string(),
            ..Default::default()
        };
        let result = setup.run();
        assert!(matches!(result, Err(Error::CrateNotFoundOnIndex)));
    }

    #[test]
    fn test_setup_run_with_no_replace_flag() {
        let _log = simple_logger::init_with_level(log::Level::Debug);
        let temp_dir = tempfile::tempdir().unwrap();
        let location = temp_dir.path().to_str().unwrap();
        let setup = Setup {
            crate_: String::from("forestry"),
            dependencies: SelectVersion::Latest,
            no_replace: false,
            location: location.to_string(),
            ..Default::default()
        };

        // First run should succeed
        let result1 = setup.run();
        assert!(result1.is_ok());

        // Second run with no_replace should still succeed
        let result2 = setup.run();
        assert!(result2.is_ok());
    }
}
