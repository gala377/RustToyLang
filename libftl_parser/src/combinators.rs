use std::fmt;
use std::fmt::Display;

use ftl_source::{
    Pointer,
    Source,
};
use ftl_lexer::token;

use crate::{
    PRes,
    ParseErr,
    Parser,
};

#[allow(type_alias_bounds)]
pub type Meth<S: Source, R> = FnMut(&mut Parser<S>) -> PRes<R, S::Pointer>;

pub struct Comb<'a, S: Source>(pub &'a mut Parser<S>);

impl<'a, S: Source> Comb<'a, S> {

    pub fn r#try<R>(self, meth: &'a mut Meth<S, R>) -> TryUninitComb<S, R> {
        TryUninitComb(self.0, meth)
    }
}

pub struct TryUninitComb<'a, S: Source, R>(&'a mut Parser<S>, &'a mut Meth<S, R>);

impl<'a, S: Source, R> TryUninitComb<'a, S, R> {

    pub fn fail<'b>(self,
        kind: &'b token::Kind,
        val: token::Value,
        msg: String) -> TryFailParser<'a, 'b, S, R> 
    {    
        TryFailParser{
            parser: self.0,
            meth: self.1,
            kind,
            val,
            msg,
        }
    }

}

pub struct TryFailParser<'a, 'b, S: Source, R>{
    parser: &'a mut Parser<S>,
    meth: &'a mut Meth<S, R>,
    kind: &'b token::Kind,
    val: token::Value,
    msg: String
}

impl<'a, 'b, S, R> TryFailParser<'a, 'b, S, R> where S: 'static + Source{

    pub fn run(self) -> R {
        let Self{parser, meth, kind, val, msg} = self;
        let beg = parser.curr_ptr();
        meth(parser).unwrap_or_else(
            |err| match err {
                ParseErr::EOF => parser.eof_reached_fatal(beg, parser.curr_ptr()),
                ParseErr::NotThisItem(tok) => {
                    parser.fatal(Parser::<S>::unexpected_token_err(
                        kind.clone(), val, tok, msg));
                }  
            }
        )
    }
}

pub struct TryRecParser;
