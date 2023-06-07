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
}

pub enum Token {
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
    And,
    Or,
    Wildcard, // the * in `select * from table`

    // whitespace
    Whitespace,

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
            ' ' | '\n' | '\t' => Token::Whitespace,

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = c.to_string();
                loop {
                    match query_chars.peek() {
                        Some('_' | 'a'..='z' | 'A'..='Z') => {
                            word.push(query_chars.next().unwrap());
                        },
                        Some(_) => break,
                        None => break,
                    }
                }

                match word.as_str() {
                    "Select" => Token::Keyword(Keyword::Select),
                    "From" => Token::Keyword(Keyword::From),
                    "Insert" => Token::Keyword(Keyword::Insert),
                    "Create" => Token::Keyword(Keyword::Create),
                    "Delete" => Token::Keyword(Keyword::Delete),
                    "Drop" => Token::Keyword(Keyword::Drop),
                    "Join" => Token::Keyword(Keyword::Join),
                    "Inner" => Token::Keyword(Keyword::Inner),
                    "Outer" => Token::Keyword(Keyword::Outer),
                    "Left" => Token::Keyword(Keyword::Left),
                    "Right" => Token::Keyword(Keyword::Right),
                    "Where" => Token::Keyword(Keyword::Where),
                    "Having" => Token::Keyword(Keyword::Having),
                    "Group" => Token::Keyword(Keyword::Group),
                    "Sort" => Token::Keyword(Keyword::Sort),
                    "By" => Token::Keyword(Keyword::By),
                    _ => Token::Entity(word),
                }
            },
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
            Token::And => "And".to_owned(),
            Token::Or => "Or".to_owned(),
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