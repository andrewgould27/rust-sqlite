use rust_sqlite::{
    lexer::Lexer,
    lexer::Token,
    parser::Parser
};

fn main() {
    let query = "SELECT id, name FROM users";

    let lexer = Lexer::new(query);
    let mut parser = Parser::new(lexer);

    println!("Passed!");

    // let parser = Parser::new(tokens);
    // let ast = parser.parse();

    // let interpreter = Interpreter::new();
    // let result = interpreter.execute();

    // println!("{:?}", result);
}
