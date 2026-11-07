use std::collections::HashMap;

use crate::body::Body;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(usize);

pub struct World {
    body_id_counter: usize,
    body_id_map: HashMap<BodyId, Box<dyn Body>>,
}

impl World {
    pub fn new() -> Self {
        World { 
            body_id_counter: 0, 
            body_id_map: HashMap::new(), 
        }
    }
    pub fn spawn(&mut self, body: Box<dyn Body>) -> BodyId {
        let id = self.body_id_counter;

        self.body_id_map.insert(BodyId(self.body_id_counter), body);

        self.body_id_counter += 1;

        BodyId(id)
    }

    pub fn despawn(&mut self, id: BodyId) {
        self.body_id_map.remove(&id);
    }

    pub fn get_body_ids(&self) -> Vec<BodyId> {
        let mut vec: Vec<BodyId> = Vec::new();

        for i in 0..self.body_id_counter {
            match self.body_id_map.get(&BodyId(i)) {
                Some(_body) => {
                    vec.push(BodyId(i))
                }
                None => continue
            }
        }
        vec
    }

    pub fn get_body(&self, id: BodyId) -> Option<&dyn Body> {
        match self.body_id_map.get(&id) {
            Some(body) => Some(& **body),
            None => None
        }
    }

    pub fn get_mut_body(&mut self, id: BodyId) -> Option<&mut dyn Body> {
        match self.body_id_map.get_mut(&id) {
            Some(body) => Some(&mut **body),
            None => None
        }
    }

    pub fn get_many_mut_body(&mut self, a: BodyId, b: BodyId) 
    -> Option<(&mut dyn Body, &mut dyn Body)> {
        if a == b {
            return None;
        }

        // SAFETY:
        // выше мы проверили что a != b, то есть мы никогда не вернем две &mut на один обьект
        // также оператор ? после get_mut вернет None если обьекта с таким айди нету

        unsafe {
            let body_a: *mut Box<dyn Body> = self.body_id_map.get_mut(&a)?;
            let body_b: *mut Box<dyn Body> = self.body_id_map.get_mut(&b)?;

            Some((&mut **body_a, &mut **body_b))
        }
    }
}