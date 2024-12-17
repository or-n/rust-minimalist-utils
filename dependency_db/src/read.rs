use super::dep::DepValue;
use eat::*;
use std::hash::Hash;

pub struct IdValue<Id, Value> {
    id: Id,
    value: Value,
}

impl<'a, Id> Eat<&'a [u8], (), ()> for super::Table<Id, DepValue<Id>>
where
    Id: Eq + Hash + Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, id_value) = <(Id, DepValue<Id>)>::eat_many(i, ());
        let table = id_value.into_iter().collect();
        Ok((i, super::Table { table }))
    }
}
