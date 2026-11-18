use log::trace;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Type(String),
    Select,
    With,
    Without,
    And,
    Or,
}

pub(crate) fn lexer(query: &str) -> Vec<Token> {
    trace!("LEXER: QUERY: {:?}", query);
    let mut token_buffer: String = String::new();
    let mut lexed_tokens: Vec<Token> = Vec::new();

    for c in query.chars() {
        if c.is_whitespace() {
            push_token(&mut token_buffer, &mut lexed_tokens);
        } else {
            token_buffer.push(c);
        }
    }
    push_token(&mut token_buffer, &mut lexed_tokens);
    trace!("LEXER: TOKENS: {:?}", lexed_tokens);
    lexed_tokens
}

fn push_token(token: &mut String, to: &mut Vec<Token>) {
    trace!("PUSH TOKEN: STR: {:?}", token);
    if token.is_empty() {
        return;
    }

    if token == "WITH" {
        to.push(Token::With);
    } else if token == "WITHOUT" {
        to.push(Token::Without);
    } else if token == "OR" {
        to.push(Token::Or);
    } else if token == "AND" {
        to.push(Token::And);
    } else if token == "SELECT" {
        to.push(Token::Select);
    } else {
        to.push(Token::Type(token.to_string()));
    }

    trace!("PUSH TOKEN: TOKEN: {:?}", to.last());

    token.clear();
}
