use ftl_source::Source;
use ftl_lexer::token;

use crate::{
    PRes,
    Parser,
};


pub mod concrete;

use concrete::*;


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

    fn or<F>(self, meth: F) -> OrComb<'a, S, R, Self, F>
        where
            Self: Combinator<'a, S, PRes<R, S::Pointer>> + Sized,
            F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>,
    {
        OrComb::chain(self, meth)
    }

    // fn then_map<R2, F1, F2>(self, mapper: F1) -> 
    //     MapComb<'a, S, PRes<R, S::Pointer>, PRes<R2, S::Pointer>, Self, F2> 
    //     where 
    //         Self: Combinator<'a, S, PRes<R, S::Pointer>> + Sized,
    //         F1: Fn(R) -> R2,
    //         F2: Fn(PRes<R, S::Pointer>) -> PRes<R2, S::Pointer>,
    // {
    //     let func = move |res: PRes<R, S::Pointer>| {
    //         res.and_then(move |val| Ok(mapper(val)))
    //     };
    //     MapComb::chain(self, func)
    // }

}

impl<'a, S, R, C> ResultCombinator<'a, S, R> for C
    where
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>> {}



pub trait PlainCombinator<'a, S: 'static + Source, R> {
    fn map<R2, F>(self, mapper: F) -> MapComb<'a, S, R, R2, Self, F>
        where
            Self: Combinator<'a, S, R>,
            F: Fn(R) -> R2,
    {
        MapComb::chain(self, mapper)
    }
}

impl<'a, S, R, C> PlainCombinator<'a, S, R> for C
    where
        S: 'static + Source,
        C: Combinator<'a, S, R>,
    {}



pub struct Comb<'a, S: 'static + Source>(pub &'a mut Parser<S>);

impl<'a, S: 'static + Source> Comb<'a, S> {
    pub fn r#try<R, F>(self, meth: F) -> impl Combinator<'a, S, PRes<R, S::Pointer>>
        where F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer> {
        TryComb(self.0, meth)
    }
}


pub struct TryComb<'a, S, R, F>(&'a mut Parser<S>, F) 
    where
        S: 'static + Source,
        F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>;

impl<'a, S, R, F> Combinator<'a, S, PRes<R, S::Pointer>> for TryComb<'a, S, R, F> 
    where 
        S: 'static + Source,
        F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>,
{
    fn run_chain(self) -> (&'a mut Parser<S>, PRes<R, S::Pointer>) {
        let Self(parser, func) = self;
        let res = func(parser);
        (parser, res)
    }
}