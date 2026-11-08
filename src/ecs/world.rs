use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::ecs::storage::Storage;

use super::entity::Entity;

pub struct World {
    current_id: u64,
    storages: HashMap<TypeId, Box<dyn Any>>
}

impl World {
    pub fn new() -> Self {
        Self { current_id: 0, storages: HashMap::new() }
    }

    pub fn spawn(&mut self) -> Entity {
        let e = Entity(self.current_id);
        self.current_id += 1;
        e
    }

    pub fn insert<T: 'static>(&mut self, e: Entity, component: T) {
        let key = TypeId::of::<T>();

        let storage = self.storages.get_mut(&key);

        match storage {
            Some(storage) => {
                match storage.downcast_mut::<Storage<T>>() {
                    Some(storage) => {
                        storage.insert(e, component);
                    }
                    None => unreachable!("Storage<T> downcasting failed")
                }
            }
            None => {
                let mut new: Storage<T> = Storage::new();

                new.insert(e, component);

                self.storages.insert(key, Box::new(new));
            }
        }
    }
}