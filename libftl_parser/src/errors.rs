use ftl_error::LangError;
use ftl_lexer::token::Token;
use ftl_source::{
    Pointer,
    Span,
};


pub struct ParserError<Ptr: Pointer> {
    pub msg: String,
    pub kind: ParserErrorKind<Ptr>,
}

pub enum ParserErrorKind<Ptr: Pointer> {
    UnexpectedToken{
        expected: Token<Ptr>,
        actual: Token<Ptr>,
    },
    TokenExpected(Token<Ptr>),
    Msg(Span<Ptr>),
}

impl<Ptr: Pointer> LangError for ParserError<Ptr> {

    type Ptr = Ptr;

    fn desc(&self) -> String {
        self.msg.clone()
    }

    fn begin(&self) -> &Self::Ptr {
        match self.kind {
            ParserErrorKind::UnexpectedToken{ actual: ref tok, .. } => &tok.span.beg,
            ParserErrorKind::TokenExpected(ref tok) => &tok.span.beg,
            ParserErrorKind::Msg(ref span) => &span.beg,
        }
    }

    fn end(&self) -> &Self::Ptr {
        match self.kind {
            ParserErrorKind::UnexpectedToken{ actual: ref tok, .. } => &tok.span.end,
            ParserErrorKind::TokenExpected(ref tok) => &tok.span.end,
            ParserErrorKind::Msg(ref span) => &span.end,
        }
    }

}