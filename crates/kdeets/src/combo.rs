use crate::Error;
use crate::remote::RemoteSparseIndex;
use tame_index::index::LocalRegistry;
use tame_index::{IndexKrate, KrateName, index::FileLock};

/// A wrapper around either a [`RemoteSparseIndex`] or a [`LocalRegistry`]
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
            Self::Local(lr) => local_krate(lr, name, lock),
        }
    }
}

fn local_krate(
    registry: &LocalRegistry,
    name: KrateName<'_>,
    lock: &FileLock,
) -> Result<Option<IndexKrate>, Error> {
    let entry = registry.index().krate(name, lock)?;
    Ok(entry.map(|entry| entry.deserialize()).transpose()?)
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
