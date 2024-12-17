pub mod dep;
pub mod read;
pub mod write;

use std::collections::HashMap;

pub struct Table<Id, Value> {
    pub table: HashMap<Id, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(deps.table.get_key_value(&0), Some((&0, &value)));
    }
}
