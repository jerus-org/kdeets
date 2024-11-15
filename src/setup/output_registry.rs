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
        match registry {
            OutputRegistry::None => (),
            _ => panic!("Expected OutputRegistry::None variant"),
        }
    }

    #[test]
    fn test_output_registry_builder() {
        let builder = LocalRegistryBuilder::create(PathBuf::from("test")).unwrap();
        let registry = OutputRegistry::Builder(builder);
        match registry {
            OutputRegistry::Builder(_) => (),
            _ => panic!("Expected OutputRegistry::Builder variant"),
        }
    }

    #[test]
    fn test_output_registry_registry() {
        let registry = OutputRegistry::Registry;
        match registry {
            OutputRegistry::Registry => (),
            _ => panic!("Expected OutputRegistry::Registry variant"),
        }
    }
}
