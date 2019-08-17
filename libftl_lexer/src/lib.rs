use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

use log::trace;

use ftl_error;
use ftl_error::LangError;
use ftl_session::Session;
use ftl_source;
use ftl_source::{Source, Span};
use ftl_utility::RcRef;

mod helpers;
pub mod token;

// Lexer

pub type Token<T> = token::Token<<T as Source>::Pointer>;

pub struct Lexer<S: Source> {
    session: RcRef<Session<S>>,

    /// Just ref to src so we don't have to write
    /// session.borrow().src.borrow_mut() nad so on.
    src: RcRef<S>,

    curr_token: Option<Token<S>>,
}

impl<S> Lexer<S>
where
    S: Source,
    S::Pointer: 'static,
{
    pub fn new(session: RcRef<Session<S>>) -> Self {
        let mut s = Self {
            session: session.clone(),
            src: session.borrow().src.clone(),
            curr_token: None,
        };
        s.next();
        return s;
    }

    pub fn curr(&self) -> Option<Token<S>> {
        self.curr_token.clone()
    }

    pub fn next(&mut self) -> Option<Token<S>> {
        trace!("next(): Getting curr char from src");
        let mut opt = self.curr_char();
        while let Some(ch) = opt {
            trace!("next(): Its Some(_)");
            if ch == '#' || ch.is_whitespace() || ch == '\n' {
                trace!("next(): Whitespace or newline, skipping");
                self.skip_whitespaces();
                self.skip_comments();
                trace!("next(): Skipped, taking next char");
                opt = self.curr_char();
            } else {
                trace!("next(): Valid character");
                break;
            }
        }
        let tok = match opt {
            Some(ch) if ch.is_digit(10) => self.collect_integer(),
            Some(ch) if helpers::is_beg_of_ident(ch) => self.collect_identifier(),
            Some(ch) if helpers::is_part_of_op(ch) => self.collect_operator(),
            Some(ch) if ch == '`' => self.collect_infix(),
            Some(ch) if helpers::is_part_of_parenthesis(ch) => self.collect_parenthesis(),
            Some(_) => self.collect_char(),
            _ => None,
        };
        self.curr_token = tok;
        trace!("next(): Returning token");
        return self.curr();
    }

    fn curr_char(&self) -> Option<char> {
        self.src.borrow().curr_char()
    }

    fn next_char(&mut self) -> Option<char> {
        self.src.borrow_mut().next_char()
    }

    pub fn curr_ptr(&self) -> S::Pointer {
        self.src.borrow().curr_ptr()
    }

    fn skip_whitespaces(&mut self) {
        let mut opt = self.curr_char();
        while let Some(ch) = opt {
            if !(ch.is_whitespace() || ch == '\n') {
                break;
            }
            opt = self.next_char();
        }
    }

    fn skip_comments(&mut self) {
        if let Some('#') = self.curr_char() {
            while let Some(ch) = self.next_char() {
                if ch == '\n' {
                    self.next_char();
                    break;
                }
            }
        }
    }

    fn collect_integer(&mut self) -> Option<Token<S>> {
        trace!("collect_integer(): collecting integer");
        if let Some('0') = self.curr_char() {
            trace!("collect_integer(): It's 0 literal");
            return self.collect_integer_zero_literal();
        }
        trace!("collect_integer(): None 0 literal");
        return self.collect_non_zero_integer_literal();
    }

    fn collect_integer_zero_literal(&mut self) -> Option<Token<S>> {
        let beg = self.curr_ptr();
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => {
                trace!("collect_integer_zero_literal(): Integer starting with 0 error");
                self.integers_cannot_start_with_zero_error(beg.clone());
                let symbol = String::from("0");
                return self.collect_poisoned_integer(symbol, beg);
            }
            Some(ch) if ch.is_alphabetic() => {
                trace!("collect_integer_zero_literal(): {} is not an integer", ch);
                self.not_an_integer_error(beg.clone());
                let symbol = String::from("0");
                return self.collect_poisoned_integer(symbol, beg);
            }
            _ => Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(0),
                span: Span {
                    beg,
                    end: self.curr_ptr(),
                },
            }),
        }
    }

    fn collect_non_zero_integer_literal(&mut self) -> Option<Token<S>> {
        let mut symbol = String::new();
        trace!("collect_non_zero_integer_literal(): collecting integer");

        symbol.push(self.curr_char().unwrap());
        trace!("collect_non_zero_integer_literal(): symbol is {}", symbol);
        let beg = self.curr_ptr();

        while let Some(ch) = self.next_char() {
            trace!("collect_non_zero_integer_literal(): character is {}", ch);
            match ch {
                ch if ch.is_digit(10) => symbol.push(ch),
                ch if ch.is_alphabetic() => {
                    trace!("collect_non_zero_integer_literal(): not an integer");
                    self.not_an_integer_error(beg.clone());
                    return self.collect_poisoned_integer(symbol, beg);
                }
                _ => break,
            }
        }
        trace!(
            "collect_non_zero_integer_literal(): collected integer {}",
            symbol
        );
        return Some(token::Token {
            kind: token::Kind::IntLiteral,
            value: token::Value::Integer(symbol.parse().unwrap()),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
        });
    }

    fn collect_poisoned_integer(
        &mut self,
        mut symbol: String,
        beg: S::Pointer,
    ) -> Option<Token<S>> {
        let mut opt = self.curr_char();
        while let Some(ch) = opt {
            if helpers::is_part_of_op(ch) || ch.is_whitespace() || ch == '\n' || ch == '#' {
                break;
            }
            symbol.push(ch);
            opt = self.next_char();
        }
        return Some(token::Token {
            kind: token::Kind::Poisoned,
            value: token::Value::String(symbol),
            span: Span {
                beg: beg,
                end: self.curr_ptr(),
            },
        });
    }

    fn collect_identifier(&mut self) -> Option<Token<S>> {
        trace!("collect_identifier(): collecting identifier");
        let mut symbol = String::new();
        trace!("collect_identifier(): unwraping curr_char");
        symbol.push(self.curr_char().unwrap());
        let beg = self.curr_ptr();
        while let Some(ch) = self.next_char() {
            trace!("collect_identifier(): curr_char {}", ch);
            if !helpers::is_part_of_ident(ch) {
                trace!("collect_identifier(): char {} is not part of an ident", ch);
                break;
            }
            trace!("collect_identifier(): adding char to symbol");
            symbol.push(ch);
            trace!("collect_identifier(): curr symbol {}", symbol);
        }

        let mut kind = token::Kind::Identifier;
        if let Some(k_kind) = helpers::is_keyword(&symbol) {
            trace!("collect_identifier(): {} is a keyword", symbol);
            kind = k_kind;
        }
        trace!("collect_identifier(): returning token");
        return Some(token::Token {
            kind,
            value: token::Value::String(symbol),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
        });
    }

    fn collect_operator(&mut self) -> Option<Token<S>> {
        trace!("collect_operator(): collecting operator");
        let mut symbol = String::new();
        trace!("collect_operator(): unwraping curr_char");
        symbol.push(self.curr_char().unwrap());
        let beg = self.curr_ptr();
        while let Some(ch) = self.next_char() {
            trace!("collect_operator(): curr_char {}", ch);
            if !helpers::is_part_of_op(ch) {
                trace!("collect_operator(): char {} is not part of an operator", ch);
                break;
            }
            trace!("collect_operator(): adding char to symbol");
            symbol.push(ch);
            trace!("collect_operator(): curr symbol {}", symbol);
        }
        trace!("collect_operator(): Returning operator");
        Some(token::Token {
            kind: if let Some(kind) = helpers::is_operator(&symbol) {
                kind
            } else {
                token::Kind::Operator
            },
            value: token::Value::String(symbol),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
        })
    }

    fn collect_parenthesis(&mut self) -> Option<Token<S>> {
        trace!("collect_parenthesis(): collecting parenthesis");
        let beg = self.curr_ptr();
        let mut symbol = String::new();
        symbol.push(self.curr_char().unwrap());
        self.next_char();
        Some(token::Token {
            kind: helpers::is_parenthesis(&symbol).unwrap(),
            value: token::Value::String(symbol),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
        })
    }

    fn collect_infix(&mut self) -> Option<Token<S>> {
        self.next_char();
        if let Some(mut tok) = self.collect_identifier() {
            tok.kind = token::Kind::InfixIdent;
            Some(tok)
        } else {
            None
        }
    }

    fn collect_char(&mut self) -> Option<Token<S>> {
        let ch = self.curr_char().unwrap();
        trace!("next(): Unknown character, returning None");
        self.unknown_character_error();
        let mut symbol = String::new();
        symbol.push(ch);
        let ptr = self.curr_ptr();
        {
            self.src.borrow_mut().next_char();
        }
        Some(token::Token {
            kind: token::Kind::Poisoned,
            value: token::Value::String(symbol),
            span: Span {
                beg: ptr.clone(),
                end: ptr,
            },
        })
    }

    fn unknown_character_error(&mut self) {
        trace!("unknown_character_error(): Error");
        self.session.borrow_mut().err(Box::new(LexingError {
            kind: LexingErrorKind::UnknownCharacter(self.curr_char().unwrap()),
            beg: self.curr_ptr(),
            end: self.curr_ptr(),
        }));
    }

    fn not_an_integer_error(&mut self, beg: S::Pointer) {
        trace!("not_an_integer_error(): Error");
        self.session.borrow_mut().err(Box::new(LexingError {
            kind: LexingErrorKind::NotAnInterger,
            beg,
            end: self.curr_ptr(),
        }));
    }

    fn integers_cannot_start_with_zero_error(&mut self, beg: S::Pointer) {
        trace!("integers_cannot_start_with_zero_error(): Error");
        self.session.borrow_mut().err(Box::new(LexingError {
            kind: LexingErrorKind::IntegersCannotStartWithZero,
            beg,
            end: self.curr_ptr(),
        }));
    }
}

// Errors

#[derive(Debug)]
pub enum LexingErrorKind {
    IntegersCannotStartWithZero,
    NotAnInterger,
    UnknownCharacter(char),
}

#[derive(Debug)]
pub struct LexingError<T: ftl_source::Pointer> {
    pub kind: LexingErrorKind,
    pub beg: T,
    pub end: T,
}

impl<T: ftl_source::Pointer + Debug> Error for LexingError<T> {}

impl<T: ftl_source::Pointer> Display for LexingError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mess = self.desc();
        writeln!(
            f,
            "Lexing error [({}:{}) - ({}:{})] {}",
            self.beg.line(),
            self.beg.position(),
            self.end.line(),
            self.end.position(),
            mess
        )
    }
}

impl<T: ftl_source::Pointer> LangError for LexingError<T> {
    type Ptr = T;

    fn desc(&self) -> String {
        match &self.kind {
            LexingErrorKind::IntegersCannotStartWithZero => {
                String::from("integers cannot start with 0")
            }
            LexingErrorKind::NotAnInterger => {
                String::from("literal is not an interger but it starts like one")
            }
            LexingErrorKind::UnknownCharacter(_) => String::from("unkown character"),
        }
    }

    fn begin(&self) -> &Self::Ptr {
        &self.beg
    }

    fn end(&self) -> &Self::Ptr {
        &self.end
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
    use ftl_utility::assert_match;

    fn make_sess_with_src(content: &str) -> RcRef<Session<ftl_source::string::String>> {
        let src = ftl_source::string::String::from(content);
        RcRef::new(Session::new(src))
    }

    #[test]
    fn lexer_creation_from_empty_string_source() {
        let sess = make_sess_with_src("");
        Lexer::new(sess);
    }

    #[test]
    fn reading_first_token_from_empty_lexer() {
        let sess = make_sess_with_src("");
        let l = Lexer::new(sess);
        assert_match!(l.curr(), None);
    }

    #[test]
    fn reading_past_first_token_from_empty_lexer() {
        let sess = make_sess_with_src("");
        let mut l = Lexer::new(sess);
        assert_match!(l.next(), None);
    }

    #[test]
    fn reading_first_token_from_just_integer_in_source() {
        let sess = make_sess_with_src("123");
        let l = Lexer::new(sess);
        assert_match!(
            l.curr(),
            Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            })
        );
    }

    #[test]
    fn skipping_comments_and_whitespaces() {
        let sess = make_sess_with_src(
            r#"
            # comment tdg d dg 
            123
        "#,
        );
        let l = Lexer::new(sess);

        assert_match!(
            l.curr(),
            Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            })
        );
    }

    #[test]
    fn get_zero_integer_literal() {
        let sess = make_sess_with_src("0");
        let l = Lexer::new(sess);
        assert_match!(
            l.curr(),
            Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(0),
                ..
            })
        );
    }

    #[test]
    fn error_on_integer_starting_from_zero() {
        let sess = make_sess_with_src("01");
        Lexer::new(sess.clone());
        assert_match!(sess.borrow_mut().handler.error_msg(), Some(_));
    }

    #[test]
    fn error_on_alpha_in_zero_literal() {
        let sess = make_sess_with_src("0a");
        Lexer::new(sess.clone());
        assert_match!(sess.borrow_mut().handler.error_msg(), Some(_));
    }

    #[test]
    fn error_on_alpha_in_nonzero_literal() {
        let sess = make_sess_with_src("657457a");
        Lexer::new(sess.clone());
        assert_match!(sess.borrow_mut().handler.error_msg(), Some(_));
    }

    #[test]
    fn read_multiple_integers() {
        let sess = make_sess_with_src(
            r#"0
        1 2 3
        5
        6
        "#,
        );
        let mut l = Lexer::new(sess);
        l.next();
        l.next();
        l.next();
        l.next();
        assert_match!(
            l.curr(),
            Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(5),
                ..
            })
        );
    }

    #[test]
    fn read_identifiers() {
        let sess = make_sess_with_src(
            r#"0
        some identifiers
        "#,
        );
        let mut l = Lexer::new(sess);
        l.next();
        let tok = l.next().unwrap();
        assert!(match tok {
            token::Token {
                kind: token::Kind::Identifier,
                value: token::Value::String(ref s),
                ..
            } => s == "identifiers",
            _ => false,
        });
    }

    #[test]
    fn read_keywords() {
        let sess = make_sess_with_src(
            r#"0
        som134e def
        "#,
        );
        let mut l = Lexer::new(sess);
        l.next();
        let tok = l.next().unwrap();
        assert!(match tok {
            token::Token {
                kind: token::Kind::FuncDef,
                value: token::Value::String(ref s),
                ..
            } => s == "def",
            _ => false,
        });
    }

    #[test]
    fn read_identifiers_alongside_integers() {
        let sess = make_sess_with_src(
            r#"0
        s_242_ome 123
        "#,
        );
        let mut l = Lexer::new(sess);
        l.next();
        l.next();
        assert_match!(
            l.curr(),
            Some(token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            })
        );
    }

    #[test]
    fn read_multiple_operators() {
        let sess = make_sess_with_src(
            r#"0
        + - ++ -- , ( )
        "#,
        );
        let mut l = Lexer::new(sess);
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap().kind, token::Kind::Comma);
        assert_match!(l.next().unwrap().kind, token::Kind::LeftParenthesis);
        assert_match!(l.next().unwrap().kind, token::Kind::RightParenthesis);
    }

    #[test]
    fn read_multiple_operators_with_integers_before_and_after() {
        let sess = make_sess_with_src(
            r#"0
        12+0 - ++ -- , ()
        "#,
        );
        let mut l = Lexer::new(sess);
        assert_match!(l.next().unwrap(),
            token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(12),
                ..
            }
        );
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap(),
            token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(0),
                ..
            }
        );
    }

    #[test]
    fn read_multiple_operators_with_identifiers_before_and_after() {
        let sess = make_sess_with_src(
            r#"0
        _Adfaf_+_12_ - ++ -- , ()
        "#,
        );
        let mut l = Lexer::new(sess);
        assert_match!(l.next().unwrap().kind, token::Kind::Identifier);
        assert_match!(l.next().unwrap().kind, token::Kind::Operator);
        assert_match!(l.next().unwrap().kind, token::Kind::Identifier);
    }

    #[test]
    fn read_multiple_operators_each_being_part_of_itself() {
        let sess = make_sess_with_src(
            r#"0
        ++--+++++
        "#,
        );
        let mut l = Lexer::new(sess);
        assert!(match l.next().unwrap() {
            token::Token {
                kind: token::Kind::Operator,
                value: token::Value::String(ref s),
                ..
            } => {
                trace!("Symbol is: {}", s);
                s == "++--+++++"
            }
            _ => false,
        });
    }

    #[test]
    fn returning_poisoned_integers() {
        let sess = make_sess_with_src("0a 123");
        let mut l = Lexer::new(sess);
        assert!(match l.curr().unwrap() {
            token::Token {
                kind: token::Kind::Poisoned,
                value: token::Value::String(ref s),
                ..
            } => {
                trace!("Symbol is: {}", s);
                s == "0a"
            }
            _ => false,
        });
        assert_match!(l.next().unwrap(), token::Token{
            kind: token::Kind::IntLiteral,
            value: token::Value::Integer(123),
            ..
        });
    }

    #[test]
    fn returning_poisoned_integers_2() {
        let sess = make_sess_with_src("123 12asdafe3");
        let mut l = Lexer::new(sess);
        assert!(match l.next().unwrap() {
            token::Token {
                kind: token::Kind::Poisoned,
                value: token::Value::String(ref s),
                ..
            } => s == "12asdafe3",
            _ => false,
        });
    }

    #[test]
    fn returning_unknown_character() {
        let sess = make_sess_with_src("123 😁0");
        let mut l = Lexer::new(sess);
        assert!(match l.next().unwrap() {
            token::Token {
                kind: token::Kind::Poisoned,
                value: token::Value::String(ref s),
                ..
            } => s == "😁",
            _ => false,
        });
    }

    #[test]
    fn returning_infix_ident() {
        let sess = make_sess_with_src("123 `abc 123");
        let mut l = Lexer::new(sess);
        assert_match!(
            l.curr().unwrap(),
            token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            }
        );
        assert!(match l.next().unwrap() {
            token::Token {
                kind: token::Kind::InfixIdent,
                value: token::Value::String(ref s),
                ..
            } => s == "abc",
            _ => false,
        });
        assert_match!(
            l.next().unwrap(),
            token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            }
        );
    }
}
