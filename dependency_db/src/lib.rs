pub mod read;
pub mod util;
pub mod write;

use std::collections::HashMap;

pub const DEPENDENCIES: &str = "dependencies";

pub struct Deps<Hash> {
    pub table: HashMap<Hash, (Vec<u8>, Vec<Hash>)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use eat::*;
    use std::fs;

    #[test]
    fn test_1_dep() {
        let bytes = fs::read(DEPENDENCIES).unwrap();
        let (i, deps) = Deps::<u32>::eat(&bytes[..], ()).unwrap();
        assert!(i.is_empty());
        assert!(deps.table.len() == 1);
        let value = (vec![], vec![]);
        assert_eq!(deps.table.get_key_value(&0), Some((&0, &value)));
    }
}
