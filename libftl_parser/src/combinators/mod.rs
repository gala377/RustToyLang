use ftl_lexer::token;
use ftl_source::Source;

use crate::{PRes, Parser};

pub mod concrete;
pub mod utility;

use concrete::{MapComb, OrComb, TryComb, TryFailMsgErrorParser, TryFailUnexpectedErrParser};
use utility::pres_lift_fn;

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
        msg: String,
    ) -> TryFailUnexpectedErrParser<'a, S, R, Self>
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
}

impl<'a, S, R, C> ResultCombinator<'a, S, R> for C
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
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

impl<'a, S: 'static + Source> Comb<'a, S> {
    pub fn r#try<R, F>(self, meth: F) -> impl Combinator<'a, S, PRes<R, S::Pointer>>
    where
        F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>,
    {
        TryComb::chain(self.0, meth)
    }
}
