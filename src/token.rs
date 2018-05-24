#[derive(Debug)]
pub enum TokenType {
    Operator,
    Delimeter,
    Reserved,
    IDToken,
    StringToken,
    NumberToken,
}

#[derive(Debug)]
pub enum DataType {
    Integer,
    Real,
    StringType,
    BoolType,
    Pointer,
}

#[derive(Debug)]
pub enum TokenData {
    TokenString(String),
    Which(i32),
    IntNum(i32),
    RealNum(f32),
}

#[derive(Debug)]
pub struct Token {
    pub tokentype: Option<TokenType>,
    pub datatype: Option<DataType>,
    pub tokenval: Option<TokenData>,
}

impl Token {
    pub fn new() -> Token {

        Token {
            tokentype: None,
            datatype: None,
            tokenval: None,
        }
    }
}
