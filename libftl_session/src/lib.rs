//! Defines compilation session as
//! well as related types.

use std::io;
use std::io::Write;

use ftl_error::{Handler, LangError};
use ftl_source::{Pointer, Source};
use ftl_utility::RcRef;

pub struct Session<S: Source> {
    pub handler: Handler<S>,
    pub src: RcRef<S>,
}

impl<S, P> Session<S>
where
    P: Pointer,
    S: Source<Pointer = P>,
{
    pub fn new(src: S) -> Self {
        let src = RcRef::new(src);
        Session {
            src: src.clone(),
            handler: Handler::new(src.clone()),
        }
    }

    pub fn err(&mut self, err: Box<dyn LangError<Ptr = P>>) {
        self.handler.err(err);
    }

    pub fn fatal(&mut self, err: Box<dyn LangError<Ptr = P>>) -> ! {
        self.handler.fatal(err);
    }

    pub fn emit_err(&self, buff: &mut impl Write) -> io::Result<usize> {
        match self.handler.error_msg() {
            None => Ok(0),
            Some(content) => buff.write(content.as_bytes()),
        }
    }
}

pub struct Emitter<S: Source> {
    sess: RcRef<Session<S>>,
}

impl<S: Source> Emitter<S> {
    pub fn new(sess: RcRef<Session<S>>) -> Self {
        Self { sess }
    }

    pub fn emit_err(&self, buff: &mut impl Write) -> io::Result<usize> {
        self.sess.borrow().emit_err(buff)
    }
}
