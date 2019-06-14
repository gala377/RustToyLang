
use ftl_source;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    // Literals
    IntLiteral,

    // Special
    /// Poisoned token represents invalid token 
    Poisoned,
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(u64),
    String(std::string::String),
    None,
}

#[derive(Clone, Debug)]
pub struct Token<T: ftl_source::Pointer> {
    pub kind: Kind,
    pub value: Value,
    pub span: ftl_source::Span<T>,
} 