pub struct Statement {

}

// holds all information we need to run a select statement
pub struct Select {
    pub select: Vec<Expression>,
    pub from: TableExpression
}

pub struct Create {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>
}

pub struct Insert {
    pub table_name: String,
    pub columns: Vec<String>,
    pub values: Vec<SqlValue>
}

pub struct Delete {
    // which table to delete from
    pub table_name: String,
    // condition which decides if we delete the row or not
    pub condition: Expression
}
// represents some operation that is evaluated in the context of a select item or a where clause
// eg. SELECT [expression1] FROM [table] WHERE [expression2]
pub struct Expression {

}

pub struct TableExpression {

}

pub struct ColumnDefinition {

}

pub enum SqlValue {
    SqlNumber(f64),
    SqlString(String),
    SqlBoolean(bool)
}