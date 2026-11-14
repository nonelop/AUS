use std::{
    any::{Any, TypeId, type_name},
    collections::HashMap,
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
    pub(crate) components: HashMap<TypeId, Storage<Box<dyn Any>>>,
    pub(crate) type_names: HashMap<String, TypeId>,
    entity_registry: Vec<Option<Entity>>,
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
                .resize_with(self.id_counter + 1, || None);
        }
        self.entity_registry[self.id_counter] = Some(e);

        self.id_counter += 1;
        e
    }

    pub fn despawn(&mut self, id: usize) {
        if self.entity_registry.len() > id {
            self.entity_registry[id] = None
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
        } else if self.entity_registry.get(id) == Some(&None) {
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

    pub fn get_by_query(&self, query: Query) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::new();
        let mut candidates: Vec<usize> = Vec::new();
        let mut is_first = true;

        for key in query.with {
            let storage = self.components.get(&key);

            if let Some(storage) = storage {
                let some_entities: Vec<usize> = storage
                    .content
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_some())
                    .map(|(i, _)| i)
                    .collect();

                if is_first {
                    candidates.extend(some_entities);
                    is_first = false
                } else {
                    candidates.retain(|id| some_entities.contains(id));
                }
            } else {
                return Vec::new();
            }
        }

        for id in &candidates {
            entities.push(Entity { id: *id });
        }

        entities
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

                let type_name = type_name::<T>();

                self.components.insert(key, new_storage);
                self.type_names.insert(type_name.to_string(), key);
            }
        }
    }
}

pub struct Query {
    with: Vec<TypeId>,
}

impl Query {
    pub fn new() -> Self {
        Self { with: Vec::new() }
    }

    pub fn with<T: 'static>(mut self) -> Self {
        self.with.push(TypeId::of::<T>());
        self
    }
}
