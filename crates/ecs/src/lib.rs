use std::{any::{Any, TypeId}, collections::HashMap};

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Storage<T> {
    pub content: Vec<Option<T>>
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self { content: Vec::new() }
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

struct World {
    pub components: HashMap<TypeId, Storage<Box<dyn Any>>>,
    pub id_counter: usize
}

impl World {
    pub fn new() -> Self {
        Self { components: HashMap::new(), id_counter: 0 }
    }
    
    pub fn spawn(&mut self) -> Entity {
        let e = Entity { id: self.id_counter };
        self.id_counter += 1;
        e
    }

    pub fn get_component<T: 'static>(&self, id: usize) -> Option<&T> {
        let key = TypeId::of::<T>();
        let storage = self.components.get(&key)?;

        storage.get(id)?.downcast_ref()
    }

    pub fn get_by_query(&self, query: Query) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::new();
        let mut candidates: Vec<usize> = Vec::new();
        let mut is_first = true;

        for i in query.with {
            let key = i.as_ref().type_id();
            let storage = self.components.get(&key);

            if let Some(storage) = storage {
                let some_entities: Vec<usize> = storage.content.iter().enumerate()
                .filter(|(_, c)| c.is_some())
                .map(|(i, _)| i )
                .collect();

                if is_first {
                    candidates.extend(some_entities);
                    is_first = false
                }
                else {
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

    pub fn append_component<T: 'static>(&mut self, id: usize, component: T) {
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

                self.components.insert(key, new_storage);
            }
        }
    }
}

struct Query {
    pub with: Vec<Box<dyn Any>>
}

impl Query {
    fn new() -> Self {
        Self { with: Vec::new() }
    }

    fn with(mut self, value: Box<dyn Any>) -> Self {
        self.with.push(value);
        self
    } 
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entity {
    pub id: usize
}

trait Boxed {
    fn as_boxed(self) -> Box<dyn Any> where Self: Sized + 'static {
        let boxed: Box<dyn Any> = Box::new(self);
        boxed
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i32
}

struct Velocity {
    x: i32
}

impl Position {
    pub const DEFAULT: Self = Self { x: 0 };
}

impl Velocity {
    pub const DEFAULT: Self = Self { x: 0 };
}

impl Boxed for Position {}
impl Boxed for Velocity {}

fn main() {
    let mut world = World::new();

    let entity1 = world.spawn();
    let entity2 = world.spawn();

    let position1 = Position { x: 5 };
    let position2 = Position { x: 10 };
    let velocity1 = Velocity { x: 1 };

    world.append_component::<Position>(entity1.id, position1);
    world.append_component::<Velocity>(entity1.id, velocity1);
    world.append_component::<Position>(entity2.id, position2);

    let entities = world.get_by_query(Query::new().with(Position::DEFAULT.as_boxed()).with(Velocity::DEFAULT.as_boxed()));

    for e in entities {
        println!("{}", e.id);
    }
}
