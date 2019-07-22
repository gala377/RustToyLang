use std::fmt;
use std::fmt::Display;

use ftl_source;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    // General
    Identifier,
    InfixIdent,
    Comment, 
    Operator,

    // Keywords
    FuncDef,
    FuncDecl,
    InfixDef,

    // Operators
    LeftParenthesis,
    RightParenthesis,
    Comma, 
    Colon,
    At, 
    
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

impl<Ptr: ftl_source::Pointer> Display for Token<Ptr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "tok {{ kind: {:?} val: {:?}, span[{}:{} - {}:{}] }}",
            self.kind,
            self.value,
            self.span.beg.line(),
            self.span.beg.position(),
            self.span.end.line(),
            self.span.end.position())
    }
}