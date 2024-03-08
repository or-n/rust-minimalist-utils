pub mod components;
pub mod entities;
pub mod traits;

use components::*;
use entities::*;
use traits::{New, Remove};

pub struct ECS {
    entities: Entities,
    components: Components,
}

impl ECS {
    pub fn new() -> ECS {
        ECS {
            entities: Entities::new(),
            components: Components::new(),
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.components.insert(Vec::<T>::new());
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.new()
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.entities.remove(entity.index)
    }

    pub fn insert_component<T: 'static>(&mut self, entity: Entity, value: T) {
        if let Some(vec) = self.components.get_mut::<T>() {
            let component_index = vec.len();
            vec.push(value);
            self.entities
                .insert_component_index::<T>(entity.index, component_index);
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.entities
            .get_component_index::<T>(entity.index)
            .map(|component_index| &self.components.get::<T>().unwrap()[component_index])
    }

    pub fn get_mut_component<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.entities
            .get_component_index::<T>(entity.index)
            .map(|component_index| &mut self.components.get_mut::<T>().unwrap()[component_index])
    }
}
