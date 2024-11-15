use tame_index::index::local::LocalRegistryBuilder;

pub(crate) enum OutputRegistry {
    None,
    Builder(LocalRegistryBuilder),
    Registry,
}
