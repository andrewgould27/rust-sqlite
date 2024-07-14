use rust_sqlite::lexer::{
    Lexer, Token
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_select() {
        let sql = "SELECT * FROM users";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select, 
            Token::Asterisk,
            Token::From,
            Token::Identifier("users".to_string())
        ]);
    }

    #[test]
    fn test_select_with_columns() {
        let sql = "SELECT id, name FROM users";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select,
            Token::Identifier("id".to_string()),
            Token::Comma,
            Token::Identifier("name".to_string()),
            Token::From,
            Token::Identifier("users".to_string())
        ]);
    }

    #[test]
    fn test_string_literals() {
        let sql = "SELECT name FROM users WHERE name = \"John Doe\"";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select,
            Token::Identifier("name".to_string()),
            Token::From, 
            Token::Identifier("users".to_string()),
            Token::Where, 
            Token::Identifier("name".to_string()),
            Token::Operator("=".to_string()),
            Token::String("John Doe".to_string())
        ]);
    }

    #[test]
    fn test_numeric_literals() {
        let sql = "SELECT * FROM temperatures WHERE value > 98.6";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select,
            Token::Asterisk,
            Token::From,
            Token::Identifier("temperatures".to_string()),
            Token::Where,
            Token::Identifier("value".to_string()),
            Token::Operator(">".to_string()),
            Token::Number(98.6)
        ]);
    }

    #[test]
    fn test_case_insesitive() {
        let sql = "select * from users WHERE age > 18";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select,
            Token::Asterisk,
            Token::From,
            Token::Identifier("users".to_string()),
            Token::Where, 
            Token::Identifier("age".to_string()),
            Token::Operator(">".to_string()),
            Token::Number(18.0)
        ]);
    }

    #[test]
    fn test_nested() {
        let sql = "SELECT * FROM (SELECT id FROM users)";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Select,
            Token::Asterisk,
            Token::From,
            Token::LeftParen,
            Token::Select,
            Token::Identifier("id".to_string()),
            Token::From,
            Token::Identifier("users".to_string()),
            Token::RightParen
        ]);
    }

    #[test]
    fn test_insert_into() {
        let sql = "INSERT INTO users (name, age) VALUES ('John Doe', 30)";
        let tokens: Vec<Token> = Lexer::new(sql).collect();

        assert_eq!(tokens, vec![
            Token::Insert,
            Token::Into,
            Token::Identifier("users".to_string()),
            Token::LeftParen,
            Token::Identifier("name".to_string()),
            Token::Comma,
            Token::Identifier("age".to_string()),
            Token::RightParen,
            Token::Values,
            Token::LeftParen,
            Token::String("John Doe".to_string()),
            Token::Comma,
            Token::Number(30.0),
            Token::RightParen,
        ]);
    }

    #[test]
    fn test_delete() {
        let sql = "DELETE FROM tables WHERE name = 'Andrew'";
        let tokens: Vec<Token> = Lexer::new(sql).collect();

        assert_eq!(tokens, vec![
            Token::Delete,
            Token::From,
            Token::Identifier("tables".to_string()),
            Token::Where, 
            Token::Identifier("name".to_string()),
            Token::Operator("=".to_string()),
            Token::String("Andrew".to_string())
        ]);
    }

    #[test]
    fn test_update() {
        let sql = "UPDATE users SET name = 'Andrew' WHERE id < 10";
        let tokens: Vec<Token> = Lexer::new(sql).collect();
        assert_eq!(tokens, vec![
            Token::Update,
            Token::Identifier("users".to_string()),
            Token::Set, 
            Token::Identifier("name".to_string()),
            Token::Operator("=".to_string()),
            Token::String("Andrew".to_string()),
            Token::Where, 
            Token::Identifier("id".to_string()),
            Token::Operator("<".to_string()),
            Token::Number(10.0)
        ]);
    }
}