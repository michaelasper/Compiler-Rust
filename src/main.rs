mod lexer;
mod token;


fn main() {

    let mut token = lexer::identifier();
    println!("{:?}", token);
}
