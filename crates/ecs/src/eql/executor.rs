use super::ParserError;
use super::parser::Filter;
use super::{Entity, World};
use log::trace;
use std::any::TypeId;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub(crate) enum Unit {
    New,
    SelectingUnit(SelectingUnit),
    _SettingUnit(),
}

#[derive(PartialEq, Debug)]
pub(crate) struct SelectingUnit {
    pub(crate) filters: Vec<Filter>,
}
enum LogicalOperator {
    Null,
    And,
    Or,
}

#[derive(Debug)]
pub(crate) enum ProcessingResult {
    New,
    Select { selected: HashSet<Entity> },
}

impl ProcessingResult {
    fn new() -> Self {
        Self::New
    }
}

impl Unit {
    pub(crate) fn add_selecting_filter(&mut self, filter: Filter) -> Result<(), ParserError> {
        trace!("UNIT: ADD FILTER: {:?}", filter);
        match self {
            Self::SelectingUnit(selecting) => {
                selecting.filters.push(filter);
                return Ok(());
            }
            _ => Err(ParserError::ExpectedSelectingStatment),
        }
    }
    pub(crate) fn add_to_selecting_filter(&mut self, id: TypeId) -> Result<(), ParserError> {
        trace!("UNIT: ADD TO FILTER: {:?}", id);
        match self {
            Self::SelectingUnit(selecting) => {
                let last = selecting.filters.last_mut();
                match last {
                    Some(f) => {
                        match f {
                            Filter::With(with) => with.push(id),
                            Filter::Without(without) => without.push(id),
                            _ => {}
                        }
                        return Ok(());
                    }
                    None => {
                        return Err(ParserError::ExpectedSelectingStatment);
                    }
                }
            }
            _ => {
                return Err(ParserError::ExpectedSelectingStatment);
            }
        }
    }
    pub(crate) fn change_to_selecting(&mut self) {
        match self {
            Self::SelectingUnit(_) => {}
            _ => {
                let new_unit = Self::SelectingUnit(SelectingUnit {
                    filters: Vec::new(),
                });
                *self = new_unit;
            }
        }
    }
    pub(crate) fn is_selecting(&self) -> bool {
        match self {
            Self::SelectingUnit(_) => true,
            _ => false,
        }
    }
    pub(crate) fn append_operators(&mut self) {
        match self {
            Self::SelectingUnit(selecting) => {
                trace!("UNIT: APPEND OPERATORS: BEFORE: {:?}", selecting.filters);
                let mut new_filters: Vec<Filter> = Vec::new();
                let mut iteration = 0;
                let last = selecting.filters.len() - 1;

                new_filters.extend(selecting.filters.clone());

                for f in &mut selecting.filters {
                    trace!("UNIT: APPEND OPERATORS: FILTER: {:?}", f);
                    if iteration != last && last >= 1 && *f != Filter::And && *f != Filter::Or {
                        new_filters.insert(iteration + 1, Filter::And);
                    }
                    iteration += 1;
                }

                selecting.filters = new_filters;
                trace!("UNIT: APPEND OPERATORS: AFTER: {:?}", selecting.filters)
            }
            _ => return,
        }
    }
}

pub(crate) fn processor(units: Vec<Unit>, world: &World) -> ProcessingResult {
    let mut result: ProcessingResult = ProcessingResult::new();

    for u in units {
        match u {
            Unit::SelectingUnit(selecting) => {
                result = ProcessingResult::Select {
                    selected: array_from_filters(selecting.filters, world),
                }
            }
            _ => {}
        }
    }
    trace!("PROCESSOR: FINAL: {:?}", result);
    result
}

fn array_from_filters(filters: Vec<Filter>, world: &World) -> HashSet<Entity> {
    trace!("ARRAY FROM FILTERS: FILTERS: {:?}", filters);
    let mut buffer: HashSet<Entity> = HashSet::new();
    let mut operator_flag: LogicalOperator = LogicalOperator::Null;

    for filter in filters {
        let mut entities: HashSet<Entity> = HashSet::new();
        match filter {
            Filter::With(with) => {
                entities = with_types(with, world);
            }
            Filter::Without(without) => {
                entities = without_types(without, world);
            }
            Filter::And => operator_flag = LogicalOperator::And,
            Filter::Or => operator_flag = LogicalOperator::Or,
        }
        match operator_flag {
            LogicalOperator::And => buffer = buffer.intersection(&entities).copied().collect(),
            LogicalOperator::Or => buffer.extend(entities),
            LogicalOperator::Null => buffer = HashSet::from_iter(entities),
        }
    }
    buffer
}

fn with_types(type_ids: Vec<TypeId>, world: &World) -> HashSet<Entity> {
    let mut buffer: HashSet<Entity> = HashSet::new();
    let mut is_first_iter = true;

    trace!("WITH IDS: IDS: {:?}", type_ids);

    for id in type_ids {
        if let Some(storage) = world.components.get(&id) {
            let with_type: HashSet<Entity> = storage
                .content
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_some())
                .map(|(i, _)| Entity { id: i })
                .collect();

            trace!("WITH IDS: ITERATION: RESULT: {:?}", with_type);
            if !is_first_iter {
                buffer = buffer.intersection(&with_type).copied().collect()
            } else {
                is_first_iter = false;
                buffer.extend(with_type);
            }

            trace!("WITH IDS: ITERATION: BUFFER: {:?}", buffer)
        }
    }
    trace!("WITH IDS: RESULT: {:?}", buffer);
    buffer
}

fn without_types(type_ids: Vec<TypeId>, world: &World) -> HashSet<Entity> {
    trace!("WITHOUT IDS: IDS: {:?}", type_ids);
    let mut buffer: HashSet<Entity> = HashSet::new();
    let mut is_first_iter = true;

    for id in type_ids {
        if let Some(storage) = world.components.get(&id) {
            let mut without_type: HashSet<Entity> = storage
                .content
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_none())
                .map(|(i, _)| Entity { id: i })
                .collect();

            if storage.content.len() <= world.id_counter && !storage.content.is_empty() {
                for id in storage.content.len() - 1..=world.id_counter {
                    without_type.insert(Entity { id });
                }
            }

            trace!("WITHOUT IDS: ITERATION: RESULT: {:?}", without_type);
            if !is_first_iter {
                buffer = buffer.intersection(&without_type).copied().collect()
            } else {
                is_first_iter = false;
                buffer.extend(without_type);
            }
            trace!("WITHOUT IDS: ITERATION: BUFFER: {:?}", buffer)
        }
    }
    buffer
}

fn type_linker(world: &World, type_name: &str) -> Option<TypeId> {
    for (raw_name, type_id) in &world.type_names {
        let name: Vec<&str> = raw_name.split("::").collect();
        let name = name[name.len() - 1];

        if type_name == name {
            return Some(*type_id);
        }
    }
    None
}
