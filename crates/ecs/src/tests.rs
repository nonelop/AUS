use super::*;

#[derive(Debug)]
struct Position {
    pub x: i32,
}

struct Moveable {}

fn test_values() -> (World, Storage<Position>, Entity) {
    let mut world = World::new();
    let storage: Storage<Position> = Storage::new();
    let entity = world.spawn();

    (world, storage, entity)
}

#[test]
fn test_storage_insert() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, Position { x: 5 });
    let component = storage.get(entity.id).unwrap();

    assert_eq!(component.x, 5)
}

#[test]
fn test_storage_remove() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, Position { x: 5 });
    storage.remove(entity.id);

    let component = storage.get(entity.id);

    assert!(component.is_none())
}

#[test]
fn test_storage_get() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, Position { x: 5 });
    let result = storage.get(entity.id).unwrap();

    assert_eq!(result.x, 5)
}

#[test]
fn test_storage_get_mut() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, Position { x: 5 });
    let result = storage.get_mut(entity.id).unwrap();

    assert_eq!(result.x, 5)
}

#[test]
fn test_world_spawn() {
    let (mut world, _storage, _entity) = test_values();
    let entity = world.spawn();

    assert_eq!(entity.id + 1, world.id_counter)
}

#[test]
fn test_world_append_get() {
    let (mut world, _storage, entity) = test_values();
    world.add_component(entity.id, Position { x: 5 });
    let result = world.get_component::<Position>(entity.id).unwrap();

    assert_eq!(result.x, 5)
}

// EQL

#[test]
fn test_eql_selecting() {
    let (mut world, _, _) = test_values();
    let entity = world.spawn();
    world.add_component(entity.id, Position { x: 10 });

    let result = world.query("(SELECT WITH Position)").unwrap();
    for e in result {
        assert_eq!(e, entity)
    }
}

#[test]
fn test_eql_logical_ops() {
    let (mut world, _, _) = test_values();
    world.reg_component::<Moveable>();
    let entity = world.spawn();
    world.add_component(entity.id, Position { x: 10 });

    let result = world
        .query("(SELECT WITH Moveable OR WITH Position)")
        .unwrap();
    for e in result {
        assert_eq!(e, entity)
    }
}

#[test]
fn test_eql_without() {
    let (mut world, _, _) = test_values();
    world.reg_component::<Position>();
    let entity = world.spawn();

    let result = world.query("(SELECT WITHOUT Position)").unwrap();
    for e in result {
        assert_eq!(e, entity)
    }
}
