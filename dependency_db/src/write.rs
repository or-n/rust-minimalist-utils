use super::Table;
// use to_seq::*;

pub trait ToSeq<Output, Data, Error> {
    fn to_seq(self, o: Output, data: Data) -> Result<Output, Error>;
}

impl ToSeq<&mut Vec<u8>, (), ()> for &[u8] {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<&mut Vec<u8>, ()> {
        o.extend_from_slice(self);
        Ok(o)
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for u32 {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<&mut Vec<u8>, ()> {
        self.to_be_bytes().to_seq(o, ())?;
        Ok(o)
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for &Vec<u8> {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<&mut Vec<u8>, ()> {
        o.extend(self);
        Ok(o)
    }
}

impl<'a, Id> ToSeq<&'a mut Vec<u8>, (), ()> for super::DepValue<Id>
where
    Id: ToSeq<&'a mut Vec<u8>, (), ()>,
{
    fn to_seq(self, mut o: &'a mut Vec<u8>, _data: ()) -> Result<&'a mut Vec<u8>, ()> {
        o = (self.bytes.len() as u32).to_seq(o, ())?;
        o = self.bytes.to_seq(o, ())?;
        o = (self.deps.len() as u32).to_seq(o, ())?;
        for dep in self.deps {
            o = dep.to_seq(o, ())?;
        }
        Ok(o)
    }
}

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
