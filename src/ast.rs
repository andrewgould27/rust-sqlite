#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Select(SelectStatement),
    Insert(InsertStatement)
}

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub columns: Vec<String>,
    pub table: String, 
    pub condition: Option<Condition>
}

#[derive(Debug, PartialEq)]
pub struct InsertStatement {

}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Comparison(String, ComparisonOperator, Value)
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterEqualThan, 
    LessEqualThan,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String)
}