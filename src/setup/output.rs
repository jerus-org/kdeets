use std::{fmt::Display, fs};

use colorful::Colorful;
use tame_index::{
    index::{
        local::{builder::Client, LocalRegistryBuilder, ValidKrate},
        ComboIndex, FileLock,
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

    pub(crate) fn initialise_local_registry(
        &mut self,
        no_replace: bool,
    ) -> Result<&mut Self, Error> {
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
        Ok(self)
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
        combo_index: &ComboIndex,
    ) -> Result<(), Error> {
        log::debug!("Adding {} dependencies", dependencies.len());
        for dependency in dependencies {
            let dependency_name = KrateName::crates_io(dependency.crate_name())?;
            let lock = FileLock::unlocked();
            let dependency_crate = combo_index.krate(dependency_name, true, &lock)?;
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

    use tame_index::index::{ComboIndex, LocalRegistry};

    use super::*;

    const TEST_CRATE: &str = "tests/registry/index/fo/re/forestry";
    const TEST_CRATE_ROOT: &str = "tests/registry/index/";
    const TEST_CRATE_NAME: &str = "forestry"; // One dependency
    const TEST_CRATE_NO_DEPENDENCY: &str = "some_crate"; // No dependencies

    pub(crate) fn get_test_index() -> Result<ComboIndex, tame_index::error::Error> {
        let local_registry = LocalRegistry::open(PathBuf::from("tests/registry"), true)?;

        Ok(ComboIndex::from(local_registry))
    }

    fn make_index_path(name: &str) -> String {
        let first_two = name[..2].to_string();
        let second_two = name[2..4].to_string();

        let path_string = format!("{}{}/{}/{}", TEST_CRATE_ROOT, first_two, second_two, name);
        println!("Manufactured string is: {}", path_string);
        path_string
    }

    fn get_index_crate(name: &str) -> IndexKrate {
        IndexKrate::new(make_index_path(name)).unwrap()
    }

    fn get_new_output(name: &str) -> SetupTestOutput {
        let index_crate = get_index_crate(name);
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let output = SetupTestOutput::new(index_crate, registry_path.to_str().unwrap());
        output
    }

    fn get_initialised_output(name: &str) -> SetupTestOutput {
        let index_crate = get_index_crate(name);
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let mut output = SetupTestOutput::new(index_crate, registry_path.to_str().unwrap());

        output.initialise_local_registry(false).unwrap();
        output
    }

    #[test]
    fn test_output_new_basic() {
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();
        let registry_path = "/tmp/registry";
        let output = SetupTestOutput::new(index_crate, registry_path);

        assert_eq!(output.registry_path, PathBuf::from("/tmp/registry"));
        assert_eq!(output.total, DiskSize::zero());
        assert!(output.crates.is_empty());
    }

    #[test]
    fn test_output_new_header_format() {
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();
        let registry_path = "/test/path";
        let output = SetupTestOutput::new(index_crate, registry_path);

        assert!(output.header.contains("Local registry"));
        assert!(output.header.starts_with("\n  "));
        assert!(output.header.contains('\n'));
    }

    #[test]
    fn test_output_new_empty_crate_name() {
        let index_path = make_index_path("holochain_serialized_bytes_derive");
        let index_crate = IndexKrate::new(index_path).unwrap();
        let registry_path = "/some/path";
        let output = SetupTestOutput::new(index_crate, registry_path);

        assert!(!output.header.is_empty());
        assert_eq!(output.registry_path, PathBuf::from("/some/path"));
    }

    #[test]
    fn test_initialise_local_registry_success() {
        let index_crate = get_index_crate("forestry");
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");
        let mut output = SetupTestOutput::new(index_crate, registry_path.to_str().unwrap());

        assert!(output.initialise_local_registry(false).is_ok());
        assert!(registry_path.exists());
    }

    #[test]
    fn test_initialise_local_registry_existing_no_replace() {
        let index_crate = get_index_crate("forestry");
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");
        fs::create_dir_all(&registry_path).unwrap();

        // Create a file to make it a valid registry
        let config_file = registry_path.join("config.json");
        fs::write(config_file, "{\"version\": 1}").unwrap();

        println!("Registry path: {}", registry_path.to_str().unwrap());

        let mut output = SetupTestOutput::new(index_crate, registry_path.to_str().unwrap());
        println!("Output registry path: {}", output.registry_path);
        let result = output.initialise_local_registry(true);
        assert!(result.is_err());
        matches!(result, Err(Error::TameIndex(_)));
    }

    #[test]
    fn test_initialise_local_registry_existing_with_replace() {
        let index_crate = get_index_crate("forestry");
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");
        fs::create_dir_all(&registry_path).unwrap();

        // Create a file to make it a valid registry
        let config_file = registry_path.join("config.json");
        fs::write(config_file, "{\"version\": 1}").unwrap();

        println!("Registry path: {}", registry_path.to_str().unwrap());

        let mut output = SetupTestOutput::new(index_crate, registry_path.to_str().unwrap());
        println!("Output registry path: {}", output.registry_path);
        assert!(output.initialise_local_registry(false).is_ok());
        assert!(registry_path.exists());
    }

    #[test]
    fn test_initialise_local_registry_permission_error() {
        let index_crate = get_index_crate("forestry");
        let registry_path = "/root/test_registry";
        let mut output = SetupTestOutput::new(index_crate, registry_path);

        let result = output.initialise_local_registry(false);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_crate_success() {
        let mut output = get_initialised_output(TEST_CRATE_NAME);
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();

        assert!(output.insert_crate(&index_crate).is_ok());
        assert_eq!(output.crates.len(), 1);
        assert_eq!(output.crates[0], "forestry".to_string());
    }

    #[test]
    fn test_insert_crate_registry_not_set() {
        let mut output = get_new_output(TEST_CRATE_NAME);
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();

        let result = output.insert_crate(&index_crate);
        assert!(matches!(result, Err(Error::LocalRegistryBuilderNotSet)));
    }

    #[test]
    fn test_add_dependency_crates_empty_dependencies() {
        let mut output = get_new_output(TEST_CRATE_NO_DEPENDENCY);
        output.initialise_local_registry(false).unwrap();
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();
        let combo_index = get_test_index().unwrap();
        let result = output.add_dependency_crates(
            index_crate.most_recent_version().dependencies(),
            &combo_index,
        );
        println!("Result: {:?}", result);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_dependency_crates_valid_dependencies() {
        let mut output = get_new_output(TEST_CRATE_NAME);
        output.initialise_local_registry(false).unwrap();
        let index_crate = IndexKrate::new(TEST_CRATE).unwrap();
        let combo_index = get_test_index().unwrap();
        let result = output.add_dependency_crates(
            index_crate.most_recent_version().dependencies(),
            &combo_index,
        );
        println!("Result: {:?}", result);

        assert!(result.is_ok());
    }
}