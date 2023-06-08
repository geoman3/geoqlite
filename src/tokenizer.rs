use std::fmt;

// start with the basics
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
    SingleQuotedString,
    DoubleQuotedString,

    // numbers
    Number,

    // End of file
    Eof,

    Invalid,
}



// for now we will alzily implement this and return the whole 
// token array at once
pub fn tokenize(query_str: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut query_chars = query_str.chars().peekable();

    loop {
        let c = match query_chars.next() {
            Some(q_char) => q_char,
            None => {
                tokens.push(Token::Eof);
                return tokens;
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
                        Some('_' | 'a'..='z' | 'A'..='Z') => {
                            let ch = query_chars.next().unwrap_or_default();
                            word.push(ch);
                        },
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
            }
            _ => Token::Invalid

        };

        tokens.push(next_token);
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Token::Entity(s) => format!("Entity: \"{s}\""),
            Token::Keyword(kw) => format!("Keyword{kw}"),
            Token::Plus => "Plus".to_owned(),
            Token::Minus => "Minus".to_owned(),
            Token::Multiply => "Multiply".to_owned(),
            Token::Divide => "Divide".to_owned(),
            Token::Modulo => "Modulo".to_owned(),
            Token::Equality => "Equality".to_owned(),
            Token::DoubleEquality => "DoubleEquality".to_owned(),
            Token::GreaterThan => "GreaterThan".to_owned(),
            Token::GreaterThanOrEqual => "GreaterThanOrEqual".to_owned(),
            Token::LessThan => "LessThan".to_owned(),
            Token::LessThanOrEqual => "LessThanOrEqual".to_owned(),
            Token::Wildcard => "Wildcard".to_owned(),
            Token::Whitespace => "Whitespace".to_owned(),
            Token::Comma => "Comma".to_owned(),
            Token::SemiColon => "SemiColon".to_owned(),
            Token::SingleQuotedString => "SingleQuotedString".to_owned(),
            Token::DoubleQuotedString => "DoubleQuotedString".to_owned(),
            Token::Number => "Number".to_owned(),
            Token::Eof => "Eof".to_owned(),
            Token::Invalid => "Invalid".to_owned(),
        };
        
        write!(f, "Token({})", repr)
    }
}