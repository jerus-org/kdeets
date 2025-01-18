const HEADER: &str = "Crate versions for";
const SETUP_HEADER: &str = "Local registry set up for";
const LINE_CHAR: char = '🭶';

mod crate_versions;
mod error;
mod rust_versions;
mod setup;

pub use crate_versions::CrateVersions;
pub use error::Error;
pub use rust_versions::RustVersions;
pub use setup::Setup;

use tame_index::external::reqwest::blocking::ClientBuilder;
use tame_index::index::{ComboIndex, RemoteSparseIndex};
use tame_index::{IndexLocation, IndexUrl, SparseIndex};

pub(crate) fn get_remote_combo_index() -> Result<ComboIndex, tame_index::error::Error> {
    let index = get_sparse_index()?;
    let builder = get_client_builder();
    let client = builder.build()?;

    let remote_index = RemoteSparseIndex::new(index, client);

    Ok(ComboIndex::from(remote_index))
}

pub(crate) fn get_sparse_index() -> Result<SparseIndex, tame_index::error::Error> {
    let il = IndexLocation::new(IndexUrl::CratesIoSparse);
    SparseIndex::new(il)
}

pub(crate) fn get_client_builder() -> ClientBuilder {
    let builder = ClientBuilder::new();
    builder.tls_built_in_root_certs(true)
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::get_remote_combo_index;
    use tame_index::{
        index::{ComboIndex, LocalRegistry},
        PathBuf,
    };
    use tempfile::TempDir;

    const TEST_REGISTRY: &str = "tests/registry";

    pub(crate) fn get_temp_local_registry() -> (TempDir, String) {
        let temp_dir = tempfile::tempdir().unwrap();
        println!("Temp dir: {}", temp_dir.path().display());
        let registry_path = temp_dir.path().join("registry");
        let registry = registry_path.to_str().unwrap();

        let options = fs_extra::dir::CopyOptions::new();

        let from_path = vec![TEST_REGISTRY];

        let _ = fs_extra::copy_items(&from_path, temp_dir.path().to_str().unwrap(), &options);
        let _ = fs_extra::copy_items(&from_path, "/tmp/test/", &options);

        (temp_dir, registry.to_string())
    }

    pub(crate) fn get_test_index(registry: &str) -> Result<ComboIndex, tame_index::error::Error> {
        let local_registry = LocalRegistry::open(PathBuf::from(registry), false)?;

        Ok(ComboIndex::from(local_registry))
    }

    #[test]
    fn test_get_sparse_index_success() {
        let result = get_remote_combo_index();
        assert!(result.is_ok());
        let index = result.unwrap();
        assert!(matches!(index, ComboIndex::Sparse(_)));
    }

    #[test]
    fn test_get_sparse_index_type() {
        let result = get_remote_combo_index();
        assert!(matches!(result, Ok(ComboIndex::Sparse(_))));
    }

    #[test]
    fn test_sparse_index_error_handling() {
        let result = get_remote_combo_index();
        match result {
            Ok(_) => (),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
}
