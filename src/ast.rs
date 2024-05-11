use crate::tokenizer::{Keyword, Token, TokenCollection};


#[derive(Debug)]
pub struct Query {
    statements: Vec<Statement>
}

impl Query {
    pub fn new(tokens: &mut TokenCollection) -> Self {
        let mut statements: Vec<Statement> = Vec::new();
        loop {
            let next_statement: Statement = match tokens.next_token() {
               Token::Keyword(Keyword::Select) => {
                    println!("I matched the select keyword!");
                    Statement::Select(SelectStatement::new(tokens))
                },
               _ => break
            };
            statements.push(next_statement)
        }
        Query{ statements: statements }
    }
}

#[derive(Debug)]
pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement)
}

#[derive(Debug)]
pub struct SelectStatement {
    select_clause: SelectClause,
    from_clause: Option<Box<FromClause>>,
    where_clause: Option<WhereClause>
}

impl SelectStatement {
    pub fn new(tokens: &mut TokenCollection) -> Self {
        // handle select clause
        let select_clause = SelectClause::new(tokens);
        let from_clause = FromClause::new(tokens);
        SelectStatement{
            select_clause: select_clause,
            from_clause: Some(Box::new(from_clause)),
            where_clause: None
        }
    }
}

#[derive(Debug)]
pub struct InsertStatement {

}

#[derive(Debug)]
pub struct UpdateStatement {

}

#[derive(Debug)]
pub struct DeleteStatement {

}

#[derive(Debug)]
pub enum Clauses {
    Select(SelectClause),
    From(FromClause),
    Where(WhereClause)
    // GroupBy, Having etc ... we'll start with just these select,from,where for now
    // should probably implement joins at some point
}

#[derive(Debug)]
pub struct SelectClause {
    distinct: bool,
    columns: Vec<ColumnExpression>
}

impl SelectClause {
    pub fn new(tokens: &mut TokenCollection) -> Self {
        let mut columns: Vec<ColumnExpression> = Vec::new();
        loop {
            let next_column = match tokens.peek_token() {
                Token::Comma => {
                    let _ = tokens.next_token();
                    continue;
                },
                Token::Keyword(Keyword::From) => {
                    let _ = tokens.next_token();
                    break
                },
                _ => ColumnExpression::new(tokens)
            };
            columns.push(next_column);
        }
        SelectClause{ distinct: false, columns: columns }
    }
}

#[derive(Debug)]
pub struct FromClause {
    table: TableExpression
}

impl FromClause {
    pub fn new(tokens: &mut TokenCollection) -> Self {
        let expression = match tokens.next_token() {
            Token::Entity(ref_name) => TableExpression::TableReference(ref_name),
            Token::OpenParentheses => TableExpression::SelectStatement(SelectStatement::new(tokens)),
            tok => panic!("unexpect token: {:?}", tok)
        };
        return FromClause{ table: expression };
    }
}

#[derive(Debug)]
pub struct WhereClause {
    boolean_expression: BooleanExpression
}

#[derive(Debug)]
pub enum BooleanExpression {
    And(Box<BinaryOperation>),
    Or(Box<BinaryOperation>),
    GreaterThan(Box<BinaryOperation>),
    LessThan(Box<BooleanExpression>),
    Value(bool)
}

#[derive(Debug)]
pub struct BinaryOperation {
    left_hand_side: BooleanExpression,
    right_hand_side: BooleanExpression
}

#[derive(Debug)]
pub enum TableExpression {
    TableReference(String),
    SelectStatement(SelectStatement)
}

#[derive(Debug)]
pub enum ColumnExpression {
    ColumnReference(String), // I think just using a string here should be fine
    Alias(AliasExpression),
    SqlFunction(SqlFunction),
    Wildcard
}

impl ColumnExpression {
    pub fn new(tokens: &mut TokenCollection) -> Self {
        let col = match tokens.peek_token() {
            Token::Multiply => {
                let _ = tokens.next_token();
                ColumnExpression::Wildcard
            },
            Token::Entity(name) => {
                let _ = tokens.next_token();
                ColumnExpression::ColumnReference(name.to_string())
            },
            // TODO: Implement alias expressions, ie. select foo as bar from baz
            token => panic!("unexpected token: {:?}", token)
        };
        return col;
    }
}

#[derive(Debug)]
pub struct AliasExpression {
    alias: String,
    column_expression: Box<ColumnExpression>
}

#[derive(Debug)]
pub struct SqlFunction {

}