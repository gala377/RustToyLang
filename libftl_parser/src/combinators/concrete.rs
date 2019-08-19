use std::marker::PhantomData;

use ftl_lexer::token;
use ftl_source::Source;

use crate::{ParseErr, Parser};

use super::*;

pub struct TryComb<'a, S, R, F>(&'a mut Parser<S>, F)
where
    S: 'static + Source,
    F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>;

impl<'a, S, P, R, F> TryComb<'a, S, R, F>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
    F: FnOnce(&mut Parser<S>) -> PRes<R, P>,
{
    pub fn chain(parser: &'a mut Parser<S>, meth: F) -> Self {
        TryComb(parser, meth)
    }
}

impl<'a, S, P, R, F> Combinator<'a, S, PRes<R, P>> for TryComb<'a, S, R, F>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
    F: FnOnce(&mut Parser<S>) -> PRes<R, P>,
{
    fn run_chain(self) -> (&'a mut Parser<S>, PRes<R, P>) {
        let Self(parser, func) = self;
        let res = func(parser);
        (parser, res)
    }
}

pub struct TryFailUnexpectedErrParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    prev: C,
    kind: token::Kind,
    val: token::Value,
    msg: String,

    _s: PhantomData<&'a S>,
    _r: PhantomData<R>,
}

impl<'a, S, R, C> TryFailUnexpectedErrParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    pub fn chain(prev: C, kind: token::Kind, val: token::Value, msg: String) -> Self {
        Self {
            prev,
            kind,
            val,
            msg,

            _r: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<'a, S, R, C> Combinator<'a, S, R> for TryFailUnexpectedErrParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    fn run_chain(self) -> (&'a mut Parser<S>, R) {
        let Self {
            prev,
            kind,
            val,
            msg,
            ..
        } = self;
        let (parser, res) = prev.run_chain();
        let res = res.unwrap_or_else(|err| {
            let beg = parser.pop_ptr();
            match err {
                ParseErr::EOF => parser.eof_reached_fatal(beg, parser.curr_ptr()),
                ParseErr::NotThisItem(tok) => {
                    parser.fatal(Parser::<S>::unexpected_token_err(
                        kind.clone(),
                        val,
                        tok,
                        msg,
                    ));
                }
            }
        });
        (parser, res)
    }
}

pub struct TryFailMsgErrorParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    prev: C,
    msg: String,

    _s: PhantomData<&'a S>,
    _r: PhantomData<R>,
}

impl<'a, S, R, C> TryFailMsgErrorParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    pub fn chain(prev: C, msg: String) -> Self {
        Self {
            prev,
            msg,
            _r: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<'a, S, R, C> Combinator<'a, S, R> for TryFailMsgErrorParser<'a, S, R, C>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
{
    fn run_chain(self) -> (&'a mut Parser<S>, R) {
        let Self { prev, msg, .. } = self;
        let (parser, res) = prev.run_chain();
        let res = res.unwrap_or_else(|err| {
            let beg = parser.pop_ptr();
            match err {
                ParseErr::EOF => parser.eof_reached_fatal(beg, parser.curr_ptr()),
                ParseErr::NotThisItem(_) => {
                    parser.fatal(Parser::<S>::msg_err(msg, beg, parser.curr_ptr()))
                }
            }
        });
        (parser, res)
    }
}

pub struct OrComb<'a, S, R, C, F>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>,
{
    prev_comb: C,
    fallback: F,

    _r: PhantomData<R>,
    _s: PhantomData<&'a S>,
}

impl<'a, S, R, C, F> OrComb<'a, S, R, C, F>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R, S::Pointer>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R, S::Pointer>,
{
    pub fn chain(prev_comb: C, fallback: F) -> Self {
        Self {
            prev_comb,
            fallback,

            _r: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<'a, S, P, R, C, F> Combinator<'a, S, PRes<R, P>> for OrComb<'a, S, R, C, F>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
    C: Combinator<'a, S, PRes<R, P>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R, P>,
{
    fn run_chain(self) -> (&'a mut Parser<S>, PRes<R, P>) {
        let Self {
            prev_comb,
            fallback,
            ..
        } = self;
        let (parser, res) = prev_comb.run_chain();
        let res = res.or_else(|_| fallback(parser));
        (parser, res)
    }
}

#[allow(dead_code)]
pub struct MapComb<'a, S, R1, R2, C, F>
where
    S: 'static + Source,
    C: Combinator<'a, S, R1>,
    F: FnOnce(R1) -> R2,
{
    prev: C,
    mapper: F,

    _s: PhantomData<&'a S>,
    _r1: PhantomData<R1>,
    _r2: PhantomData<R2>,
}

impl<'a, S, R1, R2, C, F> MapComb<'a, S, R1, R2, C, F>
where
    S: 'static + Source,
    C: Combinator<'a, S, R1>,
    F: FnOnce(R1) -> R2,
{
    #[allow(dead_code)]
    pub fn chain(prev: C, mapper: F) -> Self {
        Self {
            prev,
            mapper,
            _s: PhantomData,
            _r1: PhantomData,
            _r2: PhantomData,
        }
    }
}

impl<'a, S, R1, R2, C, F> Combinator<'a, S, R2> for MapComb<'a, S, R1, R2, C, F>
where
    S: 'static + Source,
    C: Combinator<'a, S, R1>,
    F: FnOnce(R1) -> R2,
{
    fn run_chain(self) -> (&'a mut Parser<S>, R2) {
        let Self { prev, mapper, .. } = self;
        let (parser, res) = prev.run_chain();
        (parser, mapper(res))
    }
}

#[allow(dead_code)]
pub struct OrAndThenMapComb<'a, S, C, R1, R2, F, M>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R2, S::Pointer>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R1, S::Pointer>,
    M: FnOnce(R1) -> R2,
{
    prev: C,
    meth: F,
    mapper: M,

    _r1: PhantomData<R1>,
    _r2: PhantomData<R2>,
    _s: PhantomData<&'a S>,
}

impl<'a, S, C, R1, R2, F, M> OrAndThenMapComb<'a, S, C, R1, R2, F, M>
where
    S: 'static + Source,
    C: Combinator<'a, S, PRes<R2, S::Pointer>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R1, S::Pointer>,
    M: FnOnce(R1) -> R2,
{
    #[allow(dead_code)]
    pub fn chain(prev: C, meth: F, mapper: M) -> Self {
        Self {
            prev,
            meth,
            mapper,
            _r1: PhantomData,
            _r2: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<'a, S, P, C, R1, R2, F, M> Combinator<'a, S, PRes<R2, P>>
    for OrAndThenMapComb<'a, S, C, R1, R2, F, M>
where
    P: Pointer,
    S: 'static + Source<Pointer = P>,
    C: Combinator<'a, S, PRes<R2, P>>,
    F: FnOnce(&mut Parser<S>) -> PRes<R1, P>,
    M: FnOnce(R1) -> R2,
{
    fn run_chain(self) -> (&'a mut Parser<S>, PRes<R2, P>) {
        let Self {
            prev, meth, mapper, ..
        } = self;
        let (parser, res) = prev.run_chain();
        let res = res.or_else(|_| meth(parser).and_then(move |val| Ok(mapper(val))));
        (parser, res)
    }
}
