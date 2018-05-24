mod token;
// mod lexer;

fn main() {
    println!("Hello, world!");

    let token = token::Token::new(token::TokenType::Operator, token::DataType::Integer);
}
