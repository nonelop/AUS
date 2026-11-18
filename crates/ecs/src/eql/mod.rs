mod executor;
mod lexer;
mod parser;

use super::{Entity, World};
// use executor::{ProcessingResult, processor};
use lexer::lexer;
use parser::{Parser, ParserError};
use std::collections::HashSet;

#[derive(Debug)]
pub enum EQLError {
    ParserError(ParserError),
}

pub fn resolver(query: &str, world: &World) -> Result<HashSet<Entity>, EQLError> {
    let tokens = lexer(query);
    if let Ok(units) = Parser::new().parse(tokens) {
        // let result = processor(units, world);
    }

    // Ok(match result {
    //     ProcessingResult::Select { selected } => selected,
    //     _ => HashSet::new(),
    // })

    Ok(HashSet::new())
}
