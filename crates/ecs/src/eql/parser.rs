use super::ParserError;
use super::lexer::Token;
use log::trace;

pub(crate) struct FilterList {
    filters: Vec<Filter>,
}

impl FilterList {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            filters: Vec::with_capacity(capacity),
        }
    }
    fn add_filter(&mut self, token: Token) -> Result<&mut Filter, ParserError> {
        match token {
            Token::With => {
                self.filters.push(Filter::With(Vec::with_capacity(8)));
                Ok(self.filters.last_mut().unwrap())
            }
            Token::Without => {
                self.filters.push(Filter::Without(Vec::with_capacity(8)));
                Ok(self.filters.last_mut().unwrap())
            }
            Token::And => {
                self.filters.push(Filter::And {
                    a: Box::new(Filter::New),
                    b: Box::new(Filter::New),
                });
                Ok(self.filters.last_mut().unwrap())
            }
            Token::Or => {
                self.filters.push(Filter::Or {
                    a: Box::new(Filter::New),
                    b: Box::new(Filter::New),
                });
                Ok(self.filters.last_mut().unwrap())
            }
            _ => Err(ParserError::ExpectedSelectingFilter),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Filter {
    New,
    With(Vec<String>),
    Without(Vec<String>),
    And { a: Box<Filter>, b: Box<Filter> },
    Or { a: Box<Filter>, b: Box<Filter> },
}

impl Filter {
    fn append_type(&mut self, type_name: String) -> Result<(), ParserError> {
        match self {
            Self::With(with) => {
                with.push(type_name);
                Ok(())
            }
            Self::Without(without) => {
                without.push(type_name);
                Ok(())
            }
            _ => Err(ParserError::ExpectedSelectingFilter),
        }
    }
}

pub(crate) enum ParsedUnit {
    New,
    Selecting { filters: FilterList },
}

impl ParsedUnit {
    fn selecting(&mut self) {
        *self = ParsedUnit::Selecting {
            filters: FilterList::with_capacity(8),
        }
    }
    fn new_filter(&mut self, token: Token) -> Result<&mut Filter, ParserError> {
        if let Self::Selecting { filters } = self {
            match token {
                Token::With => Ok(filters.add_filter(token)?),
                Token::Without => Ok(filters.add_filter(token)?),
                _ => Err(ParserError::ExpectedSelectingFilter),
            }
        } else {
            Err(ParserError::ExpectedSelectingStatment)
        }
    }
}

pub(crate) fn parser(tokens: Vec<Token>) -> Result<Vec<ParsedUnit>, ParserError> {
    let parsed_units: Vec<ParsedUnit> = Vec::with_capacity(8);
    let mut current_unit = ParsedUnit::New;
    let mut current_filter: &mut Filter = &mut Filter::New;

    for t in tokens {
        match t {
            Token::Select => current_unit.selecting(),
            Token::Type(type_name) => {
                current_filter.append_type(type_name);
            }
            _ => {
                current_filter = current_unit.new_filter(t)?;
            }
        }
    }

    Ok(parsed_units)
}
