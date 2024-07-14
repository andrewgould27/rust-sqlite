use crate::lexer::{
    Lexer, 
    Token
};

use crate::ast::{
    ASTNode, 
    SelectStatement,
    InsertStatement,
    Condition,
    ComparisonOperator,
    Value
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer, 
            current_token: Token::EOF
        };

        parser.advance();
        parser 
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next().unwrap_or(Token::EOF);
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        match self.current_token {
            Token::Select => self.parse_select(),
            // Token::Insert => self.parse_insert(),
            _ => Err("Unexpected token".to_string())
        }
    }

    pub fn parse_select(&mut self) -> Result<ASTNode, String> {
        self.advance();

        let columns = self.parse_columns()?;

        if self.current_token != Token::From {
            return Err("Exepected FROM clause".to_string())
        }

        self.advance();

        let table = match self.current_token {
            Token::Identifier(ref name) => {
                let table_name = name.clone();
                self.advance();
                table_name 
            }
            _ => return Err("Expected table name".to_string())
        };

        let condition = if self.current_token == Token::Where {
            self.advance();
            Some(self.parse_condition()?)
        } else {
            None 
        };

        Ok(ASTNode::Select(SelectStatement {
            columns,
            table, 
            condition
        }))
    }

    // pub fn parse_insert(&mut self) -> Result<ASTNode, String> {
        // Gimme a minute
    // }

    pub fn parse_columns(&mut self) -> Result<Vec<String>, String> {
        let mut columns = Vec::new();

        loop {
            match self.current_token {
                Token::Identifier(ref name) => {
                    columns.push(name.clone());
                    self.advance();
                }
                Token::Asterisk => {
                    columns.push("*".to_string());
                    self.advance();
                    break;
                }
                _ => return Err("Expected column name or *".to_string())
            }

            match self.current_token {
                Token::Comma => self.advance(),
                _ => break
            }
        }

        Ok(columns)
    }

    pub fn parse_condition(&mut self) -> Result<Condition, String> {
        let column = match self.current_token {
            Token::Identifier(ref name) => {
                let column_name = name.clone();
                self.advance();
                column_name 
            }
            _ => return Err("Expected column name in condition".to_string())
        };

        let operator = match self.current_token {
            Token::Operator(ref op) => {
                let operator = match op.as_str() {
                    "=" => ComparisonOperator::Equals, 
                    "<>" => ComparisonOperator::NotEquals,
                    ">" => ComparisonOperator::GreaterThan,
                    "<" => ComparisonOperator::LessThan,
                    ">=" => ComparisonOperator::GreaterEqualThan,
                    "<=" => ComparisonOperator::LessEqualThan,
                    _ => return Err("Unsupported operator".to_string())
                };
                self.advance();
                operator 
            }
            _ => return Err("Expected operator in conditional".to_string())
        };

        let value = match self.current_token {
            Token::Number(n) => {
                self.advance();
                Value::Number(n)
            }
            Token::String(ref s) => {
                let string_val = s.clone();
                self.advance();
                Value::String(string_val)
            }
            _ => return Err("Expected value in conditional".to_string())
        };

        Ok(Condition::Comparison(column, operator, value))
    }
}