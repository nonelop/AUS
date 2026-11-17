use std::iter::Peekable;
use std::vec::IntoIter;

use super::ParserError;
use super::lexer::Token;
use log::trace;

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Filter {
    With(Vec<String>),
    Without(Vec<String>),
    And { a: Box<Filter>, b: Box<Filter> },
    Or { a: Box<Filter>, b: Box<Filter> },
}

enum RawFilter {
    With(Vec<String>),
    Without(Vec<String>),
    And,
    Or,
}

pub(crate) enum ParsedUnit {
    Selecting { filters: Vec<Filter> },
}

pub(crate) struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    fn raw_filters(&mut self) -> Vec<RawFilter> {
        let mut raw_filters: Vec<RawFilter> = Vec::with_capacity(8);

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::With | Token::Without => {
                    let filter: Vec<String> = Vec::with_capacity(8);
                    if let Some(token) = self.tokens.next() {
                        while let Some(next) = self.tokens.peek() {
                            match next {
                                Token::Type(type_name) => filter.push(type_name.to_string()),
                                _ => break
                            }
                        }
                    }
                    match token {
                        Token::With => raw_filters.push(RawFilter::With(filter)),
                        Token::Without => raw_filters.push(RawFilter::Without(filter)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        raw_filters
    }
}

pub(crate) fn parser(tokens: Vec<Token>) -> Result<Vec<ParsedUnit>, ParserError> {
    let parsed_units: Vec<ParsedUnit> = Vec::with_capacity(8);

    /* */

    Ok(parsed_units)
}
