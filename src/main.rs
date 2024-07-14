use rust_sqlite::{
    lexer::Lexer,
    lexer::Token
};

fn main() {
    let query = "SELECT id, name FROM users";

    let tokens: Vec<Token> = Lexer::new(query).collect();
    assert_eq!(tokens, vec![
        Token::Select,
        Token::Identifier("id".to_string()),
        Token::Comma,
        Token::Identifier("name".to_string()),
        Token::From, 
        Token::Identifier("users".to_string())
    ]);

    println!("Passed!");

    // let lexer = Lexer::new(query);
    // let tokens = lexer.tokenize();

    // let parser = Parser::new(tokens);
    // let ast = parser.parse();

    // let interpreter = Interpreter::new();
    // let result = interpreter.execute();

    // println!("{:?}", result);
}
