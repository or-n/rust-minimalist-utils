use super::DepValue;
use eat::*;
use std::hash::Hash;

pub struct IdValue<Id, Value> {
    id: Id,
    value: Value,
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

impl<'a, Id> Eat<&'a [u8], (), ()> for IdValue<Id, DepValue<Id>>
where
    Id: Eat<&'a [u8], (), ()>,
    DepValue<Id>: Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, id) = Id::eat(i, ())?;
        let (i, value) = DepValue::<Id>::eat(i, ())?;
        Ok((i, IdValue { id, value }))
    }
}

impl<'a, Id> Eat<&'a [u8], (), ()> for super::Table<Id, DepValue<Id>>
where
    Id: Eq + Hash + Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, id_value) = IdValue::<Id, DepValue<Id>>::eat_many(i, ());
        let table = id_value.into_iter().map(|x| (x.id, x.value)).collect();
        Ok((i, super::Table { table }))
    }
}
