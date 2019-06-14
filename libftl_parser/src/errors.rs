use ftl_error::LangError;
use ftl_lexer::token::Token;
use ftl_source::{
    Source,
    Pointer,
};

pub enum ParserError<Ptr: Pointer> {
    UnexpectedToken{
        expected: Token<Ptr>,
        actual: Token<Ptr>,
    },
    TokenExpected(Token<Ptr>),
}

impl<Ptr: Pointer> LangError for ParserError<Ptr> {

    type Ptr = Ptr;

    fn desc(&self) -> &str {
        match *self {
            ParserError::UnexpectedToken{ .. } => "Unexpected token",
            ParserError::TokenExpected(ref tok) => "Token expected",
        }
    }

    fn begin(&self) -> &Self::Ptr {
        match *self {
            ParserError::UnexpectedToken{ actual: ref tok, .. } => &tok.span.beg,
            ParserError::TokenExpected(ref tok) => &tok.span.beg,
        }
    }

    fn end(&self) -> &Self::Ptr {
        match *self {
            ParserError::UnexpectedToken{ actual: ref tok, .. } => &tok.span.end,
            ParserError::TokenExpected(ref tok) => &tok.span.end,
        }
    }

}