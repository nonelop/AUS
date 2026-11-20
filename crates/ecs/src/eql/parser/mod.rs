#[cfg(test)]
mod tests;

use std::matches;

use super::lexer::Token;

#[derive(Debug, PartialEq)]
pub(crate) enum FilterTree {
    With(Vec<String>),
    Without(Vec<String>),
    And {
        a: Box<FilterTree>,
        b: Box<FilterTree>,
    },
    Or {
        a: Box<FilterTree>,
        b: Box<FilterTree>,
    },
}

#[derive(Debug, PartialEq)]
pub enum FilterError {
    ExpectedArguments,
    UnexpectedArguments,
}

impl FilterTree {
    fn from_raw(
        raw: &RawFilter,
        a: Option<&RawFilter>,
        b: Option<&RawFilter>,
    ) -> Result<Self, FilterError> {
        match (raw, a, b) {
            (RawFilter::And, Some(a), Some(b)) => Ok(Self::And {
                a: Box::new(FilterTree::from_raw(a, None, None)?),
                b: Box::new(FilterTree::from_raw(b, None, None)?),
            }),
            (RawFilter::Or, Some(a), Some(b)) => Ok(Self::Or {
                a: Box::new(FilterTree::from_raw(a, None, None)?),
                b: Box::new(FilterTree::from_raw(b, None, None)?),
            }),
            (RawFilter::And | RawFilter::Or, _, _) => Err(FilterError::ExpectedArguments),
            (RawFilter::With(with), None, None) => Ok(Self::With(with.to_vec())),
            (RawFilter::Without(without), None, None) => Ok(Self::Without(without.to_vec())),
            (RawFilter::With(_) | RawFilter::Without(_), _, _) => {
                Err(FilterError::UnexpectedArguments)
            }
        }
    }
    fn set_b(&mut self, value: FilterTree) -> Result<(), FilterError> {
        match self {
            FilterTree::And { a: _, b } | FilterTree::Or { a: _, b } => {
                **b = value;
                Ok(())
            }
            _ => Err(FilterError::UnexpectedArguments),
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
    SelectingUnit { tree: FilterTree },
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    ExpectedUnitAssignToken,
    ExpectedSelectingFilters,
    ExpectedTypeForFilter,
    UnexpectedType(String),
    UnexpectedLogicalOperator,
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
        self.validate_tokens()?;

        for tokens in self.tokens.split(|t| matches!(t, Token::Select)).skip(1) {
            result.push(Self::build_selecting_unit(tokens)?)
        }

        Ok(result)
    }
    fn validate_tokens(&self) -> Result<(), ParserError> {
        let mut prev: Option<&Token> = None;
        for token in &self.tokens {
            match (prev, token) {
                // Valid
                (None, Token::Select) => {}
                (Some(Token::Select), Token::With | Token::Without) => {}
                (Some(Token::With) | Some(Token::Without), Token::Type(_)) => {}
                (Some(Token::Type(_)), _) => {}
                (Some(Token::And) | Some(Token::Or), Token::With | Token::Without) => {}

                // Invalid
                (None, _) => {
                    return Err(ParserError::ExpectedUnitAssignToken);
                }
                (Some(Token::Select), _) => {
                    return Err(ParserError::ExpectedSelectingFilters);
                }
                (Some(Token::With) | Some(Token::Without), _) => {
                    return Err(ParserError::ExpectedTypeForFilter);
                }
                (Some(Token::And) | Some(Token::Or), _) => {
                    return Err(ParserError::ExpectedSelectingFilters);
                }
                _ => {}
            }
            prev = Some(token)
        }
        Ok(())
    }
    fn build_selecting_unit(slice: &[Token]) -> Result<ProcessingUnit, ParserError> {
        let mut raw = Self::raw_filters(&slice);
        raw = Self::append_operators(raw);
        let try_build = Self::build_filter(raw);
        let result = match try_build {
            Ok(filter_tree) => ProcessingUnit::SelectingUnit { tree: filter_tree },
            Err(e) => {
                return Err(ParserError::FilterError(e));
            }
        };

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
    fn raw_filters(tokens: &[Token]) -> Vec<RawFilter> {
        let mut result: Vec<RawFilter> = Vec::with_capacity(8);

        for token in tokens {
            match token {
                Token::With => result.push(RawFilter::With(Vec::with_capacity(8))),
                Token::Without => result.push(RawFilter::Without(Vec::with_capacity(8))),
                Token::And => result.push(RawFilter::And),
                Token::Or => result.push(RawFilter::Or),
                Token::Type(name) => {
                    if let Some(RawFilter::With(types)) | Some(RawFilter::Without(types)) =
                        result.last_mut()
                    {
                        types.push(name.to_string());
                    }
                }
                _ => {}
            }
        }
        result
    }
    fn build_filter(raw: Vec<RawFilter>) -> Result<FilterTree, FilterError> {
        let mut result: FilterTree;
        let mut iter = raw.into_iter();

        result = match iter.next() {
            Some(RawFilter::With(names)) => {
                FilterTree::from_raw(&RawFilter::With(names), None, None)?
            }
            Some(RawFilter::Without(names)) => {
                FilterTree::from_raw(&RawFilter::Without(names), None, None)?
            }
            _ => {
                return Err(FilterError::UnexpectedArguments);
            }
        };

        while let Some(filter) = iter.next() {
            match filter {
                RawFilter::And => {
                    result = FilterTree::And {
                        a: Box::new(result),
                        b: Box::new(FilterTree::With(vec![])),
                    }
                }
                RawFilter::Or => {
                    result = FilterTree::Or {
                        a: Box::new(result),
                        b: Box::new(FilterTree::With(vec![])),
                    }
                }
                RawFilter::With(_) | RawFilter::Without(_) => {
                    result.set_b(FilterTree::from_raw(&filter, None, None)?)?;
                }
            }
        }
        Ok(result)
    }
}
