use eat::*;
use std::hash::Hash;

pub struct KeyValue<Hash>((Hash, (Vec<u8>, Vec<Hash>)));

impl<'a, T> Eat<&'a [u8], (), ()> for KeyValue<T>
where
    T: Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, key) = T::eat(i, ())?;
        let (i, file) = SeqN::<u8>::eat(i, ())?;
        let (i, deps) = SeqN::<T>::eat(i, ())?;
        let value = (file.0, deps.0);
        Ok((i, KeyValue((key, value))))
    }
}

impl<'a, T> Eat<&'a [u8], (), ()> for super::Deps<T>
where
    T: Eq + Hash + Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, deps) = KeyValue::<T>::eat_many(i, ());
        let table = deps.into_iter().map(|x| x.0).collect();
        Ok((i, super::Deps { table }))
    }
}
