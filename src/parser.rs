use crate::tokenizer::{Token};
// use std::iter::Peekable;
// use crate::ast;

// pub struct Parser {
//     tokens: Box<dyn Iterator<Item = &Token>>
// }

// impl Parser {
//     pub fn new(tokens: Vec<&Token>) -> Self {
//         let token_iterator = Box::new(tokens.into_iter()) as Box<dyn Iterator<Item = &Token>>;
//         Self {
//             tokens: token_iterator
//         }
//     }

//     pub fn next_token(mut self) -> Token {
//         loop {
//             match self.tokens.next() {
//                 Some(Token::Whitespace) => continue,
//                 token => return token.unwrap_or_else(|| Token::Eof)
//             }
//         }
//     }

//     pub fn parse_statement(mut self) -> ast::Statement {
        
//     }
// }

pub struct Parser {
    tokens: dyn Iterator<Item = Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter()
        }
    }
}