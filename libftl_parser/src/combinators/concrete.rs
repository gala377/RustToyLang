use std::marker::PhantomData;

use ftl_source::Source;
use ftl_lexer::token;

use crate::{
    ParseErr,
    Parser,
};

use super::*;

pub struct TryFailUnexpectedErrParser<'a, S, R, C> where
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>>
{
    prev: C,
    kind: token::Kind,
    val: token::Value,
    msg: String,

    _s: PhantomData<&'a S>,
    _r: PhantomData<R>,
}

impl<'a, S, R, C> TryFailUnexpectedErrParser<'a, S, R, C> where 
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>> 
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

impl<'a, S, R, C> Combinator<'a, S, R> for TryFailUnexpectedErrParser<'a, S, R, C> where
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>>
{
    fn run_chain(self) -> (&'a mut Parser<S>, R) {
        let Self{prev, kind, val, msg, ..} = self;
        let (parser, res) = prev.run_chain();
        let res = res.unwrap_or_else(
            |err| {
                let beg = parser.pop_ptr();                
                match err {
                    ParseErr::EOF => parser.eof_reached_fatal(beg, parser.curr_ptr()),
                    ParseErr::NotThisItem(tok) => {
                        parser.fatal(Parser::<S>::unexpected_token_err(
                            kind.clone(), val, tok, msg));
                    }  
                }
            }
        );
        (parser, res)
    }
}



pub struct TryFailMsgErrorParser<'a, S, R, C> where 
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>>
{
    prev: C,
    msg: String,

    _s: PhantomData<&'a S>,
    _r: PhantomData<R>,
}

impl <'a, S, R, C> TryFailMsgErrorParser<'a, S, R, C> where
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>> 
{
    pub fn chain(prev: C, msg: String) -> Self {
        Self{prev, msg, _r: PhantomData, _s: PhantomData}
    }
}

impl <'a, S, R, C> Combinator<'a, S, R> for TryFailMsgErrorParser<'a, S, R, C> where
        S: 'static + Source,
        C: Combinator<'a, S, PRes<R, S::Pointer>>
{
    fn run_chain(self) -> (&'a mut Parser<S>, R) {
        let Self{prev, msg, ..} = self;
        let (parser, res) = prev.run_chain();
        let res = res.unwrap_or_else(
            |err| {
                let beg = parser.pop_ptr();
                match err {
                    ParseErr::EOF => parser.eof_reached_fatal(beg, parser.curr_ptr()),
                    ParseErr::NotThisItem(_) => 
                        parser.fatal(Parser::<S>::msg_err(
                            msg,
                            beg,
                            parser.curr_ptr()
                        )),
                }
            }
        );
        (parser, res)
    }
}


// pub struct TryOrUninitComb<'a, S, R1, R2, C> where
//         S: Source,
//         C: Combinator<'a, S, PRes<R1, S::Pointer>> 
// {
//     prev_comb: C,
//     fallback: &'a Meth<S, R2>,
// }