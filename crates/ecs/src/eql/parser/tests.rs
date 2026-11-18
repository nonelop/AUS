use super::*;

#[test]
fn selecting_unit() {
    let tokens = lexer("SELECT WITH Position WITHOUT Velocity");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::And {
            a: Box::new(Filter::With(vec!["Position".to_string()])),
            b: Box::new(Filter::Without(vec!["Velocity".to_string()]))
        }]
        }]
    )
}

#[test]
fn raw_filters() {
    let tokens = lexer("WITH Position OR WITH Velocity");
    let mut parser = Parser::new();
    parser.tokens = tokens;
    let filters = parser.raw_filters().unwrap();

    assert_eq!(
        filters,
        vec![
            RawFilter::With(vec!["Position".to_string()]),
            RawFilter::Or,
            RawFilter::With(vec!["Velocity".to_string()])
        ]
    )
}

#[test]
fn append_operators() {
    let tokens = lexer("WITH Position WITHOUT Velocity");
    let mut parser = Parser::new();
    parser.tokens = tokens;
    let mut filters = parser.raw_filters().unwrap();
    filters = Parser::append_operators(filters);

    assert_eq!(
        filters,
        vec![
            RawFilter::With(vec!["Position".to_string()]),
            RawFilter::And,
            RawFilter::Without(vec!["Velocity".to_string()])
        ]
    )
}

#[test]
fn append_operators_exception() {
    let tokens = lexer("WITH Position");
    let mut parser = Parser::new();
    parser.tokens = tokens;
    let mut filters = parser.raw_filters().unwrap();
    filters = Parser::append_operators(filters);

    assert_eq!(
        filters,
        vec![RawFilter::With(vec!["Position".to_string()]),]
    )
}

#[test]
fn build_filters() {
    let tokens = lexer("WITH Position WITHOUT Velocity");
    let mut parser = Parser::new();
    parser.tokens = tokens;
    let mut raw_filters = parser.raw_filters().unwrap();
    raw_filters = Parser::append_operators(raw_filters);
    let filters = Parser::build_filters(raw_filters).unwrap();

    assert_eq!(
        filters,
        vec![Filter::And {
            a: Box::new(Filter::With(vec!["Position".to_string()])),
            b: Box::new(Filter::Without(vec!["Velocity".to_string()]))
        }]
    )
}
