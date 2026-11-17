mod executor;
mod lexer;
mod parser;

use super::{Entity, World};
use executor::{ProcessingResult, processor};
use lexer::lexer;
use parser::parser;
use std::collections::HashSet;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedParen,
    UnexpectedLogicalOperator,
    UnexpectedSelectingFilter,
    ExpectedSelectingFilter,
    ExpectedSelectingStatment,
    InvalidType(String),
}

pub fn resolver(query: &str, world: &World) -> Result<HashSet<Entity>, ParserError> {
    let tokens = lexer(query);
    let units = parser(tokens)?;
    let result = processor(units, world);

    Ok(match result {
        ProcessingResult::Select { selected } => selected,
        _ => HashSet::new(),
    })
}
