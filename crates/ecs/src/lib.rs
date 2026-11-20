use eql::resolver;
use std::{
    any::{Any, TypeId, type_name},
    collections::{HashMap, HashSet},
};

#[cfg(test)]
mod tests;

pub mod eql;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity {
    pub id: usize,
}

#[derive(Debug)]
pub(crate) struct Storage<T> {
    pub(crate) content: Vec<Option<T>>,
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: usize, value: T) {
        if id >= self.content.len() {
            self.content.resize_with(id + 1, || None);
        }

        self.content[id] = Some(value)
    }

    pub fn remove(&mut self, id: usize) {
        if id >= self.content.len() {
            return;
        }

        self.content[id] = None
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.content.get(id)?.as_ref()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.content.get_mut(id)?.as_mut()
    }
}

pub struct World {
    components: HashMap<TypeId, Storage<Box<dyn Any>>>,
    type_names: HashMap<String, TypeId>,
    entity_registry: Vec<bool>,
    id_counter: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            type_names: HashMap::new(),
            entity_registry: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let e = Entity {
            id: self.id_counter,
        };

        if self.entity_registry.len() <= self.id_counter {
            self.entity_registry
                .resize_with(self.id_counter + 1, || false);
        }
        self.entity_registry[self.id_counter] = true;

        self.id_counter += 1;
        e
    }

    pub fn despawn(&mut self, id: usize) {
        if self.entity_registry.len() > id {
            self.entity_registry[id] = false
        } else {
            return;
        }

        for (_, storage) in &mut self.components {
            storage.remove(id);
        }
    }

    pub fn check(&self, id: usize) -> bool {
        if self.entity_registry.len() <= id {
            false
        } else if self.entity_registry.get(id) == Some(&false) {
            false
        } else {
            true
        }
    }

    pub fn get_component<T: 'static>(&self, id: usize) -> Option<&T> {
        let key = TypeId::of::<T>();
        let storage = self.components.get(&key)?;

        storage.get(id)?.downcast_ref()
    }

    pub fn get_mut_component<T: 'static>(&mut self, id: usize) -> Option<&mut T> {
        let key = TypeId::of::<T>();
        let storage = self.components.get_mut(&key)?;

        storage.get_mut(id)?.downcast_mut()
    }

    pub fn get_two_mut_components<F: 'static, S: 'static>(
        &mut self,
        id1: usize,
        id2: usize,
    ) -> Option<(&mut F, &mut S)> {
        if TypeId::of::<F>() != TypeId::of::<S>() || id1 != id2 {
            unsafe {
                let first: *mut F = self.get_mut_component::<F>(id1)?;
                let second: *mut S = self.get_mut_component::<S>(id2)?;

                Some((&mut *first, &mut *second))
            }
        } else {
            None
        }
    }

    pub(crate) fn get_storage(&self, id: TypeId) -> Option<&Storage<Box<dyn Any>>> {
        self.components.get(&id)
    }

    pub fn reg_component<T: 'static>(&mut self) {
        let key = TypeId::of::<T>();

        if let Some(_) = self.components.get(&key) {
            return;
        } else {
            let new_storage = Storage::new();
            self.components.insert(key, new_storage);

            let raw_name = type_name::<T>();
            let name: Vec<&str> = raw_name.split("::").collect();
            let name = *name.last().unwrap();

            self.type_names.insert(name.to_string(), key);
        }
    }

    pub fn add_component<T: 'static>(&mut self, id: usize, component: T) {
        let key = TypeId::of::<T>();
        let storage = self.components.get_mut(&key);

        let boxed: Box<dyn Any> = Box::new(component);

        match storage {
            Some(storage) => {
                storage.insert(id, boxed);
            }
            None => {
                let mut new_storage = Storage::new();
                new_storage.insert(id, boxed);

                let raw_name = type_name::<T>();
                let name: Vec<&str> = raw_name.split("::").collect();
                let name = *name.last().unwrap();

                self.components.insert(key, new_storage);
                self.type_names.insert(name.to_string(), key);
            }
        }
    }

    pub fn query(&mut self, query: &str) -> Result<HashSet<Entity>, eql::EQLError> {
        resolver(query, self)
    }
}
