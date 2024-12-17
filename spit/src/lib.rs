use std::collections::HashMap;

pub trait Spit<Output, Error> {
    fn spit(self, o: Output) -> Result<Output, Error>;
}

pub trait SpitMany<Output, Error>
where
    Self: Sized,
{
    fn spit_many<I>(x: I, o: Output) -> Result<Output, Error>
    where
        I: Iterator<Item = Self>;
}

impl<T> SpitMany<Vec<u8>, T> for T
where
    T: Clone + Spit<Vec<u8>, ()>,
{
    fn spit_many<I>(x: I, mut o: Vec<u8>) -> Result<Vec<u8>, T>
    where
        I: Iterator<Item = Self>,
    {
        for item in x {
            o = item.clone().spit(o).map_err(|_| item.clone())?;
        }
        Ok(o)
    }
}

impl Spit<Vec<u8>, ()> for u8 {
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.push(self);
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

impl<A, B> Spit<Vec<u8>, ()> for (A, B)
where
    A: Spit<Vec<u8>, ()>,
    B: Spit<Vec<u8>, ()>,
{
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o = self.0.spit(o)?;
        o = self.1.spit(o)?;
        Ok(o)
    }
}
