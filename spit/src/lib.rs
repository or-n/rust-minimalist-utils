use std::collections::HashMap;

pub trait Spit<Output, Error> {
    fn spit(self, o: Output) -> Result<Output, Error>;
}

pub trait SpitMany<Output, Error>
where
    Self: Sized,
{
    fn spit(x: &[Self], o: Output) -> Result<Output, Error>;
}

impl<T> SpitMany<Vec<u8>, ()> for T
where
    T: Clone + Spit<Vec<u8>, ()>,
{
    fn spit(x: &[Self], mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        for item in x.iter() {
            o = item.clone().spit(o)?;
        }
        Ok(o)
    }
}

impl Spit<Vec<u8>, ()> for &[u8] {
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend_from_slice(self);
        Ok(o)
    }
}

impl Spit<Vec<u8>, ()> for u32 {
    fn spit(self, o: Vec<u8>) -> Result<Vec<u8>, ()> {
        self.to_be_bytes().spit(o)
    }
}

impl Spit<Vec<u8>, ()> for &Vec<u8> {
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend(self);
        Ok(o)
    }
}

impl<Id, Value> Spit<Vec<u8>, ()> for HashMap<Id, Value>
where
    Id: Spit<Vec<u8>, ()>,
    Value: Spit<Vec<u8>, ()>,
{
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        for (id, value) in self {
            o = id.spit(o)?;
            o = value.spit(o)?;
        }
        Ok(o)
    }
}
