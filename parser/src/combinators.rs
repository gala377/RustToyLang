use std::cell::RefCell;

use source::Source;
use lexer::Lexer;
use lexer::token::Token;
use lexer::token;
use error::Handler;

use crate::Parser;
use crate::ParseRes;
use crate::nodes;

pub fn make_tok_parser(kind: token::Kind) -> Parser {
    move |lexer, _handler| {
        if let Some(Token{
            kind: kind,
            ..
        }) = lexer.curr() {
            return Some(Box::new());
        }
        None
    }
}