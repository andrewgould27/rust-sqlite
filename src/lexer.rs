use std::{
    iter::Peekable,
    str::Chars
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Select, 
    Insert,
    Delete, 
    Update,
    Into,
    Set,
    Values,
    From,
    Where, 
    Order, 
    By, 
    Identifier(String),
    Number(f64),
    String(String),
    Operator(String),
    Comma,
    Asterisk,
    LeftParen,
    RightParen,
    Asc, 
    Desc,
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
                    if let Some(&next_ch) = self.input.peek() {
                        if (ch == '<' && next_ch == '=') || (ch == '>' && next_ch == '=') || (ch == '<' && next_ch == '>') {
                            op.push(self.input.next().unwrap());
                        }
                    }
                    Token::Operator(op)
                }
                '\'' | '"' => {
                    let s = self.read_string(ch);
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
                        "INSERT" => Token::Insert,
                        "UPDATE" => Token::Update, 
                        "DELETE" => Token::Delete, 
                        "ORDER" => Token::Order, 
                        "BY" => Token::By, 
                        "ASC" => Token::Asc, 
                        "DESC" => Token::Desc, 
                        "SET" => Token::Set, 
                        "FROM" => Token::From, 
                        "WHERE" => Token::Where, 
                        "INTO" => Token::Into, 
                        "VALUES" => Token::Values,
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

    fn read_string(&mut self, quote: char) -> String {
        let mut s = String::new();
        while let Some(ch) = self.input.next() {
            if ch == quote {
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

