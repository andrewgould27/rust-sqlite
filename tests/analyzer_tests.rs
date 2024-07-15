#[cfg(test)]
mod tests {
    use super::*;
    use rust_sqlite::semantic_analyzer::{SemanticAnalyzer, SemanticError};
    use rust_sqlite::ast::{ASTNode, ComparisonOperator, Condition, OrderByClause, OrderDirection, SelectStatement, Value};
    use rust_sqlite::schema::{DatabaseSchema, TableSchema, ColumnType};
    use std::collections::HashMap;

    fn setup_test_schema() -> DatabaseSchema {
        let mut user_columns = HashMap::new();
        user_columns.insert("id".to_string(), ColumnType::Integer);
        user_columns.insert("name".to_string(), ColumnType::String);
        user_columns.insert("age".to_string(), ColumnType::Integer);

        let mut product_columns = HashMap::new();
        product_columns.insert("id".to_string(), ColumnType::Integer);
        product_columns.insert("name".to_string(), ColumnType::String);
        product_columns.insert("price".to_string(), ColumnType::Float);

        let mut tables = HashMap::new();
        tables.insert("users".to_string(), TableSchema { columns: user_columns });
        tables.insert("products".to_string(), TableSchema { columns: product_columns });

        DatabaseSchema { tables }
    }

    #[test]
    fn test_valid_select() {
        let schema = setup_test_schema();
        let analyzer = SemanticAnalyzer::new(schema);

        let select_stmt = SelectStatement {
            columns: vec!["name".to_string(), "age".to_string()],
            table: "users".to_string(), 
            condition: None, 
            order_by: Vec::<OrderByClause>::new()
        };

        let result = analyzer.analyze(&ASTNode::Select(select_stmt));
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_table() {
        let schema = setup_test_schema();
        let analyzer = SemanticAnalyzer::new(schema);

        let select_stmt = SelectStatement {
            columns: vec!["name".to_string()],
            table: "not_a_real_table".to_string(), 
            condition: None, 
            order_by: Vec::<OrderByClause>::new()
        };

        let result = analyzer.analyze(&ASTNode::Select(select_stmt));
        assert!(matches!(result, Err(SemanticError::TableNotFound(_))));
    }

    #[test]
    fn test_missing_column() {
        let schema = setup_test_schema();
        let analyzer = SemanticAnalyzer::new(schema);

        let select_stmt = SelectStatement {
            columns: vec!["not_a_real_column".to_string()],
            table: "users".to_string(), 
            condition: None, 
            order_by: Vec::<OrderByClause>::new()
        };

        let result = analyzer.analyze(&ASTNode::Select(select_stmt));
        assert!(matches!(result, Err(SemanticError::ColumnNotFound(_))));
    }

    #[test]
    fn test_condition_mismatch() {
        let schema = setup_test_schema();
        let analyzer = SemanticAnalyzer::new(schema);

        let select_stmt = SelectStatement {
            columns: vec!["name".to_string()],
            table: "users".to_string(), 
            condition: Some(Condition::Comparison(
                "age".to_string(), 
                ComparisonOperator::Equals, 
                Value::String("eighteen".to_string())
            )),
            order_by: Vec::<OrderByClause>::new()
        };

        let result = analyzer.analyze(&ASTNode::Select(select_stmt));
        assert!(matches!(result, Err(SemanticError::TypeMismatch(_))));
    }

    #[test]
    fn test_invalid_order_by_column() {
        let schema = setup_test_schema();
        let analyzer = SemanticAnalyzer::new(schema);

        let select_stmt = SelectStatement {
            columns: vec!["name".to_string(), "age".to_string()],
            table: "users".to_string(),
            condition: None, 
            order_by: vec![
                OrderByClause {
                    column: "non_existent_column".to_string(),
                    order: OrderDirection::Asc
                }
            ]
        };

        let result = analyzer.analyze(&ASTNode::Select(select_stmt));
        assert!(matches!(result, Err(SemanticError::ColumnNotFound(_))));
    }
}
