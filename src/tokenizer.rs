use std::fmt;
use std::collections::VecDeque;


#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Select,
    From,
    Insert,
    Create,
    Delete,
    Drop,
    Join,
    Inner,
    Outer,
    Left,
    Right,
    Where,
    Having,
    Group,
    Sort,
    By,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Token {
    Whitespace,

    // could be column or table or db
    Entity(String),

    Keyword(Keyword),

    // operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equality,
    DoubleEquality,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Wildcard, // the * in `select * from table`

    // delimiters
    Comma,
    SemiColon,

    // strings
    SingleQuotedString(String),
    DoubleQuotedString(String),

    // numbers
    Number(String),

    // et al.
    OpenParentheses,
    CloseParentheses,

    // End of file
    Eof,

    Invalid,
}



// for now we will lazily implement this and return the whole 
// token array at once, might be worth breaking this down to producing 1 token at a time
// to hand off to a second thread to build the AST but Im not expecting BIG strings
pub fn tokenize(query_str: &str) -> TokenCollection {
    let mut tokens: VecDeque<Token> = VecDeque::new();
    let mut query_chars = query_str.chars().peekable();

    loop {
        // grab the first char of the next token
        let c = match query_chars.next() {
            Some(q_char) => q_char,
            // if there isnt a next char, then we have read the whole string
            // return the tokens
            None => {
                tokens.push_back(Token::Eof);
                return TokenCollection::new(tokens);
            }
        };
        let next_token: Token = match c {
            // coalesce whitespace chars into a single token
            ' ' | '\n' | '\t' => {
                loop {
                    match query_chars.peek() {
                        Some(' ' | '\n' | '\t') => { let _ = query_chars.next(); },
                        Some(_) => break,
                        None => break
                    };
                }
                Token::Whitespace
            },

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = c.to_string();
                loop {
                    match query_chars.peek() {
                        Some('_' | 'a'..='z' | 'A'..='Z') => word.push(query_chars.next().unwrap_or_default()),
                        Some(_) => break,
                        None => break,
                    }
                }

                // this should maybe be a method on the Token or Keyword enum ---
                match word.to_lowercase().as_str() {
                    "select" => Token::Keyword(Keyword::Select),
                    "from" => Token::Keyword(Keyword::From),
                    "insert" => Token::Keyword(Keyword::Insert),
                    "create" => Token::Keyword(Keyword::Create),
                    "delete" => Token::Keyword(Keyword::Delete),
                    "drop" => Token::Keyword(Keyword::Drop),
                    "join" => Token::Keyword(Keyword::Join),
                    "inner" => Token::Keyword(Keyword::Inner),
                    "outer" => Token::Keyword(Keyword::Outer),
                    "left" => Token::Keyword(Keyword::Left),
                    "right" => Token::Keyword(Keyword::Right),
                    "where" => Token::Keyword(Keyword::Where),
                    "having" => Token::Keyword(Keyword::Having),
                    "group" => Token::Keyword(Keyword::Group),
                    "sort" => Token::Keyword(Keyword::Sort),
                    "by" => Token::Keyword(Keyword::By),
                    "and" => Token::Keyword(Keyword::And),
                    "or" => Token::Keyword(Keyword::Or),
                    _ => Token::Entity(word),
                }
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => {
                // figure out if multiply or select wildcard
                Token::Multiply
            },
            '(' => Token::OpenParentheses,
            ')' => Token::CloseParentheses,
            '/' => Token::Divide,
            '%' => Token::Modulo,
            '=' => {
                match query_chars.peek() {
                    Some('=') => {
                        let _ = query_chars.next();
                        Token::DoubleEquality
                    },
                    Some(_) => Token::Equality,
                    None => Token::Equality
                }
            },
            '>' => {
                match query_chars.peek() {
                    Some('=') => {
                        let _ = query_chars.next();
                        Token::GreaterThanOrEqual
                    }
                    Some(_) => Token::GreaterThan,
                    None => Token::GreaterThan
                }
            },
            '<' => {
                match query_chars.peek() {
                    Some('=') => {
                        let _ = query_chars.next();
                        Token::LessThanOrEqual
                    }
                    Some(_) => Token::LessThan,
                    None => Token::LessThan
                }
            },
            ',' => Token::Comma,
            ';' => Token::SemiColon,
            '"' => {
                let mut word = c.to_string();
                loop {
                    match query_chars.peek() {
                        Some('"') => {
                            word.push(query_chars.next().unwrap_or_default());
                            break;
                        }
                        Some(_) => word.push(query_chars.next().unwrap_or_default()),
                        None => break,
                    }
                }
                Token::DoubleQuotedString(word)
            },
            '\'' => {
                let mut word = c.to_string();
                loop {
                    match query_chars.peek() {
                        Some('\'') => {
                            word.push(query_chars.next().unwrap_or_default());
                            break;
                        }
                        Some(_) => word.push(query_chars.next().unwrap_or_default()),
                        None => break,
                    }
                }
                Token::SingleQuotedString(word)
            },
            '0'..='9' | '.' => {
                let mut num_string = c.to_string();
                loop {
                    match query_chars.peek() {
                        Some('0'..='9' | '.' ) => num_string.push(query_chars.next().unwrap_or_default()),
                        Some(_) => break,
                        None => break,
                    }
                }
                Token::Number(num_string)
            }
            _ => Token::Invalid

        };

        tokens.push_back(next_token);
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Keyword::Select => "Select",
            Keyword::From => "From",
            Keyword::Insert => "Insert",
            Keyword::Create => "Create",
            Keyword::Delete => "Delete",
            Keyword::Drop => "Drop",
            Keyword::Join => "Join",
            Keyword::Inner => "Inner",
            Keyword::Outer => "Outer",
            Keyword::Left => "Left",
            Keyword::Right => "Right",
            Keyword::Where => "Where",
            Keyword::Having => "Having",
            Keyword::Group => "Group",
            Keyword::Sort => "Sort",
            Keyword::By => "By",
            Keyword::And => "And",
            Keyword::Or => "Or"
        };

        write!(f, "Token({})", repr)
    }
}


pub struct TokenCollection {
    // idk if I actually want this public
    tokens: VecDeque<Token>
}

impl TokenCollection {
    pub fn new(tokens: VecDeque<Token>) -> Self {
        println!("here are the tokens: {:?}", tokens);
        TokenCollection { tokens: tokens }
    }

    // gets the next non-whitespace token
    pub fn next_token(&mut self) -> Token {
        loop {
            let return_token = match self.tokens.pop_front() {
                Some(Token::Whitespace) => continue,
                Some(token) => token,
                None => Token::Eof
            };
            println!("{:?}", return_token);
            return return_token;
        }
    }

    // "peeks" the next non-whitespace token
    pub fn peek_token(&mut self) -> Token {
        loop {
            match self.tokens.pop_front() {
                Some(Token::Whitespace) => continue,
                Some(token) => {
                    self.tokens.push_front(token.clone());
                    return token;
                },
                None => return Token::Eof
            }
        }
    }
}