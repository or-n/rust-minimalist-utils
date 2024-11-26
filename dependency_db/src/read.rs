use eat::*;

pub struct KeyValue<Hash>((Hash, (Vec<u8>, Vec<Hash>)));

impl Eat<&[u8], (), ()> for KeyValue<u32> {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, key) = u32::eat(i, ())?;
        let (i, file) = SeqN::<u8>::eat(i, ())?;
        let (i, deps) = SeqN::<u32>::eat(i, ())?;
        let value = (file.0, deps.0);
        Ok((i, KeyValue((key, value))))
    }
}

impl Eat<&[u8], (), ()> for super::Deps<u32> {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, deps) = KeyValue::<u32>::eat_many(i, ());
        let table = deps.into_iter().map(|x| x.0).collect();
        Ok((i, super::Deps { table }))
    }
}
