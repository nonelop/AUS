#[cfg(test)]
mod tests;

use std::matches;

use super::lexer::{Token, lexer};

#[derive(Debug, PartialEq)]
enum Filter {
    With(Vec<String>),
    Without(Vec<String>),
    And { a: Box<Filter>, b: Box<Filter> },
    Or { a: Box<Filter>, b: Box<Filter> },
}

#[derive(Debug)]
enum FilterError {
    ExpectedArguments,
    UnexpectedArguments,
    UnreachableException,
}

impl Filter {
    fn from_raw(
        raw: &RawFilter,
        a: Option<&RawFilter>,
        b: Option<&RawFilter>,
    ) -> Result<Self, FilterError> {
        match (raw, a, b) {
            (RawFilter::And, Some(a), Some(b)) => Ok(Self::And {
                a: Box::new(Filter::from_raw(a, None, None)?),
                b: Box::new(Filter::from_raw(b, None, None)?),
            }),
            (RawFilter::Or, Some(a), Some(b)) => Ok(Self::Or {
                a: Box::new(Filter::from_raw(a, None, None)?),
                b: Box::new(Filter::from_raw(b, None, None)?),
            }),
            (RawFilter::And | RawFilter::Or, _, _) => Err(FilterError::ExpectedArguments),
            (RawFilter::With(with), None, None) => Ok(Self::With(with.to_vec())),
            (RawFilter::Without(without), None, None) => Ok(Self::Without(without.to_vec())),
            (RawFilter::With(_) | RawFilter::Without(_), _, _) => {
                Err(FilterError::UnexpectedArguments)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum RawFilter {
    With(Vec<String>),
    Without(Vec<String>),
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ProcessingUnit {
    SelectingUnit { filters: Vec<Filter> },
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedType(String),
    ExpectedFilters,
    UnexpectedLogicalOperator,
    ExpecterdUnitAssignToken,
    FilterError(FilterError),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::with_capacity(12),
        }
    }
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<ProcessingUnit>, ParserError> {
        let mut result: Vec<ProcessingUnit> = Vec::with_capacity(8);
        self.tokens = tokens;

        for token in &self.tokens {
            match token {
                Token::Select => {
                    result.push(self.build_selecting_unit()?);
                }
                _ => {}
            }
        }

        Ok(result)
    }
    fn build_selecting_unit(&self) -> Result<ProcessingUnit, ParserError> {
        let result: ProcessingUnit;

        let mut raw = self.raw_filters()?;
        raw = Self::append_operators(raw);
        let try_build = Self::build_filters(raw);
        match try_build {
            Ok(filters) => {
                result = ProcessingUnit::SelectingUnit { filters }
            }
            Err(e) => {
                return Err(ParserError::FilterError(e));
            } 
        }

        Ok(result)
    }
    fn append_operators(filters: Vec<RawFilter>) -> Vec<RawFilter> {
        let mut result: Vec<RawFilter> = Vec::with_capacity(filters.len() * 2);
        let mut prev_is_filter = false;

        for filter in filters {
            let is_filter = matches!(filter, RawFilter::With(_) | RawFilter::Without(_));
            if is_filter && prev_is_filter {
                result.push(RawFilter::And);
            }
            prev_is_filter = is_filter;
            result.push(filter);
        }
        result
    }
    fn raw_filters(&self) -> Result<Vec<RawFilter>, ParserError> {
        let mut buffer: Vec<RawFilter> = Vec::with_capacity(8);

        for token in &self.tokens {
            match token {
                Token::With => buffer.push(RawFilter::With(Vec::with_capacity(8))),
                Token::Without => buffer.push(RawFilter::Without(Vec::with_capacity(8))),
                Token::And => buffer.push(RawFilter::And),
                Token::Or => buffer.push(RawFilter::Or),
                Token::Type(name) => {
                    if let Some(RawFilter::With(types)) | Some(RawFilter::Without(types)) =
                        buffer.last_mut()
                    {
                        types.push(name.to_string());
                    } else {
                        return Err(ParserError::UnexpectedType(name.to_string()));
                    }
                }
                _ => {}
            }
        }
        Ok(buffer)
    }
    fn build_filters(raw: Vec<RawFilter>) -> Result<Vec<Filter>, FilterError> {
        let raw_len = raw.len();
        let mut iteration = 0;
        let mut result: Vec<Filter> = Vec::with_capacity(raw_len);

        for filter in &raw {
            if let Some(prev) = raw.get(if iteration == 0 {
                iteration
            } else {
                iteration - 1
            }) && let Some(next) = raw.get(iteration + 1)
            {
                match filter {
                    RawFilter::And | RawFilter::Or => {
                        result.push(Filter::from_raw(filter, Some(prev), Some(next))?)
                    }
                    _ => {
                        if raw_len == 1 {
                            result.push(Filter::from_raw(filter, None, None)?)
                        }
                    }
                }
            }
            iteration += 1;
        }
        Ok(result)
    }
}
