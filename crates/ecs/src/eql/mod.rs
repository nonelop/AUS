mod lexer;
mod parser;
mod processor;

use super::{Entity, World};
use lexer::lexer;
use parser::{Parser, ParserError};
use processor::{ProcessorError, processor};
use std::collections::HashSet;

#[derive(Debug)]
pub enum EQLError {
    ParserError(ParserError),
    ProcessorError(ProcessorError),
}

pub fn resolver(query: &str, world: &World) -> Result<HashSet<Entity>, EQLError> {
    let tokens = lexer(query);
    let try_units = Parser::new().parse(tokens);
    match try_units {
        Ok(units) => {
            let try_entities = processor(units, world);
            match try_entities {
                Ok(enitities) => {
                    return Ok(enitities);
                }
                Err(e) => {
                    return Err(EQLError::ProcessorError(e));
                }
            }
        }
        Err(e) => {
            return Err(EQLError::ParserError(e));
        }
    }
}
