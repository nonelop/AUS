use std::any::TypeId;
use super::World;

#[derive(Debug, PartialEq)]
pub enum Token {
    Type(String),
    With,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedParen,
    InvalidType
}

#[derive(PartialEq, Debug)]
enum ProcessingUnitType {
    Null,
    With
}

#[derive(PartialEq, Debug)]
pub struct ProcessingUnit {
    unit_type: ProcessingUnitType,
    with: Vec<TypeId>
}

impl ProcessingUnit {
    fn new() -> Self {
        Self { unit_type: ProcessingUnitType::Null, with: Vec::new() }
    }
}

pub fn lexer(query: &str) -> Vec<Token> {
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
    lexed_tokens
}

pub fn parser(tokens: Vec<Token>, world: &World) -> Result<Vec<ProcessingUnit>, ParserError> {
    let mut units: Vec<ProcessingUnit> = Vec::new();
    let mut unit_buffer: ProcessingUnit = ProcessingUnit::new();

    for t in tokens {
        match t {
            Token::With => {
                unit_buffer.unit_type = ProcessingUnitType::With;
            }
            Token::OpenParen => {
                if unit_buffer.unit_type == ProcessingUnitType::Null {
                    return Err(ParserError::UnexpectedParen);
                }
            }
            Token::CloseParen => {
                units.push(
                    std::mem::replace(&mut unit_buffer, ProcessingUnit::new())
                );
            }
            Token::Type(type_name) => {
                if let Some(type_id) = type_name_to_type_id(world, &type_name) {
                    unit_buffer.with.push(type_id);
                } else {
                    return Err(ParserError::InvalidType);
                }
            }
        }
    }
    Ok(units)
}

fn push_token(token: &mut String, to: &mut Vec<Token>) {
    if token.is_empty() {
        return;
    }

    if token == "WITH" {
        to.push(Token::With);
    } else {
        to.push(Token::Type(token.to_string()));
    }

    token.clear();
}

pub fn type_name_to_type_id(world: &World, type_name: &str) -> Option<TypeId> {
    for (raw_name, type_id) in &world.type_names{
        let name: Vec<&str> = raw_name.split("::").collect();
        let name = name[name.len() - 1];

        if type_name == name {
            return Some(*type_id);
        }
    }
    None
}