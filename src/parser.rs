use crate::lexer::{
    Lexer, 
    Token
};

use crate::ast::{
    ASTNode, 
    SelectStatement,
    InsertStatement,
    UpdateStatement,
    DeleteStatement,
    Condition,
    ComparisonOperator,
    Value,
    OrderByClause,
    OrderDirection
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
            Token::Insert => self.parse_insert(),
            Token::Delete => self.parse_delete(),
            Token::Update => self.parse_update(),
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

        let order_by = if self.current_token == Token::Order {
            self.parse_order_by()?
        } else {
            Vec::<OrderByClause>::new()
        };

        Ok(ASTNode::Select(SelectStatement {
            columns,
            table, 
            condition,
            order_by 
        }))
    }

    pub fn parse_insert(&mut self) -> Result<ASTNode, String> {
        self.advance();

        if self.current_token != Token::Into {
            return Err("Expected INTO after INSERT".to_string());
        }
        self.advance();

        let table = match &self.current_token {
            Token::Identifier(name) => {
                let table_name = name.clone();
                self.advance();
                table_name
            }
            _ => return Err("Expected identifier".to_string())
        };

        let columns = if self.current_token == Token::LeftParen {
            self.parse_column_list()?
        } else {
            Vec::new()
        };

        if self.current_token != Token::Values {
            return Err("Expected values keyword".to_string());
        }
        self.advance();

        let values = self.parse_value_list()?;

        if columns.len() > 0 && columns.len() != values.len() {
            return Err("Number of columns doesn't match number of values".to_string());
        }

        Ok(ASTNode::Insert(InsertStatement {
            table, 
            columns, 
            values, 
        }))
    }

    pub fn parse_update(&mut self) -> Result<ASTNode, String> {
        self.advance();

        let table = match &self.current_token {
            Token::Identifier(name) => {
                let table_name = name.clone();
                self.advance();
                table_name 
            }
            _ => return Err("Expected table name after UPDATE.".to_string())
        }; 

        if self.current_token != Token::Set {
            return Err("Expected SET after table name in UPDATE statement.".to_string());
        }
        self.advance();

        let updates = self.parse_update_list()?;

        let condition = if self.current_token == Token::Where {
            self.advance();
            Some(self.parse_condition()?)
        } else {
            None 
        };

        Ok(ASTNode::Update(UpdateStatement {
            table,
            updates,
            condition 
        }))
    }

    pub fn parse_update_list(&mut self) -> Result<Vec<(String, Value)>, String> {
        let mut updates = Vec::new();

        loop {
            let column = match &self.current_token {
                Token::Identifier(name) => {
                    let column_name = name.clone();
                    self.advance();
                    column_name
                }
                _ => return Err("Expected column name in UPDATE statement.".to_string())
            };

            if self.current_token != Token::Operator(String::from("=")) {
                return Err("Expected '=' after column name in UPDATE statement.".to_string());
            }
            self.advance();

            let value = match &self.current_token {
                Token::Number(n) => {
                    let num = *n;
                    self.advance();
                    Value::Number(num)
                }
                Token::String(s) => {
                    let string_val = s.clone();
                    self.advance();
                    Value::String(string_val)
                }
                _ => return Err("Expected value in UPDATE statement.".to_string())
            };

            updates.push((column, value));

            if self.current_token != Token::Comma {
                break;
            }

            self.advance();
        }

        Ok(updates)
    }

    pub fn parse_delete(&mut self) -> Result<ASTNode, String> {
        self.advance();

        if self.current_token != Token::From {
            return Err("Expected FROM after DELETE".to_string());
        }
        self.advance();

        let table = match &self.current_token {
            Token::Identifier(name) => {
                let table_name = name.clone();
                self.advance();
                table_name
            }
            _ => return Err("Expected table name after FROM".to_string())
        };

        let condition = if self.current_token == Token::Where {
            self.advance();
            Some(self.parse_condition()?)
        } else {
            None 
        };

        Ok(
            ASTNode::Delete(
                DeleteStatement { 
                    table, 
                    condition 
                }
            )
        )
    }

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

    pub fn parse_column_list(&mut self) -> Result<Vec<String>, String> {
        self.advance();
        let mut columns: Vec<String> = Vec::new();

        loop {
            match &self.current_token {
                Token::Identifier(name) => {
                    columns.push(name.clone());
                    self.advance();
                }
                _ => return Err("Expected column name".to_string())
            }

            match self.current_token {
                Token::Comma => {
                    self.advance();
                    continue;
                }
                Token::RightParen => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected comma or right parens".to_string())
            }
        }

        Ok(columns)
    }

    pub fn parse_value_list(&mut self) -> Result<Vec<Value>, String> {
        if self.current_token != Token::LeftParen {
            return Err("Expected left parens before values".to_string());
        }
        self.advance();

        let mut values = Vec::new();

        loop {
            match &self.current_token {
                Token::Number(n) => {
                    values.push(Value::Number(*n));
                    self.advance();
                }
                Token::String(s) => {
                    values.push(Value::String(s.clone()));
                    self.advance();
                }
                _ => return Err("Expected value".to_string())
            }

            match self.current_token {
                Token::Comma => {
                    self.advance();
                    continue;
                }
                Token::RightParen => {
                    self.advance();
                    break;
                }
                _ => return Err("Expected comma or right parens".to_string())
            }
        }

        Ok(values)
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

    fn parse_order_by(&mut self) -> Result<Vec<OrderByClause>, String> {
        self.advance();

        if self.current_token != Token::By {
            return Err("Expected BY after ORDER.".to_string());
        }
        self.advance();

        let mut clauses = Vec::new();
        loop {
            let column = match &self.current_token {
                Token::Identifier(name) => {
                    let col_name = name.clone();
                    self.advance();
                    col_name 
                }
                _ => return Err("Expected column name in ORDER BY clause.".to_string())
            };

            let order = if self.current_token == Token::Asc {
                self.advance();
                OrderDirection::Asc 
            } else if self.current_token == Token::Desc {
                self.advance();
                OrderDirection::Desc 
            } else {
                OrderDirection::Asc
            };

            clauses.push(OrderByClause { column, order });

            if self.current_token != Token::Comma {
                break;
            }
            self.advance();
        };

        Ok(clauses)
    }
}