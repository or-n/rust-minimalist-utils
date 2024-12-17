pub mod dep;

#[cfg(test)]
mod tests {
    use super::dep::{DepValue, DEPENDENCIES};
    use eat::*;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_1_dep() {
        let bytes = fs::read(DEPENDENCIES).unwrap();
        let (i, id_value) = <(u32, DepValue<u32>)>::eat_many(&bytes[..], ());
        let deps: HashMap<_, _> = id_value.into_iter().collect();
        assert!(i.is_empty());
        assert!(deps.len() == 1);
        let value = DepValue {
            bytes: vec![],
            deps: vec![],
        };
        assert_eq!(deps.get_key_value(&2137), Some((&2137, &value)));
    }
}
