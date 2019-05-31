
use crate::source;

#[derive(Clone)]
pub enum Kind {
    // General
    Identifier,
    Comment, 

    // Keywords
    FuncDef,

    // Operators
    LeftParenthesis,
    RightParenthesis,
    Addition, 
    Substraction,
    Comma, 
    Increment,
    Decrement, 
    //Literals
    IntLiteral
}

#[derive(Clone)]
pub enum Value {
    Integer(i64),
    String(std::string::String),
    None,
}

#[derive(Clone)]
pub struct Token<T: source::Pointer> {
    pub kind: Kind,
    pub value: Value,
    pub beg: T,
    pub end: T,
} 