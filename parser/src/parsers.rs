use std::cell::RefCell;

use source::Source;
use lexer::Lexer;
use lexer::token::Token;
use lexer::token;
use error::Handler;


use crate::Parser;
use crate::ParseRes;
use crate::nodes;


pub fn parse_int_literal<'a, 'b, S: Source>(
    lexer: &mut Lexer<'a, 'b, S>,
    _handler: &RefCell<Handler<S>>) -> ParseRes 
    where S::Pointer: 'static {

    if let Some(Token{
        kind: token::Kind::IntLiteral,
        value: token::Value::Integer(value), 
        ..
    }) = lexer.curr() {
        return Some(Box::new(nodes::IntLiteral{value}));
    }
    None
}

pub fn parse_identifier<'a, 'b, S: Source>(
    lexer: &mut Lexer<'a, 'b, S>,
    _handler: &RefCell<Handler<S>>) -> ParseRes 
    where S::Pointer: 'static {

    if let Some(Token{
        kind: token::Kind::Identifier,
        value: token::Value::String(value), 
        ..
    }) = lexer.curr() {
        return Some(Box::new(nodes::Identifier{value}));
    }
    None
}
