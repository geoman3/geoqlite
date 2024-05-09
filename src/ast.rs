pub struct Query {
    statements: Vec<Statement>
}


pub enum Statement {
    Select(SelectStatement),
    Insert(InsertStatement),
    Update(UpdateStatement),
    Delete(DeleteStatement)
}

pub struct SelectStatement {
    select_clause: SelectClause,
    from_clause: Option<Box<FromClause>>,
    where_clause: Option<WhereClause>
}

pub struct InsertStatement {

}

pub struct UpdateStatement {

}

pub struct DeleteStatement {

}

pub enum Clauses {
    Select(SelectClause),
    From(FromClause),
    Where(WhereClause)
    // GroupBy, Having etc ... we'll start with just these select,from,where for now
    // should probably implement joins at some point
}

pub struct SelectClause {
    distinct: bool,
    columns: Vec<ColumnExpression>
}

pub struct FromClause {
    table: TableExpression
}

pub struct WhereClause {
    boolean_expression: BooleanExpression
}

pub enum BooleanExpression {
    And(Box<BinaryOperation>),
    Or(Box<BinaryOperation>),
    GreaterThan(Box<BinaryOperation>),
    LessThan(Box<BooleanExpression>),
    Value(bool)
}

pub struct BinaryOperation {
    left_hand_side: BooleanExpression,
    right_hand_side: BooleanExpression
}

pub enum TableExpression {
    TableReference(String),
    SelectStatement(SelectStatement)
}

pub enum ColumnExpression {
    ColumnReference(String), // I think just using a string here should be fine
    Alias(AliasExpression),
    SqlFunction(SqlFunction)
}

pub struct AliasExpression {
    alias: String,
    column_expression: Box<ColumnExpression>
}

pub struct SqlFunction {

}