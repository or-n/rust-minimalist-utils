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
        let (i, bytes) = SeqN::<u8>::eat(i, ())?;
        let (i, deps) = SeqN::<Id>::eat(i, ())?;
        let r = DepValue {
            bytes: bytes.0,
            deps: deps.0,
        };
        Ok((i, r))
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
