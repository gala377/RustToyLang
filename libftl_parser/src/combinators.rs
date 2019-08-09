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

pub type Meth<'a, S: Source, R> = FnMut(&'a mut Parser<S>) -> PRes<R, S::Pointer>;

pub struct Comb<'a, S: Source>(pub &'a mut Parser<S>);

impl<'a, S: Source> Comb<'a, S> {

    pub fn r#try<R>(self, meth: &'a mut Meth<'a, S, R>) -> TryUninitComb<'a, S, R> {
        TryUninitComb(self.0, meth)
    }
}

pub struct TryUninitComb<'a, S: Source, R>(&'a mut Parser<S>, &'a mut Meth<'a, S, R>);

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
    meth: &'a mut Meth<'a, S, R>,
    kind: &'b token::Kind,
    val: token::Value,
    msg: String
}

impl<'a, 'b, S: Source, R> TryFailParser<'a, 'b, S, R> where S::Pointer: 'static {

    pub fn run(self) -> R {
        let beg = self.parser.curr_ptr();
        (self.meth)(self.parser).unwrap_or_else(
            |err| match err {
                ParseErr::EOF => self.parser.eof_reached_fatal(beg, self.parser.curr_ptr()),
                ParseErr::NotThisItem(tok) => {
                    self.parser.fatal(Parser::<S>::unexpected_token_err(
                        self.kind.clone(),
                        self.val, 
                        tok, 
                        self.msg));
                }  
            }
        )
    }
}

pub struct TryRecParser;
