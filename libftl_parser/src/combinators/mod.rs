use ftl_lexer::token;
use ftl_source::{Pointer, Source};

use crate::{PRes, Parser};

pub mod concrete;
pub mod utility;

use concrete::{
    MapComb, OrAndThenMapComb, OrComb, TryComb, TryFailMsgErrorParser, TryFailUnexpectedErrParser,
};

pub trait Combinator<'a, S: 'static + Source, R>: Sized {
    fn run_chain(self) -> (&'a mut Parser<S>, R);

    fn run(self) -> R {
        let (_, res) = self.run_chain();
        res
    }
}

pub trait ResultCombinator<'a, S, P, R>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
{
    fn fail_unex_tok(
        self,
        kind: token::Kind,
        val: token::Value,
        msg: String,
    ) -> TryFailUnexpectedErrParser<'a, S, R, Self>
    where
        Self: Combinator<'a, S, PRes<R, P>> + Sized,
    {
        TryFailUnexpectedErrParser::chain(self, kind, val, msg)
    }

    fn fail_msg(self, msg: String) -> TryFailMsgErrorParser<'a, S, R, Self>
    where
        Self: Combinator<'a, S, PRes<R, P>> + Sized,
    {
        TryFailMsgErrorParser::chain(self, msg)
    }

    fn or<F>(self, meth: F) -> OrComb<'a, S, R, Self, F>
    where
        Self: Combinator<'a, S, PRes<R, P>> + Sized,
        F: FnOnce(&mut Parser<S>) -> PRes<R, P>,
    {
        OrComb::chain(self, meth)
    }

    fn or_and_map<F, M, R2>(self, meth: F, mapper: M) -> OrAndThenMapComb<'a, S, Self, R2, R, F, M>
    where
        Self: Combinator<'a, S, PRes<R, P>> + Sized,
        F: FnOnce(&mut Parser<S>) -> PRes<R2, P>,
        M: FnOnce(R2) -> R,
    {
        OrAndThenMapComb::chain(self, meth, mapper)
    }
}

impl<'a, S, P, R, C> ResultCombinator<'a, S, P, R> for C
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
    C: Combinator<'a, S, PRes<R, P>>,
{
}

pub trait PlainCombinator<'a, S: 'static + Source, R> {
    fn map<R2, F>(self, mapper: F) -> MapComb<'a, S, R, R2, Self, F>
    where
        Self: Combinator<'a, S, R>,
        F: FnOnce(R) -> R2,
    {
        MapComb::chain(self, mapper)
    }
}

impl<'a, S, R, C> PlainCombinator<'a, S, R> for C
where
    S: 'static + Source,
    C: Combinator<'a, S, R>,
{
}

pub struct Comb<'a, S: 'static + Source>(pub &'a mut Parser<S>);

impl<'a, S, P> Comb<'a, S>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
{
    pub fn r#try<R, F>(self, meth: F) -> impl Combinator<'a, S, PRes<R, P>>
    where
        F: FnOnce(&mut Parser<S>) -> PRes<R, P>,
    {
        TryComb::chain(self.0, meth)
    }
}
