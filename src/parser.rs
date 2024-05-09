use crate::ast;
use crate::tokenizer::{tokenize, TokenCollection};


pub fn parse_statement(query_str: &str) -> ast::Query {
    let mut tokens = tokenize(query_str);
    parse_tokens(&mut tokens)
}

pub fn parse_tokens(tokens: &mut TokenCollection) -> ast::Query {
    ast::Query::new(tokens)
}
