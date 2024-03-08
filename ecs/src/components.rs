use std::any::TypeId;
use std::collections::HashMap;

pub struct Components {
    components: HashMap<TypeId, Box<dyn AnyVec>>,
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, items: Vec<T>) {
        if let None = self.components.get(&TypeId::of::<T>()) {
            self.components
                .insert(TypeId::of::<T>(), Box::new(items) as Box<dyn AnyVec>);
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&Vec<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|v| v.as_any().downcast_ref::<Vec<T>>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut Vec<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.as_any_mut().downcast_mut::<Vec<T>>())
    }
}

trait AnyVec: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> AnyVec for Vec<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
