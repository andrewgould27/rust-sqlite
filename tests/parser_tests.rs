#[cfg(test)]
mod tests {
    use super::*;
    use rust_sqlite::lexer::Lexer;
    use rust_sqlite::parser::Parser;
    use rust_sqlite::ast::{
        ASTNode, ComparisonOperator, Condition, InsertStatement, SelectStatement, UpdateStatement, Value
    };

    fn parse_sql(sql: &str) -> Result<ASTNode, String> {
        let lexer = Lexer::new(sql);
        let mut parser = Parser::new(lexer);
        parser.parse()
    }

    #[test]
    fn test_simple_select() {
        let ast = parse_sql("SELECT * FROM users").unwrap();
        assert_eq!(
            ast, 
            ASTNode::Select(SelectStatement {
                columns: vec!["*".to_string()],
                table: "users".to_string(),
                condition: None
            })
        )
    }

    #[test]
    fn test_select_with_where_clause() {
        let ast = parse_sql("SELECT * FROM users WHERE age > 18").unwrap();
        assert_eq!(
            ast,
            ASTNode::Select(SelectStatement {
                columns: vec!["*".to_string()],
                table: "users".to_string(),
                condition: Some(Condition::Comparison(
                    "age".to_string(),
                    ComparisonOperator::GreaterThan,
                    Value::Number(18.0),
                )),
            })
        );
    }

    #[test]
    fn test_insert_with_columns() {
        let ast = parse_sql("INSERT INTO users (name, age) VALUES ('John', 30)").unwrap();
        assert_eq!(
            ast,
            ASTNode::Insert(InsertStatement {
                table: "users".to_string(),
                columns: vec!["name".to_string(), "age".to_string()],
                values: vec![Value::String("John".to_string()), Value::Number(30.0)],
            })
        );
    }

    #[test]
    fn test_select_with_different_operators() {
        let operators = vec![
            ("=", ComparisonOperator::Equals),
            ("<>", ComparisonOperator::NotEquals),
            (">", ComparisonOperator::GreaterThan),
            ("<", ComparisonOperator::LessThan),
            (">=", ComparisonOperator::GreaterEqualThan),
            ("<=", ComparisonOperator::LessEqualThan),
        ];

        for (op_str, op_enum) in operators {
            let sql = format!("SELECT * FROM users WHERE age {} 18", op_str);
            let ast = parse_sql(&sql).unwrap();
            assert_eq!(
                ast,
                ASTNode::Select(SelectStatement {
                    columns: vec!["*".to_string()],
                    table: "users".to_string(),
                    condition: Some(Condition::Comparison(
                        "age".to_string(),
                        op_enum,
                        Value::Number(18.0),
                    )),
                })
            );
        }
    }

    #[test]
    fn test_simple_update() {
        let ast = parse_sql("UPDATE users SET name = 'Andrew' WHERE id = 27").unwrap();
        assert_eq!(ast, 
            ASTNode::Update(UpdateStatement {
                table: "users".to_string(),
                updates: vec![
                    ("name".to_string(), Value::String("Andrew".to_string()))
                ],
                condition: Some(Condition::Comparison("id".to_string(), ComparisonOperator::Equals, Value::Number(27.0)))
            })
        )
    }

    #[test]
    fn test_update_multiple_columns() {
        let ast = parse_sql("UPDATE products SET price = 19.99, stock = 100 WHERE category = 'Sports'").unwrap();
        assert_eq!(ast, 
            ASTNode::Update(UpdateStatement {
                table: "products".to_string(),
                updates: vec![
                    ("price".to_string(), Value::Number(19.99)), 
                    ("stock".to_string(), Value::Number(100.0))
                ],
                condition: Some(Condition::Comparison("category".to_string(), ComparisonOperator::Equals, Value::String("Sports".to_string())))
            })
        )
    }

    #[test]
    #[should_panic]
    fn test_bad_select_statement() {
        parse_sql("SELECT FROM id WHERE name = 'Andrew'").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_bad_update_statement() {
        parse_sql("UPDATE table SET = 10").unwrap();
    }
}