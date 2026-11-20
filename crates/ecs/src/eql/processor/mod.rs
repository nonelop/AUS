#[cfg(test)]
mod tests;

use super::parser::{FilterTree, ProcessingUnit};
use crate::{Entity, World};
use std::any::TypeId;
use std::collections::HashSet;

#[derive(Debug)]
pub enum ProcessorError {
    CannotLinkType(String),
    CannotFindStorage(TypeId),
}

pub fn processor(
    units: Vec<ProcessingUnit>,
    world: &World,
) -> Result<HashSet<Entity>, ProcessorError> {
    let mut result: HashSet<Entity> = HashSet::new();

    for unit in units {
        match unit {
            ProcessingUnit::SelectingUnit { tree } => {
                result.extend(resolve_selecting_tree(tree, world)?);
            }
        }
    }
    Ok(result)
}

fn resolve_selecting_tree(
    tree: FilterTree,
    world: &World,
) -> Result<HashSet<Entity>, ProcessorError> {
    let mut result: HashSet<Entity> = HashSet::with_capacity(128);

    match tree {
        FilterTree::And { a, b } => {
            let left: HashSet<Entity> = resolve_selecting_tree(*a, world)?;
            let right: HashSet<Entity> = resolve_selecting_tree(*b, world)?;
            result.extend(left.intersection(&right));
        }
        FilterTree::Or { a, b } => {
            let mut left: HashSet<Entity> = resolve_selecting_tree(*a, world)?;
            let right: HashSet<Entity> = resolve_selecting_tree(*b, world)?;
            left.extend(right);
            result.extend(left);
        }
        FilterTree::With(names) => {
            let types: Vec<TypeId> = names
                .into_iter()
                .map(|n| type_linker(world, &n))
                .collect::<Result<Vec<TypeId>, _>>()?;
            result.extend(with_resolver(types, world)?);
        }
        FilterTree::Without(names) => {
            let types: Vec<TypeId> = names
                .into_iter()
                .map(|n| type_linker(world, &n))
                .collect::<Result<Vec<TypeId>, _>>()?;
            result.extend(without_resolver(types, world)?);
        }
    }
    Ok(result)
}

fn with_resolver(with: Vec<TypeId>, world: &World) -> Result<HashSet<Entity>, ProcessorError> {
    let mut result: HashSet<Entity> = HashSet::with_capacity(32);
    let mut is_first_iter = true;

    for id in with {
        if let Some(storage) = world.get_storage(id) {
            let entities: HashSet<Entity> = storage
                .content
                .iter()
                .enumerate()
                .filter(|(_, e)| e.is_some())
                .map(|(i, _)| Entity { id: i })
                .collect();

            if !is_first_iter {
                result.retain(|e| entities.contains(e));
            } else {
                result.extend(entities);
                is_first_iter = false;
            }
        } else {
            return Err(ProcessorError::CannotFindStorage(id));
        }
    }
    Ok(result)
}

fn without_resolver(
    without: Vec<TypeId>,
    world: &World,
) -> Result<HashSet<Entity>, ProcessorError> {
    let mut result: HashSet<Entity> = HashSet::with_capacity(32);
    let mut is_first_iter = true;

    for id in without {
        if let Some(storage) = world.get_storage(id) {
            let mut entities: HashSet<Entity> = storage
                .content
                .iter()
                .enumerate()
                .filter(|(_, e)| e.is_none())
                .map(|(i, _)| Entity { id: i })
                .collect();

            if storage.content.len() < world.entity_registry.len() {
                for id in storage.content.len()..world.entity_registry.len() {
                    if let Some(true) = world.entity_registry.get(id) {
                        entities.insert(Entity { id });
                    }
                }
            }

            if !is_first_iter {
                result.retain(|e| entities.contains(e));
            } else {
                result.extend(entities);
                is_first_iter = false;
            }
        } else {
            return Err(ProcessorError::CannotFindStorage(id));
        }
    }
    Ok(result)
}

fn type_linker(world: &World, type_name: &str) -> Result<TypeId, ProcessorError> {
    if let Some(id) = world.type_names.get(type_name) {
        return Ok(*id);
    }
    Err(ProcessorError::CannotLinkType(type_name.to_string()))
}
