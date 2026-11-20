use crate::*;

struct A {}
struct B {}
struct C {}

#[test]
fn single_selecting() {
    let mut world = World::new();
    for _ in 1..=10 {
        world.spawn();
    }
    world.reg_component::<A>();
    world.add_component(3, A {});

    let entities = world.query("SELECT WITH A").unwrap();
    let expected: HashSet<Entity> = HashSet::from_iter(vec![Entity { id: 3 }]);

    assert_eq!(entities, expected)
}

#[test]
fn selecting_with_ops() {
    let mut world = World::new();
    for _ in 1..=10 {
        world.spawn();
    }
    world.reg_component::<A>();
    world.reg_component::<B>();
    world.add_component(3, A {});
    world.add_component(3, B {});

    let entities = world.query("SELECT WITH A AND WITH B").unwrap();
    let expected: HashSet<Entity> = HashSet::from_iter(vec![Entity { id: 3 }]);

    assert_eq!(entities, expected)
}

#[test]
fn selecting_with_nesting_ops() {
    let mut world = World::new();
    for _ in 1..=10 {
        world.spawn();
    }
    world.reg_component::<A>();
    world.reg_component::<B>();
    world.reg_component::<C>();
    world.add_component(3, A {});
    world.add_component(3, B {});

    let entities = world.query("SELECT WITH A AND WITH B AND WITHOUT C").unwrap();
    let expected: HashSet<Entity> = HashSet::from_iter(vec![Entity { id: 3 }]);

    assert_eq!(entities, expected)
}

#[test]
fn unreged_component() {
    let mut world = World::new();

    let entities = world.query("SELECT WITH A");

    assert!(entities.is_err())
}