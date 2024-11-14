use std::{fmt::Display, fs};

use crate::{get_client_builder, Error, LINE_CHAR, SETUP_HEADER};

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use tame_index::{
    index::{
        local::{builder::Client, LocalRegistryBuilder, ValidKrate},
        FileLock, RemoteSparseIndex,
    },
    IndexKrate, KrateName, PathBuf,
};

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Setup {
    #[clap(flatten)]
    logging: Verbosity,
    #[clap(default_value = "false", short = 'r', long)]
    /// Do not replace the existing registry if it exists
    no_replace: bool,
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
        let index = crate::get_sparce_index()?;
        let client = crate::get_client_builder().build()?;

        let remote_index = RemoteSparseIndex::new(index, client);
        let crate_name = KrateName::crates_io(&self.crate_)?;

        let index_crate = remote_index.krate(crate_name, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        let output = SetupTestOutput::new(index_crate.clone());
        let registry_path = PathBuf::from("tests/registry_new");

        let registry = match LocalRegistryBuilder::create(registry_path.clone()) {
            Ok(registry) => registry,
            Err(e) => {
                if !self.no_replace {
                    return Err(Error::TameIndex(e));
                } else {
                    log::warn!("Registry already exists, replacing.");
                    fs::remove_dir_all(&registry_path)?;
                    LocalRegistryBuilder::create(registry_path.clone())?
                }
            }
        };

        let client = Client::build(get_client_builder())?;

        let index = crate::get_sparce_index()?;

        let index_config = index.index_config()?;

        let mut krates = vec![];

        for version in &index_crate.versions {
            log::debug!("Downloaded for version {}", version.version);
            let krate = ValidKrate::download(&client, &index_config, version)?;
            krates.push(krate);
        }

        registry.insert(&index_crate, &krates)?;

        registry.finalize(true)?;

        Ok(output.to_string())
    }
}

struct SetupTestOutput {
    #[allow(dead_code)]
    index_crate: IndexKrate,
    header: String,
}

impl SetupTestOutput {
    fn new(index_crate: IndexKrate) -> Self {
        let mut header = String::from("\n  ");
        header.push_str(SETUP_HEADER);
        header.push(' ');
        header.push_str(index_crate.name().cyan().to_string().as_str());
        header.push('.');
        header.push_str("\n  ");
        let mut i = 0;
        while i < SETUP_HEADER.len() + 2 + index_crate.name().len() {
            header.push(LINE_CHAR);
            i += 1;
        }
        header.push('\n');

        Self {
            index_crate,
            header,
        }
    }
}

impl Display for SetupTestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        Ok(())
    }
}
