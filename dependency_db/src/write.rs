use super::Table;
use to_seq::*;

impl<Id, Value> ToSeq<Vec<u8>, ()> for Table<Id, Value>
where
    Id: ToSeq<Vec<u8>, ()>,
    Value: ToSeq<Vec<u8>, ()>,
{
    fn to_seq(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        for (id, value) in self.table {
            o = id.to_seq(o)?;
            o = value.to_seq(o)?;
        }
        Ok(o)
    }
}
