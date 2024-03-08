pub trait UniqueId {
    fn unique_id(&mut self) -> usize;
}

pub trait New<Id> {
    fn new(&mut self) -> Id;
}

pub trait Remove<Id, Error> {
    fn remove(&mut self, id: Id) -> Result<(), Error>;
}

pub trait Get<Id, Component> {
    fn get(&self, id: Id) -> Option<&Component>;
}

pub trait GetMut<Id, Component> {
    fn get(&mut self, id: Id) -> Option<&mut Component>;
}

pub trait Size {
    fn len(&self) -> usize;
}
