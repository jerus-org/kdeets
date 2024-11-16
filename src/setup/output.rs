use std::{fmt::Display, fs};

use colorful::Colorful;
use tame_index::{
    index::{
        local::{builder::Client, LocalRegistryBuilder, ValidKrate},
        FileLock, RemoteSparseIndex,
    },
    IndexDependency, IndexKrate, KrateName, PathBuf,
};

use crate::{Error, LINE_CHAR, SETUP_HEADER};

use super::{DiskSize, OutputRegistry};

pub(crate) struct SetupTestOutput {
    #[allow(dead_code)]
    index_crate: IndexKrate,
    header: String,
    registry_path: PathBuf,
    registry: OutputRegistry,
    crates: Vec<String>,
    total: DiskSize,
}

impl SetupTestOutput {
    pub(crate) fn new(index_crate: IndexKrate, registry_path: &str) -> Self {
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
            total: DiskSize::zero(),
        }
    }

    pub(crate) fn initialise_local_registry(&mut self, no_replace: bool) -> Result<(), Error> {
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

    pub(crate) fn insert_crate(&mut self, index_crate: &IndexKrate) -> Result<(), Error> {
        let OutputRegistry::Builder(registry_builder) = &mut self.registry else {
            return Err(Error::LocalRegistryBuilderNotSet);
        };

        let client = Client::build(crate::get_client_builder())?;
        let index = crate::get_sparce_index()?;
        let index_config = index.index_config()?;

        let mut krates = vec![];

        for version in &index_crate.versions {
            log::debug!("Downloaded for version {}", version.version);
            let krate = ValidKrate::download(&client, &index_config, version)?;
            krates.push(krate);
        }

        let written = registry_builder.insert(index_crate, &krates)?;
        self.total += written;
        log::debug!("Inserted crate {} into registry", index_crate.name());
        self.crates.push(index_crate.name().to_string());
        Ok(())
    }

    pub(crate) fn add_dependency_crates(
        &mut self,
        dependencies: &[IndexDependency],
        remote_index: &RemoteSparseIndex,
    ) -> Result<(), Error> {
        log::debug!("Adding {} dependencies", dependencies.len());
        for dependency in dependencies {
            let dependency_name = KrateName::crates_io(dependency.crate_name())?;
            let lock = FileLock::unlocked();
            let dependency_crate = remote_index.krate(dependency_name, true, &lock)?;
            if let Some(dependency_crate) = dependency_crate {
                self.insert_crate(&dependency_crate)?
            } else {
                log::warn!("Could not find dependency: {}, skipping.", dependency_name);
            };
        }
        Ok(())
    }

    pub(crate) fn finalize(self) -> Result<Self, Error> {
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
            total: self.total,
        })
    }
}

impl Display for SetupTestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        if !self.crates.is_empty() {
            write!(f, "  Crates added:\n    ")?;
            self.crates.join("\n    ").fmt(f)?;
        };
        write!(f, "\n  Total bytes written: {}\n", self.total)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CRATE: &str = "tests/registry/index/fo/re/forestry";
    const TEST_CRATE_ROOT: &str = "tests/registry/index/";

    fn make_index_path(name: &str) -> String {
        let first_two = name[..2].to_string();
        let second_two = name[2..4].to_string();

        let path_string = format!("{}{}/{}/{}", TEST_CRATE_ROOT, first_two, second_two, name);
        println!("Manufactured string is: {}", path_string);
        path_string
    }

    #[test]
    fn test_output_new_basic() {
        let krate = IndexKrate::new(TEST_CRATE).unwrap();
        let registry_path = "/tmp/registry";
        let output = SetupTestOutput::new(krate, registry_path);

        assert_eq!(output.registry_path, PathBuf::from("/tmp/registry"));
        assert_eq!(output.total, DiskSize::zero());
        assert!(output.crates.is_empty());
    }

    #[test]
    fn test_output_new_header_format() {
        let krate = IndexKrate::new(TEST_CRATE).unwrap();
        let registry_path = "/test/path";
        let output = SetupTestOutput::new(krate, registry_path);

        assert!(output.header.contains("Local registry"));
        assert!(output.header.starts_with("\n  "));
        assert!(output.header.contains('\n'));
    }

    #[test]
    fn test_output_new_empty_crate_name() {
        let index_path = make_index_path("holochain_serialized_bytes_derive");
        let krate = IndexKrate::new(index_path).unwrap();
        let registry_path = "/some/path";
        let output = SetupTestOutput::new(krate, registry_path);

        assert!(!output.header.is_empty());
        assert_eq!(output.registry_path, PathBuf::from("/some/path"));
    }
}
