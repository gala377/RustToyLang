use ftl_source::Source;
use ftl_lexer::token;

use crate::{
    PRes,
    Parser,
};


pub mod concrete;

use concrete::*;


#[allow(type_alias_bounds)]
pub type Meth<S: 'static + Source, R> = dyn FnMut(&mut Parser<S>) -> PRes<R, S::Pointer>;


pub trait Combinator<'a, S: 'static + Source, R>: Sized {

    fn run_chain(self) -> (&'a mut Parser<S>, R);

    fn run(self) -> R {
        let (_, res) = self.run_chain();
        res 
    }
}


pub trait ResultCombinator<'a, S: 'static + Source, R> {
    
    fn fail_unex_tok(
    self,
    kind: token::Kind,
    val: token::Value,
    msg: String) -> TryFailUnexpectedErrParser<'a, S, R, Self> 
        where 
            Self: Combinator<'a, S, PRes<R, S::Pointer>> + Sized,
    {    
        TryFailUnexpectedErrParser::chain(self, kind, val, msg)
    }

    fn fail_msg(self, msg: String) -> TryFailMsgErrorParser<'a, S, R, Self> 
    where 
        Self: Combinator<'a, S, PRes<R, S::Pointer>> + Sized, 
    {
        TryFailMsgErrorParser::chain(self, msg)
    }

    fn or(self, meth: &'a mut Meth<S, R>) -> OrComb<'a, S, R, Self> 
        where
            Self: Combinator<'a, S, PRes<R, S::Pointer>> + Sized,
    {
        OrComb::chain(self, meth)
    }
}

impl<'a, S, R, C> ResultCombinator<'a, S, R> for C where 
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>> {}



pub struct Comb<'a, S: 'static + Source>(pub &'a mut Parser<S>);

impl<'a, S: 'static + Source> Comb<'a, S> {
    pub fn r#try<R>(self, meth: &'a mut Meth<S, R>) -> impl Combinator<'a, S, PRes<R, S::Pointer>> {
        TryComb(self.0, meth)
    }
}



pub struct TryComb<'a, S: 'static + Source, R>(&'a mut Parser<S>, &'a mut Meth<S, R>);

impl<'a, S, R> Combinator<'a, S, PRes<R, S::Pointer>> for TryComb<'a, S, R> where S: 'static + Source {
    fn run_chain(self) -> (&'a mut Parser<S>, PRes<R, S::Pointer>) {
        let res = self.1(self.0);
        (self.0, res)
    }
} 