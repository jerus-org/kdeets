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
    let index = get_sparce_index()?;
    let builder = get_client_builder();
    let client = builder.build()?;

    let remote_index = RemoteSparseIndex::new(index, client);

    Ok(ComboIndex::from(remote_index))
}

pub(crate) fn get_sparce_index() -> Result<SparseIndex, tame_index::error::Error> {
    let il = IndexLocation::new(IndexUrl::CratesIoSparse);
    SparseIndex::new(il)
}

pub(crate) fn get_client_builder() -> ClientBuilder {
    let builder = ClientBuilder::new();
    builder.tls_built_in_root_certs(true)
}

#[cfg(test)]
mod tests {

    use crate::get_remote_combo_index;
    use tame_index::{
        index::{ComboIndex, LocalRegistry},
        PathBuf,
    };

    const TEST_REGISGTRY: &str = "tests/registry";

    pub(crate) fn get_test_index() -> Result<ComboIndex, tame_index::error::Error> {
        let temp_dir = tempfile::tempdir().unwrap();
        let registry_path = temp_dir.path().join("registry");

        copy_dir::copy_dir(TEST_REGISGTRY, &registry_path)?;

        let local_registry =
            LocalRegistry::open(PathBuf::from(registry_path.to_str().unwrap()), false)?;

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
