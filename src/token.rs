
#[derive(Debug)]
pub enum TokenType {
    Operator,
    Delimeter,
    Reserved,
    IDToken,
    StringToken,
    NumberToken,
    None,
}

#[derive(Debug)]
pub enum DataType {
    Integer,
    Real,
    StringType,
    BoolType,
    Pointer,
    None,
}

#[derive(Debug)]
pub enum TokenData {
    TokenString(String),
    Which(i32),
    IntNum(i32),
    RealNum(f32),
    None,
}

#[derive(Debug)]
pub struct Token {
    pub tokentype: TokenType,
    pub datatype: DataType,
    pub tokenval: TokenData,
}

impl Token {
    pub fn new() -> Token {

        Token {
            tokentype: TokenType::None,
            datatype: DataType::None,
            tokenval: TokenData::None,
        }
    }
}
