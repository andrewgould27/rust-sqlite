use std::{
    iter::Peekable,
    str::Chars
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Select, 
    Insert,
    From,
    Where, 
    Identifier(String),
    Number(f64),
    String(String),
    Operator(String),
    Comma,
    Asterisk,
    LeftParen,
    RightParen,
    EOF
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable()
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some(ch) => match ch {
                ',' => Token::Comma, 
                '*' => Token::Asterisk,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '=' | '>' | '<' => {
                    let mut op = ch.to_string();
                    if let Some('=') = self.input.peek() {
                        op.push(self.input.next().unwrap());
                    }
                    Token::Operator(op)
                }
                '"' => {
                    let s = self.read_string();
                    Token::String(s)
                }
                '0'..='9' => {
                    let num = self.read_number(ch);
                    Token::Number(num)
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier(ch);
                    match ident.to_uppercase().as_str() {
                        "SELECT" => Token::Select,
                        "FROM" => Token::From, 
                        "WHERE" => Token::Where, 
                        _ => Token::Identifier(ident)
                    }
                }
                _ => panic!("Unexpected character: {ch}")
            },
            None => Token::EOF
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(ch) = self.input.next() {
            if ch == '"' {
                break;
            }
            s.push(ch);
        }
        s
    }

    fn read_number(&mut self, first_digit: char) -> f64 {
        let mut num = String::from(first_digit);
        while let Some(&ch) = self.input.peek() {
            if !ch.is_digit(10) && ch != '.' {
                break;
            }
            num.push(self.input.next().unwrap());
        }
        num.parse().unwrap()
    }

    fn read_identifier(&mut self, first_char: char) -> String {
        let mut ident = String::from(first_char);
        while let Some(&ch) = self.input.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            ident.push(self.input.next().unwrap());
        }
        ident 
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token; 

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token == Token::EOF {
            None 
        } else {
            Some(token)
        }
    }
}

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
}