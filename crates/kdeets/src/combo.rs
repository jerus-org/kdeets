use tame_index::index::LocalRegistry;
use tame_index::{
    Error, IndexKrate, KrateName,
    index::{FileLock, RemoteSparseIndex},
};

/// A wrapper around either a [`RemoteGitIndex`] or [`RemoteSparseIndex`]
#[non_exhaustive]
pub enum ComboIndex {
    /// An HTTP sparse index
    Sparse(RemoteSparseIndex),
    /// A local registry
    Local(LocalRegistry),
}

impl ComboIndex {
    /// Retrieves the index metadata for the specified crate name, optionally
    /// writing a cache entry for it if there was not already an up to date one
    ///
    /// Note no cache entry is written if this is a `Local` registry as they do
    /// not use .cache files
    #[inline]
    pub fn krate(
        &self,
        name: KrateName<'_>,
        write_cache_entry: bool,
        lock: &FileLock,
    ) -> Result<Option<IndexKrate>, Error> {
        match self {
            Self::Sparse(index) => index.krate(name, write_cache_entry, lock),
            Self::Local(lr) => lr.cached_krate(name, lock),
        }
    }

    /// Retrieves the cached crate metadata if it exists
    #[inline]
    #[allow(dead_code)]
    pub fn cached_krate(
        &self,
        name: KrateName<'_>,
        lock: &FileLock,
    ) -> Result<Option<IndexKrate>, Error> {
        match self {
            Self::Sparse(index) => index.cached_krate(name, lock),
            Self::Local(lr) => lr.cached_krate(name, lock),
        }
    }
}

impl From<RemoteSparseIndex> for ComboIndex {
    #[inline]
    fn from(index: RemoteSparseIndex) -> Self {
        Self::Sparse(index)
    }
}

impl From<LocalRegistry> for ComboIndex {
    #[inline]
    fn from(local: LocalRegistry) -> Self {
        Self::Local(local)
    }
}
