use tame_index::index::local::LocalRegistryBuilder;

pub(crate) enum OutputRegistry {
    None,
    Builder(LocalRegistryBuilder),
    Registry,
}

#[cfg(test)]
mod tests {
    use tame_index::PathBuf;

    use super::*;

    #[test]
    fn test_output_registry_none() {
        let registry = OutputRegistry::None;
        matches!(registry, OutputRegistry::None);
    }

    #[test]
    fn test_output_registry_builder() {
        let builder = LocalRegistryBuilder::create(PathBuf::from("test")).unwrap();
        let registry = OutputRegistry::Builder(builder);
        matches!(registry, OutputRegistry::Builder(_));
    }

    #[test]
    fn test_output_registry_registry() {
        let registry = OutputRegistry::Registry;
        matches!(registry, OutputRegistry::Registry);
    }
}
