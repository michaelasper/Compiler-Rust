mod token;
// mod lexer;

fn main() {
    println!("Hello, world!");

    let mut token = token::Token::new();
    token.datatype = Some(token::DataType::Integer);
    println!("{:?}", token);
}
