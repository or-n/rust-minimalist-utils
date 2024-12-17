pub mod dep;
pub mod read;

#[cfg(test)]
mod tests {
    use super::dep::{DepValue, DEPENDENCIES};
    use super::read::Table;
    use eat::*;
    use std::fs;

    #[test]
    fn test_1_dep() {
        let bytes = fs::read(DEPENDENCIES).unwrap();
        let (i, deps) = Table::<u32, DepValue<u32>>::eat(&bytes[..], ()).unwrap();
        assert!(i.is_empty());
        assert!(deps.table.len() == 1);
        let value = DepValue {
            bytes: vec![],
            deps: vec![],
        };
        assert_eq!(deps.table.get_key_value(&2137), Some((&2137, &value)));
    }
}
