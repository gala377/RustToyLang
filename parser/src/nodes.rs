use crate::ast;
use lexer::token;

pub struct IntLiteral {
    pub value: i64,
}

impl ast::Node for IntLiteral {}

pub struct Identifier {
    pub value: String,
}

impl ast::Node for Identifier {}


pub struct Token {
    pub token: token::Token,
}

impl ast::Node for Token {}