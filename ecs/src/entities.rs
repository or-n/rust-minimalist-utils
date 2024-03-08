use std::any::TypeId;
use std::collections::HashMap;

use crate::traits::*;

#[derive(Clone, Copy)]
pub struct Entity {
    pub index: usize,
    unique_id: usize,
}

pub struct Entities {
    component_indices: Vec<HashMap<TypeId, usize>>,
    unique_id: Vec<usize>,
    next_id: usize,
    free: Vec<usize>,
}

impl UniqueId for Entities {
    fn unique_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Size for Entities {
    fn len(&self) -> usize {
        self.unique_id.len()
    }
}

impl New<Entity> for Entities {
    fn new(&mut self) -> Entity {
        let unique_id = self.unique_id();
        let index = match self.free.pop() {
            Some(index) => {
                self.component_indices[index].clear();
                self.unique_id[index] = unique_id;
                index
            }
            _ => {
                let index = self.len();
                self.component_indices.push(HashMap::new());
                self.unique_id.push(unique_id);
                index
            }
        };
        Entity { index, unique_id }
    }
}

impl Remove<usize, ()> for Entities {
    fn remove(&mut self, id: usize) -> Result<(), ()> {
        self.remove(id);
        self.free.push(id);
        Ok(())
    }
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            component_indices: Vec::new(),
            unique_id: Vec::new(),
            next_id: 0,
            free: Vec::new(),
        }
    }

    pub fn insert_component_index<T: 'static>(&mut self, entity: usize, component_index: usize) {
        self.component_indices[entity].insert(TypeId::of::<T>(), component_index);
    }

    pub fn get_component_index<T: 'static>(&self, entity: usize) -> Option<usize> {
        self.component_indices[entity]
            .get(&TypeId::of::<T>())
            .copied()
    }

    pub fn remove(&mut self, entity: usize) {
        if self.len() - 1 == entity {
            self.component_indices.pop();
            self.unique_id.pop();
        } else {
            self.component_indices[entity] = self.component_indices.pop().unwrap();
            self.unique_id[entity] = self.unique_id.pop().unwrap();
        }
    }
}
