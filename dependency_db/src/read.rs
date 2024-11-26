use crate::util::{SeqN, U32};
use eat::*;

pub struct KeyValue<Hash>((Hash, (Vec<u8>, Vec<Hash>)));

impl Eat<&[u8], (), ()> for KeyValue<u32> {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, key) = U32::eat(i, ())?;
        let (i, file) = SeqN::<u8>::eat(i, ())?;
        let (i, deps) = SeqN::<U32>::eat(i, ())?;
        let deps = deps.0.into_iter().map(|x| x.0).collect();
        let value = (file.0, deps);
        Ok((i, KeyValue((key.0, value))))
    }
}

impl Eat<&[u8], (), ()> for super::Deps<u32> {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, deps) = KeyValue::<u32>::eat_many(i, ());
        let table = deps.into_iter().map(|x| x.0).collect();
        Ok((i, super::Deps { table }))
    }
}
