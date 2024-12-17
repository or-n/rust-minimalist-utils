use super::Table;
use to_seq::*;

impl<'a, Id, Value> ToSeq<&'a mut Vec<u8>, ()> for Table<Id, Value>
where
    Id: ToSeq<&'a mut Vec<u8>, ()>,
    Value: ToSeq<&'a mut Vec<u8>, ()>,
{
    fn to_seq(self, mut o: &'a mut Vec<u8>) -> Result<&mut Vec<u8>, ()> {
        for (id, value) in self.table {
            o = id.to_seq(o)?;
            o = value.to_seq(o)?;
        }
        Ok(o)
    }
}
