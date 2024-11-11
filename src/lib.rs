const HEADER: &str = "Crate versions for";
const LINE_CHAR: char = '🭶';

mod crate_versions;
mod error;
mod rust_versions;

pub use crate_versions::CrateVersions;
pub use error::Error;
pub use rust_versions::RustVersions;
use tame_index::external::reqwest::blocking::ClientBuilder;
use tame_index::index::{ComboIndex, RemoteSparseIndex};

use tame_index::{IndexLocation, IndexUrl, SparseIndex};

pub(crate) fn get_sparce_index() -> Result<ComboIndex, tame_index::error::Error> {
    let il = IndexLocation::new(IndexUrl::CratesIoSparse);
    let index = SparseIndex::new(il)?;

    let builder = ClientBuilder::new();
    let builder = builder.tls_built_in_root_certs(true);
    let client = builder.build()?;

    let remote_index = RemoteSparseIndex::new(index, client);

    Ok(ComboIndex::from(remote_index))
}

#[cfg(test)]
mod tests {

    use crate::get_sparce_index;
    use tame_index::index::ComboIndex;

    #[test]
    fn test_get_sparse_index_success() {
        let result = get_sparce_index();
        assert!(result.is_ok());
        let index = result.unwrap();
        assert!(matches!(index, ComboIndex::Sparse(_)));
    }

    #[test]
    fn test_get_sparse_index_type() {
        let result = get_sparce_index();
        assert!(matches!(result, Ok(ComboIndex::Sparse(_))));
    }

    #[test]
    fn test_sparse_index_error_handling() {
        let result = get_sparce_index();
        match result {
            Ok(_) => (),
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }
}
