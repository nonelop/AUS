use super::{Entity, World};
use std::{any::TypeId, collections::HashSet};
use log::{debug, trace};

#[derive(Debug, PartialEq)]
enum Token {
    Type(String),
    Select,
    With,
    Without,
    And,
    Or,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedParen,
    UnexpectedLogicalOperator,
    UnexpectedSelectingFilter,
    ExpectedSelectingStatment,
    InvalidType(String),
}

#[derive(PartialEq, Clone, Debug)]
enum Filter {
    With(Vec<TypeId>),
    Without(Vec<TypeId>),
    And,
    Or,
}

enum LogicalOperator {
    Null,
    And,
    Or,
}

#[derive(PartialEq, Debug)]
enum Unit {
    New,
    SelectingUnit(SelectingUnit),
    SettingUnit(),
}

impl Unit {
    fn new() -> Self {
        Self::New
    }
    fn add_selecting_filter(&mut self, filter: Filter) -> Result<(), ParserError> {
        trace!("UNIT: ADD FILTER: {:?}", filter);
        match self {
            Self::SelectingUnit(selecting) => {
                selecting.filters.push(filter);
                return Ok(());
            }
            _ => {
                Err(ParserError::ExpectedSelectingStatment)
            }
        }
    }
    fn add_to_selecting_filter(&mut self, id: TypeId) -> Result<(), ParserError> {
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
            _ => Ok(()),
        }
    }
    fn change_to_selecting(&mut self) {
        trace!("UNIT: CHANGING TO SELECTING");
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
    fn is_selecting(&self) -> bool {
        match self {
            Self::SelectingUnit(_) => true,
            _ => false,
        }
    }
    fn append_operators(&mut self) {
        match self {
            Self::SelectingUnit(selecting) => {
                trace!("UNIT: APPEND OPERATORS: BEFORE: {:?}", selecting.filters);
                let mut new_filters: Vec<Filter> = Vec::new();
                let mut iteration = 0;
                let last = selecting.filters.len() - 1;

                new_filters.extend(selecting.filters.clone());

                for f in &mut selecting.filters {
                    if iteration != last && last >= 2 && *f != Filter::And {
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

#[derive(PartialEq, Debug)]
struct SelectingUnit {
    filters: Vec<Filter>,
}

enum ProcessingResult {
    New,
    Select(SelectingResult),
}

impl ProcessingResult {
    fn new() -> Self {
        Self::New
    }
}

struct SelectingResult {
    selected: HashSet<Entity>,
}

pub fn resolver(query: &str, world: &World) -> Result<HashSet<Entity>, ParserError> {
    let tokens = lexer(query);
    let units = parser(tokens, world)?;
    let result = processor(units, world);

    Ok(match result {
        ProcessingResult::Select(s) => s.selected,
        _ => HashSet::new(),
    })
}

fn lexer(query: &str) -> Vec<Token> {
    trace!("LEXER: QUERY: {:?}", query);
    let mut token_buffer: String = String::new();
    let mut lexed_tokens: Vec<Token> = Vec::new();

    for c in query.chars() {
        match c {
            '(' => {
                push_token(&mut token_buffer, &mut lexed_tokens);
                lexed_tokens.push(Token::OpenParen);
            }
            ')' => {
                push_token(&mut token_buffer, &mut lexed_tokens);
                lexed_tokens.push(Token::CloseParen);
            }
            c => {
                if c.is_whitespace() {
                    push_token(&mut token_buffer, &mut lexed_tokens);
                } else {
                    token_buffer.push(c);
                }
            }
        }
    }
    trace!("LEXER: TOKENS: {:?}", lexed_tokens);
    lexed_tokens
}

fn parser(tokens: Vec<Token>, world: &World) -> Result<Vec<Unit>, ParserError> {
    let mut units: Vec<Unit> = Vec::new();
    let mut unit_buffer: Unit = Unit::new();

    trace!("PARSER: TOKENS: {:?}", tokens);

    for t in tokens {
        match t {
            Token::Select => {
                unit_buffer.change_to_selecting();
            }
            Token::With => {
                unit_buffer.add_selecting_filter(Filter::With(Vec::new()))?;
            }
            Token::Without => {
                unit_buffer.add_selecting_filter(Filter::Without(Vec::new()))?;
            }
            Token::OpenParen => { /* */ }
            Token::CloseParen => {
                if unit_buffer.is_selecting() {
                    unit_buffer.append_operators();
                    units.push(std::mem::replace(&mut unit_buffer, Unit::New))
                }
            }
            Token::Type(name) => {
                if let Some(id) = type_name_to_type_id(world, &name) {
                    unit_buffer.add_to_selecting_filter(id)?;
                } else {
                    return Err(ParserError::InvalidType(name));
                }
            }
            Token::And => {
                unit_buffer.add_selecting_filter(Filter::And)?;
            }
            Token::Or => {
                unit_buffer.add_selecting_filter(Filter::Or)?;
            }
        }
    }
    trace!("PARSER: UNITS: {:?}", units);
    Ok(units)
}

fn processor(units: Vec<Unit>, world: &World) -> ProcessingResult {
    let mut result: ProcessingResult = ProcessingResult::new();

    for u in units {
        match u {
            Unit::SelectingUnit(selecting) => {
                result = ProcessingResult::Select(SelectingResult {
                    selected: array_from_filters(selecting.filters, world),
                })
            }
            _ => {}
        }
    }
    result
}

// === Utils ===

fn push_token(token: &mut String, to: &mut Vec<Token>) {
    trace!("PUSH TOKEN: STR: {:?}", token);
    if token.is_empty() {
        return;
    }

    if token == "WITH" {
        to.push(Token::With);
    } else if token == "WITHOUT" {
        to.push(Token::Without);
    } else if token == "OR" {
        to.push(Token::Or);
    } else if token == "AND" {
        to.push(Token::And);
    } else if token == "SELECT" {
        to.push(Token::Select);
    } else {
        to.push(Token::Type(token.to_string()));
    }

    trace!("PUSH TOKEN: TOKEN: {:?}", to.last());

    token.clear();
}

fn type_name_to_type_id(world: &World, type_name: &str) -> Option<TypeId> {
    for (raw_name, type_id) in &world.type_names {
        let name: Vec<&str> = raw_name.split("::").collect();
        let name = name[name.len() - 1];

        if type_name == name {
            return Some(*type_id);
        }
    }
    None
}

fn array_from_filters(filters: Vec<Filter>, world: &World) -> HashSet<Entity> {
    let mut result: HashSet<Entity> = HashSet::new();
    let mut buffer: HashSet<Entity> = HashSet::new();
    let mut operator_flag: LogicalOperator = LogicalOperator::Null;

    trace!("ARRAY FROM FILTERS: FILTERS: {:?}", filters);

    for filter in filters {
        match filter {
            Filter::With(with) => {
                let entities = with_ids(with, world);

                match operator_flag {
                    LogicalOperator::And => buffer = arrays_and(entities, Vec::from_iter(buffer)),
                    LogicalOperator::Or => buffer = arrays_or(entities, Vec::from_iter(buffer)),
                    LogicalOperator::Null => buffer = arrays_and(entities, Vec::from_iter(buffer)),
                }
            }
            Filter::And => operator_flag = LogicalOperator::And,
            Filter::Or => operator_flag = LogicalOperator::Or,
            _ => {}
        }
    }
    result.extend(buffer);
    result
}

fn with_ids(ids: Vec<TypeId>, world: &World) -> Vec<Entity> {
    let mut result: Vec<Entity> = Vec::new();
    let mut buffer: HashSet<Entity> = HashSet::new();

    trace!("WITH IDS: IDS: {:?}", ids);

    for id in ids {
        if let Some(storage) = world.components.get(&id) {
            let with_id: Vec<Entity> = storage
                .content
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_some())
                .map(|(i, _)| Entity { id: i })
                .collect();

            trace!("WITH IDS: ITERATION: RESULT: {:?}", with_id);
            buffer = arrays_and(with_id, Vec::from_iter(buffer));
            trace!("WITH IDS: ITERATION: BUFFER: {:?}", buffer)
        }
    }
    for e in buffer {
        result.push(e);
    }
    trace!("WITH IDS: RESULT: {:?}", result);
    result
}

fn arrays_and(arr1: Vec<Entity>, arr2: Vec<Entity>) -> HashSet<Entity> {
    let result: HashSet<Entity>;

    if arr1.is_empty() {
        return HashSet::from_iter(arr2);
    } else if arr2.is_empty() {
        return HashSet::from_iter(arr1);
    }

    let arr_and: Vec<Entity> = arr1.into_iter().filter(|e| arr2.contains(e)).collect();

    result = HashSet::from_iter(arr_and);

    result
}

fn arrays_or(arr1: Vec<Entity>, arr2: Vec<Entity>) -> HashSet<Entity> {
    let mut result: HashSet<Entity> = HashSet::new();

    for e in arr1 {
        result.insert(e);
    }
    for e in arr2 {
        result.insert(e);
    }

    result
}
