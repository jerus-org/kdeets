use std::{fmt::Display, fs};

use crate::{get_client_builder, Error, LINE_CHAR, SETUP_HEADER};

use clap::{Parser, ValueEnum};
use clap_verbosity::Verbosity;
use colorful::Colorful;
use tame_index::{
    index::{
        local::{builder::Client, LocalRegistryBuilder, ValidKrate},
        FileLock, RemoteSparseIndex,
    },
    IndexKrate, KrateName, PathBuf,
};

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
        let index = crate::get_sparce_index()?;
        let client = crate::get_client_builder().build()?;

        let remote_index = RemoteSparseIndex::new(index, client);
        let crate_name = KrateName::crates_io(&self.crate_)?;

        let index_crate = remote_index.krate(crate_name, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        let mut output = SetupTestOutput::new(index_crate.clone(), "tests/registry_new");

        output.initialise_local_registry(self.no_replace)?;

        output.insert_crate(&index_crate)?;

        let final_output = output.finalize()?;

        Ok(final_output.to_string())
    }
}

struct SetupTestOutput {
    #[allow(dead_code)]
    index_crate: IndexKrate,
    header: String,
    registry_path: PathBuf,
    registry: OutputRegistry,
    crates: Vec<String>,
}

impl SetupTestOutput {
    fn new(index_crate: IndexKrate, registry_path: &str) -> Self {
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

        let registry_path = PathBuf::from(registry_path);

        Self {
            index_crate,
            header,
            registry_path,
            registry: OutputRegistry::None,
            crates: Vec::new(),
        }
    }

    fn initialise_local_registry(&mut self, no_replace: bool) -> Result<(), Error> {
        let registry_path = self.registry_path.clone();

        let registry_builder = match LocalRegistryBuilder::create(registry_path.clone()) {
            Ok(registry) => registry,
            Err(e) => {
                if no_replace {
                    return Err(Error::TameIndex(e));
                } else {
                    log::warn!("Registry already exists, replacing.");
                    fs::remove_dir_all(&registry_path)?;
                    LocalRegistryBuilder::create(registry_path.clone())?
                }
            }
        };
        log::debug!("Created registry at {}", registry_path);
        self.registry = OutputRegistry::Builder(registry_builder);
        Ok(())
    }

    fn insert_crate(&mut self, index_crate: &IndexKrate) -> Result<(), Error> {
        let OutputRegistry::Builder(registry_builder) = &mut self.registry else {
            return Err(Error::LocalRegistryBuilderNotSet);
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

        registry_builder.insert(index_crate, &krates)?;
        log::debug!("Inserted crate {} into registry", index_crate.name());
        self.crates.push(index_crate.name().to_string());
        Ok(())
    }

    fn finalize(self) -> Result<Self, Error> {
        if !matches!(self.registry, OutputRegistry::Builder(_)) {
            return Err(Error::LocalRegistryBuilderNotSet);
        };

        let OutputRegistry::Builder(registry_builder) = self.registry else {
            return Err(Error::LocalRegistryBuilderNotSet);
        };

        let temp = registry_builder;

        let _local_registry = temp.finalize(true)?;

        Ok(Self {
            index_crate: self.index_crate,
            header: self.header,
            registry_path: self.registry_path,
            registry: OutputRegistry::Registry,
            crates: self.crates,
        })
    }
}

enum OutputRegistry {
    None,
    Builder(LocalRegistryBuilder),
    Registry,
}

impl Display for SetupTestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        if !self.crates.is_empty() {
            write!(f, "  Crates added:\n    ")?;
            self.crates.join("\n    ").fmt(f)?;
        };
        Ok(())
    }
}
