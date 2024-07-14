enum Token {
    Select, 
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

