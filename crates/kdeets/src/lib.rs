const HEADER: &str = "Crate versions for";
const SETUP_HEADER: &str = "Local registry set up for";
const LINE_CHAR: char = '🭶';

mod combo;
mod crate_versions;
mod error;
mod rust_versions;
mod setup;

pub use crate_versions::CrateVersions;
pub use error::Error;
pub use rust_versions::RustVersions;
pub use setup::Setup;

pub(crate) use combo::ComboIndex;

use reqwest::blocking::ClientBuilder;
use tame_index::index::RemoteSparseIndex;
use tame_index::{IndexLocation, IndexUrl, SparseIndex};

/// Returns `true` if the given version of a crate exists in the crates.io index,
/// or `false` if the crate is found but the version is absent.
///
/// Returns an error if the crate is not found on the index or if the index
/// cannot be queried.
///
/// # Errors
///
/// Returns [`Error::CrateNotFoundOnIndex`] when the crate is absent from the index.
/// Returns other [`Error`] variants on index access failures.
///
/// # Examples
///
/// ```no_run
/// use kdeets_lib::version_exists;
///
/// # fn main() -> Result<(), kdeets_lib::Error> {
/// let exists = version_exists("serde", "1.0.0")?;
/// assert!(exists);
/// # Ok(())
/// # }
/// ```
pub fn version_exists(crate_name: &str, version: &str) -> Result<bool, Error> {
    let index = get_remote_combo_index()?;
    version_exists_in_index(&index, crate_name, version)
}

/// Returns all published version strings for a crate from the crates.io index.
///
/// # Errors
///
/// Returns [`Error::CrateNotFoundOnIndex`] when the crate is absent from the index.
/// Returns other [`Error`] variants on index access failures.
///
/// # Examples
///
/// ```no_run
/// use kdeets_lib::list_versions;
///
/// # fn main() -> Result<(), kdeets_lib::Error> {
/// let versions = list_versions("serde")?;
/// assert!(versions.contains(&"1.0.0".to_string()));
/// # Ok(())
/// # }
/// ```
pub fn list_versions(crate_name: &str) -> Result<Vec<String>, Error> {
    let index = get_remote_combo_index()?;
    list_versions_in_index(&index, crate_name)
}

pub(crate) fn version_exists_in_index(
    index: &ComboIndex,
    crate_name: &str,
    version: &str,
) -> Result<bool, Error> {
    use tame_index::{KrateName, index::FileLock};

    let lock = FileLock::unlocked();
    let index_krate = index.krate(KrateName::crates_io(crate_name)?, true, &lock)?;

    let Some(index_krate) = index_krate else {
        return Err(Error::CrateNotFoundOnIndex);
    };

    Ok(index_krate
        .versions
        .iter()
        .any(|v| v.version.as_str() == version))
}

pub(crate) fn list_versions_in_index(
    index: &ComboIndex,
    crate_name: &str,
) -> Result<Vec<String>, Error> {
    use tame_index::{KrateName, index::FileLock};

    let lock = FileLock::unlocked();
    let index_krate = index.krate(KrateName::crates_io(crate_name)?, true, &lock)?;

    let Some(index_krate) = index_krate else {
        return Err(Error::CrateNotFoundOnIndex);
    };

    Ok(index_krate
        .versions
        .iter()
        .map(|v| v.version.to_string())
        .collect())
}

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
    // Create a certificate store using webpki_roots, which packages
    let rcs: rustls::RootCertStore = webpki_roots::TLS_SERVER_ROOTS.iter().cloned().collect();
    let client_config = rustls::ClientConfig::builder_with_provider(std::sync::Arc::new(
        // Use `ring` as the crypto provider
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(rustls::DEFAULT_VERSIONS)
    .unwrap()
    .with_root_certificates(rcs)
    .with_no_client_auth();

    reqwest::blocking::Client::builder()
        // Set the TLS backend. Note that this *requires* that the version of
        // rustls is the same as the one reqwest is using
        .tls_backend_preconfigured(client_config)
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::ComboIndex;
    use crate::get_remote_combo_index;
    use tame_index::{PathBuf, index::LocalRegistry};
    use tempfile::TempDir;

    const TEST_REGISTRY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/registry");

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
            Err(e) => panic!("Expected Ok, got Err: {e:?}"),
        }
    }

    // Network tests for the public API — exercise the full call chain:
    // version_exists / list_versions → get_remote_combo_index → _in_index helper

    #[test]
    fn test_version_exists_real_crate_known_version() {
        let result = crate::version_exists("serde", "1.0.0");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        assert!(result.unwrap(), "Expected serde 1.0.0 to exist on crates.io");
    }

    #[test]
    fn test_version_exists_real_crate_nonexistent_version() {
        let result = crate::version_exists("serde", "99.99.99");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        assert!(!result.unwrap(), "Expected serde 99.99.99 to not exist");
    }

    #[test]
    fn test_list_versions_real_crate() {
        let result = crate::list_versions("serde");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        let versions = result.unwrap();
        assert!(
            versions.contains(&"1.0.0".to_string()),
            "Expected serde versions to contain 1.0.0"
        );
    }

    // Local-registry tests for the internal helpers

    #[test]
    fn test_version_exists_known_version_returns_true() {
        let (_temp_dir, registry) = get_temp_local_registry();
        let index = get_test_index(&registry).unwrap();
        // some_crate 0.2.1 is present in the local test registry
        let result = crate::version_exists_in_index(&index, "some_crate", "0.2.1");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        assert!(result.unwrap(), "Expected version 0.2.1 to exist");
    }

    #[test]
    fn test_version_exists_unknown_version_returns_false() {
        let (_temp_dir, registry) = get_temp_local_registry();
        let index = get_test_index(&registry).unwrap();
        // 99.99.99 does not exist for some_crate
        let result = crate::version_exists_in_index(&index, "some_crate", "99.99.99");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        assert!(!result.unwrap(), "Expected version 99.99.99 to not exist");
    }

    #[test]
    fn test_list_versions_contains_known_version() {
        let (_temp_dir, registry) = get_temp_local_registry();
        let index = get_test_index(&registry).unwrap();
        // some_crate 0.2.1 is present in the local test registry
        let result = crate::list_versions_in_index(&index, "some_crate");
        assert!(result.is_ok(), "Expected Ok, got {result:?}");
        let versions = result.unwrap();
        assert!(
            versions.contains(&"0.2.1".to_string()),
            "Expected versions to contain 0.2.1, got {versions:?}"
        );
    }

    #[test]
    fn test_version_exists_nonexistent_crate_returns_error() {
        let (_temp_dir, registry) = get_temp_local_registry();
        let index = get_test_index(&registry).unwrap();
        // nonexistent-crate-xyz is not in the local test registry
        let result = crate::version_exists_in_index(&index, "nonexistent-crate-xyz", "1.0.0");
        assert!(
            result.is_err(),
            "Expected Err for nonexistent crate, got {result:?}"
        );
    }
}
