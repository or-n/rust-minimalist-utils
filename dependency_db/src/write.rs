use super::Table;
use spit::*;

impl<Id, Value> Spit<Vec<u8>, ()> for Table<Id, Value>
where
    Id: Spit<Vec<u8>, ()>,
    Value: Spit<Vec<u8>, ()>,
{
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        for (id, value) in self.table {
            o = id.spit(o)?;
            o = value.spit(o)?;
        }
        Ok(o)
    }
}
