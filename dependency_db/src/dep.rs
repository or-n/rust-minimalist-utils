use eat::*;
use to_seq::*;

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

impl<'a, Id> ToSeq<&'a mut Vec<u8>, ()> for DepValue<Id>
where
    Id: ToSeq<&'a mut Vec<u8>, ()>,
{
    fn to_seq(self, mut o: &'a mut Vec<u8>) -> Result<&'a mut Vec<u8>, ()> {
        o = (self.bytes.len() as u32).to_seq(o)?;
        o = self.bytes.to_seq(o)?;
        o = (self.deps.len() as u32).to_seq(o)?;
        for dep in self.deps {
            o = dep.to_seq(o)?;
        }
        Ok(o)
    }
}
