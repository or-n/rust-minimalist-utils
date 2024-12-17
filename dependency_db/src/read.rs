use super::dep::DepValue;
use eat::*;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Table<Id, Value> {
    pub table: HashMap<Id, Value>,
}

impl<'a, Id> Eat<&'a [u8], (), ()> for Table<Id, DepValue<Id>>
where
    Id: Eq + Hash + Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (i, id_value) = <(Id, DepValue<Id>)>::eat_many(i, ());
        let table = id_value.into_iter().collect();
        Ok((i, Table { table }))
    }
}
