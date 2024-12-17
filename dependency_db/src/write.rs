use super::Table;
use to_seq::*;

impl<'a, Value> ToSeq<&'a mut Vec<u8>, (), ()> for Table<u32, Value>
where
    Value: ToSeq<&'a mut Vec<u8>, (), ()>,
{
    fn to_seq(self, mut o: &'a mut Vec<u8>, _data: ()) -> Result<&mut Vec<u8>, ()> {
        for (key, value) in self.table {
            o = key.to_seq(o, ())?;
            o = value.to_seq(o, ())?;
        }
        Ok(o)
    }
}
