use super::entity::Entity;

pub struct Storage<T> {
    items: Vec<Option<T>>
}

impl<T> Storage<T> {
    pub fn new () -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, e: Entity, value: T) {
        let idx = e.0 as usize;
        if idx >= self.items.len() {
            self.items.resize_with(idx + 1, || None);
        }
        self.items[idx] = Some(value);
    }

    pub fn get(&self, e: Entity) -> Option<&T> {
        self.items.get(e.0 as usize)?.as_ref()
    }

    pub fn get_mut(&mut self, e: Entity) -> Option<&mut T> {
        self.items.get_mut(e.0 as usize)?.as_mut()
    }

    pub fn remove(&mut self, e: Entity) -> Option<T> {
        self.items.get_mut(e.0 as usize)?.take()
    }
}