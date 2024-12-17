use eat::*;
use spit::*;

pub const DEPENDENCIES: &str = "dependencies";

#[derive(Debug, PartialEq)]
pub struct DepValue<Id> {
    pub bytes: Vec<u8>,
    pub deps: Vec<Id>,
}

impl<'a, Id> Eat<&'a [u8], (), ()> for DepValue<Id>
where
    Id: Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, len_bytes) = u32::eat(i, ())?;
        let (i, bytes) = u8::eat_len(i, (), len_bytes as usize)?;
        let (i, len_deps) = u32::eat(i, ())?;
        let (i, deps) = Id::eat_len(i, (), len_deps as usize)?;
        Ok((i, DepValue { bytes, deps }))
    }
}

impl<Id> Spit<Vec<u8>, ()> for DepValue<Id>
where
    Id: Spit<Vec<u8>, ()>,
{
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o = (self.bytes.len() as u32).spit(o)?;
        o = self.bytes.spit(o)?;
        o = (self.deps.len() as u32).spit(o)?;
        for dep in self.deps {
            o = dep.spit(o)?;
        }
        Ok(o)
    }
}
