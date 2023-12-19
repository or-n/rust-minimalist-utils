use std::ops::{Add, Rem, Sub};

pub struct Mod<T>(pub T);

#[derive(Clone, Copy)]
pub struct ModValue<T>(T);

impl<T: Copy> ModValue<T> {
    pub fn get(&self) -> T {
        self.0
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T>> Mod<T> {
    pub fn new(&self, value: T) -> ModValue<T> {
        ModValue(value % self.0)
    }

    pub fn add(&self, a: ModValue<T>, b: ModValue<T>) -> ModValue<T> {
        ModValue((a.get() + b.get()) % self.0)
    }

    pub fn neg(&self, value: ModValue<T>) -> ModValue<T> {
        ModValue((self.0 - value.get()) % self.0)
    }
}
