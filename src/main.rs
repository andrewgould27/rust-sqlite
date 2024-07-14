fn main() {
    let query = "SELECT * FROM users WHERE age > 18";
    let lexer = Lexer::new(sql);
    let tokens = lexer.tokenize();

    let parser = Parser::new(tokens);
    let ast = parser.parse();

    let interpreter = Interpreter::new();
    let result = interpreter.execute();

    println!("{:?}", result);
}
