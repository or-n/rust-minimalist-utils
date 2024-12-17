use super::DepValue;
use spit::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;

pub struct DB<Id> {
    db: HashMap<Id, DepValue<Id>>,
    top: HashSet<Id>,
    db_path: PathBuf,
    top_path: PathBuf,
}

pub enum Push<Id> {
    Found,
    DepNotFound(Id),
    Spit(Id),
    SpitCommand(TopCommand<Id>),
    IO(std::io::Error),
}

#[derive(Clone)]
pub enum TopCommand<Id> {
    Insert(Id),
    Remove(Id),
}

impl<Id> Spit<Vec<u8>, ()> for TopCommand<Id>
where
    Id: Spit<Vec<u8>, ()>,
{
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        match self {
            TopCommand::Insert(id) => {
                o = ['+' as u8].spit(o)?;
                id.spit(o)
            }
            TopCommand::Remove(id) => {
                o = ['-' as u8].spit(o)?;
                id.spit(o)
            }
        }
    }
}

impl<Id> DB<Id>
where
    Id: Copy + Debug + Eq + Hash + From<DepValue<Id>> + Spit<Vec<u8>, ()>,
{
    pub fn push(&mut self, value: DepValue<Id>) -> Result<(), Push<Id>> {
        let id = Id::from(value.clone());
        if self.db.contains_key(&id) {
            return Err(Push::Found);
        }
        if let Some(dep) = value.deps.iter().find(|dep| !self.db.contains_key(dep)) {
            return Err(Push::DepNotFound(*dep));
        }
        let top_deps = value.deps.iter().filter(|dep| self.top.contains(dep));
        let top_deps: Vec<_> = top_deps.map(|x| *x).collect();
        {
            let commands = top_deps.iter().map(|x| ('-' as u8, *x));
            let bytes = <(u8, Id)>::spit_many(commands, Vec::new()).map_err(|_| Push::Spit(id))?;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.top_path)
                .map_err(Push::IO)?;
            file.write_all(&bytes).map_err(Push::IO)?;
        }
        for dep in top_deps.iter() {
            self.top.remove(dep);
        }
        {
            let bytes = value.clone().spit(Vec::new()).map_err(|_| Push::Spit(id))?;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.db_path)
                .map_err(Push::IO)?;
            file.write_all(&bytes).map_err(Push::IO)?;
        }
        self.db.insert(id, value);
        {
            let bytes = TopCommand::Insert(id)
                .spit(Vec::new())
                .map_err(|_| Push::Spit(id))?;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.top_path)
                .map_err(Push::IO)?;
            file.write_all(&bytes).map_err(Push::IO)?;
        }
        self.top.insert(id);
        Ok(())
    }

    pub fn migrate(self: &mut DB<Id>, other: &DB<Id>, id: Id) -> Result<usize, ()> {
        let value = other.db.get(&id).ok_or(())?;
        if self.db.contains_key(&id) {
            return Ok(0);
        }
        let mut migrated = 0;
        for dep in value.deps.iter() {
            migrated += self.migrate(other, *dep)?;
        }
        self.push(value.clone()).map_err(|_| ())?;
        Ok(migrated + 1)
    }
}
