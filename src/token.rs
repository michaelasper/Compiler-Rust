
pub enum TokenType {
    Operator,
    Delimeter,
    Reserved,
    Idtok,
    Stringtok,
    Numbertok,
}

pub enum DataType {
    
    Integer,
    Real,
    Stringtype,
    Booltype,
    Pointer,

}

pub struct Token {
    tokentype:  TokenType, 
    datatype:   DataType,
}

impl Token {

    pub fn new(tokentype: TokenType, datatype: DataType) -> Token {

        Token {
            tokentype: tokentype,
            datatype: datatype,
        }
    }
}    
