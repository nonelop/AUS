use super::*;
use crate::eql::lexer::lexer;

#[test]
fn with_a() {
    let tokens = lexer("SELECT WITH A");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", unit);

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::With(vec!["A".to_string()])]
        }]
    )
}

#[test]
fn without_a() {
    let tokens = lexer("SELECT WITHOUT A");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", unit);

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::Without(vec!["A".to_string()])]
        }]
    )
}

#[test]
fn with_a_without_b() {
    let tokens = lexer("SELECT WITH A WITHOUT B");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", unit);

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::And {
                a: Box::new(Filter::With(vec!["A".to_string()])),
                b: Box::new(Filter::Without(vec!["B".to_string()]))
            }]
        }]
    )
}

#[test]
fn with_a_or_with_b() {
    let tokens = lexer("SELECT WITH A OR WITH B");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", unit);

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::Or {
                a: Box::new(Filter::With(vec!["A".to_string()])),
                b: Box::new(Filter::With(vec!["B".to_string()]))
            }]
        }]
    )
}

#[test]
fn nesting_operators() {
    let tokens = lexer("SELECT WITH A AND WITHOUT B OR WITH C");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", unit);

    assert_eq!(
        unit,
        vec![ProcessingUnit::SelectingUnit {
            filters: vec![Filter::Or {
                a: Box::new(Filter::And {
                    a: Box::new(Filter::With(vec!["A".to_string()])),
                    b: Box::new(Filter::Without(vec!["B".to_string()]))
                }),
                b: Box::new(Filter::With(vec!["C".to_string()]))
            }]
        }]
    )
}

#[test]
fn many_select() {
    let tokens = lexer("SELECT WITH A AND WITH B SELECT WITH C");
    let mut parser = Parser::new();
    let units = parser.parse(tokens).unwrap();

    println!("UNIT: {:?}", units);

    assert_eq!(
        units,
        vec![
            ProcessingUnit::SelectingUnit {
                filters: vec![Filter::And {
                    a: Box::new(Filter::With(vec!["A".to_string()])),
                    b: Box::new(Filter::With(vec!["B".to_string()]))
                }]
            },
            ProcessingUnit::SelectingUnit {
                filters: vec![Filter::With(vec!["C".to_string()])]
            }
        ]
    )
}

#[test]
fn unexpected_starts_operator() {
    let tokens = lexer("SELECT AND WITH A");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens);

    println!("UNIT: {:?}", unit);

    assert!(unit.is_err())
}

#[test]
fn unexpected_ends_operator() {
    let tokens = lexer("SELECT WITH A AND");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens);

    println!("UNIT: {:?}", unit);

    assert!(unit.is_err())
}

#[test]
fn expected_unit_assigment() {
    let tokens = lexer("WITH A");
    let mut parser = Parser::new();
    let unit = parser.parse(tokens);

    println!("UNIT: {:?}", unit);

    assert!(unit.is_err())
}
