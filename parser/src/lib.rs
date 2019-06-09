use std::cell;

use lexer;
use source; 
use error;

pub mod combinators;
pub mod ast;
pub mod nodes;
pub mod parsers;

pub type Parser<'src, 'err, S> = Fn(&mut lexer::Lexer<'src, 'err, S>, &cell::RefCell<error::Handler<S>>) -> ParseRes;
pub type ParseRes = Option<Box<dyn ast::Node>>;
