use super::*;

#[derive(Debug)]
struct TestComponent {
    pub x: i32,
}

fn test_values() -> (World, Storage<TestComponent>, Entity) {
    let mut world = World::new();
    let storage: Storage<TestComponent> = Storage::new();
    let entity = world.spawn();

    (world, storage, entity)
}

#[test]
fn test_storage_insert() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, TestComponent { x: 5 });
    let component = storage.get(entity.id).unwrap();

    assert_eq!(component.x, 5)
}

#[test]
fn test_storage_remove() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, TestComponent { x: 5 });
    storage.remove(entity.id);

    let component = storage.get(entity.id);

    assert!(component.is_none())
}

#[test]
fn test_storage_get() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, TestComponent { x: 5 });
    let result = storage.get(entity.id).unwrap();

    assert_eq!(result.x, 5)
}

#[test]
fn test_storage_get_mut() {
    let (_world, mut storage, entity) = test_values();
    storage.insert(entity.id, TestComponent { x: 5 });
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
    world.add_component(entity.id, TestComponent { x: 5 });
    let result = world.get_component::<TestComponent>(entity.id).unwrap();

    assert_eq!(result.x, 5)
}
