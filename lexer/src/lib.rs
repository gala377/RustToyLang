use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

pub mod source;
pub mod token;

mod helpers;

use crate::source::Source;

pub type Token<T> = token::Token<<T as Source>::Pointer>;
pub type Result<T> = std::result::Result<Option<Token<T>>, LexingError<<T as Source>::Pointer>>;

pub struct Lexer<S: Source> {
    src: S,
    curr_token: Option<Token<S>>,
}


impl<S: Source> Lexer<S> {
    
    pub fn new(src: S) -> std::result::Result<Self, LexingError<S::Pointer>> {
        let mut s =  Self {
            src,
            curr_token: None,
        };
        s.next()?;
        return Ok(s);
    }

    pub fn curr(&self) -> Option<Token<S>> {
        self.curr_token.clone()
    }

    pub fn next(&mut self) -> Result<S> {
        let mut opt = self.src.curr_char();
        while let Some(ch) = opt {
            if ch == '#' || ch.is_whitespace() || ch == '\n' {
                self.skip_whitespaces();
                self.skip_comments();
                opt = self.src.curr_char();
            } else {
                break; 
            }
        }
        let res =  match opt {
            Some(ch) if ch.is_digit(10) => self.collect_integer(),
            Some(ch) if helpers::is_beg_of_ident(ch) => self.collect_identifier(),
            Some(ch) if helpers::is_part_of_op(ch) => self.collect_operator(),
            Some(_) => Err(
                LexingError{
                    kind: LexingErrorKind::UnknownCharacter,
                    beg: self.src.curr_ptr(),
                    end: self.src.curr_ptr(),
                }),
            None => Ok(None),
        };
        self.curr_token = res?;
        return Ok(self.curr());
    }

    fn skip_whitespaces(&mut self) {
        let mut opt = self.src.curr_char();
        while let Some(ch) = opt {
            if !(ch.is_whitespace() || ch == '\n') {
                break;
            }
            opt = self.src.next_char();             
        }
    }

    fn skip_comments(&mut self) {
        if let Some('#') = self.src.curr_char() {
            while let Some(ch) = self.src.next_char() {
                if ch == '\n' {
                    self.src.next_char();
                    break;
                }
            }
        }
    }

    fn collect_integer(&mut self) -> Result<S> {
        if let Some('0') = self.src.curr_char() {
            return self.collect_integer_zero_literal();
        }
        return self.collect_non_zero_integer_literal();
        
        
    }

    fn collect_integer_zero_literal(&mut self) -> Result<S> {
        let beg = self.src.curr_ptr();
        match self.src.next_char() {
            Some(ch) if ch.is_digit(10) => Err(
                LexingError{
                    kind: LexingErrorKind::IntegersCannotStartWithZero,
                    beg,
                    end: self.src.curr_ptr(),
                }),
            Some(ch) if ch.is_alphabetic() => Err(
                LexingError{
                    kind: LexingErrorKind::NotAnInterger,
                    beg,
                    end: self.src.curr_ptr(),
                }),
            _ => Ok(Some(token::Token{
                    kind: token::Kind::IntLiteral,
                    value: token::Value::Integer(0),
                    beg , 
                    end: self.src.curr_ptr(),
                })),
        }
    }

    fn collect_non_zero_integer_literal(&mut self)  -> Result<S> {
        let mut symbol = String::new();
        symbol.push(self.src.curr_char().unwrap());
        let beg = self.src.curr_ptr();

        while let Some(ch) = self.src.next_char() {
            match ch {
                ch if ch.is_digit(10) => symbol.push(ch),
                ch if ch.is_alphabetic() => return Err(
                    LexingError{
                        kind: LexingErrorKind::NotAnInterger,
                        beg,
                        end: self.src.curr_ptr(),
                    }),
                _ => break,
            }
        }

        return Ok( Some(
            token::Token {
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(symbol.parse().unwrap()),
                beg, 
                end: self.src.curr_ptr(),
            }
        ));
    }
    
    fn collect_identifier(&mut self) -> Result<S> {
        let mut symbol = String::new();
        symbol.push(self.src.curr_char().unwrap());
        let beg = self.src.curr_ptr();
        while let Some(ch) = self.src.next_char() {
            if !helpers::is_part_of_ident(ch) {
                break;
            }
            symbol.push(ch);
        }

        let mut kind = token::Kind::Identifier;
        if let Some(k_kind) = helpers::is_keyword(&symbol) {
            kind = k_kind;
        }
        return Ok(Some(token::Token{
                kind,
                value: token::Value::String(symbol), 
                beg,
                end: self.src.curr_ptr(),
            }));
    }

    fn collect_operator(&mut self) -> Result<S> {
        let mut symbol = String::new();
        symbol.push(self.src.curr_char().unwrap());
        let beg = self.src.curr_ptr();
        let mut op_kind = token::Kind::Comma; // it doesn't matter really
        while let Some(kind) = helpers::is_operator(&symbol) {
            op_kind = kind;
            match self.src.next_char() {
                Some(ch) => symbol.push(ch),
                None => symbol.push('\0'),
            }   
        }
        symbol.pop();
        Ok(Some(token::Token{
            kind: op_kind,
            value: token::Value::String(symbol),
            beg,
            end: self.src.curr_ptr(),
        }))
    }
}

#[derive(Debug)]
pub enum LexingErrorKind {
    IntegersCannotStartWithZero,
    NotAnInterger,
    UnknownCharacter,

}

#[derive(Debug)]
pub struct LexingError<T: source::Pointer> {
    pub kind: LexingErrorKind,
    pub beg: T,
    pub end: T,
}

impl<T: source::Pointer + Debug> Error for LexingError<T> {}
impl<T: source::Pointer> Display for LexingError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mess = match &self.kind {
            LexingErrorKind::IntegersCannotStartWithZero => "integers cannot start with 0",
            LexingErrorKind::NotAnInterger => "literal is not an interger but it starts like one",
            LexingErrorKind::UnknownCharacter => "uknown character",
        };
        writeln!(f, "Lexing error [({}:{}) - ({}:{})] {}",
            self.beg.line(),
            self.beg.position(),
            self.end.line(),
            self.end.position(),
            mess)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use utility::assert_match;

    #[test]
    fn lexer_creation_from_empty_string_source() {
        Lexer::new(source::string::String::from("")).unwrap();
    }

    #[test]
    fn reading_first_token_from_empty_lexer() {
        let l = Lexer::new(source::string::String::from("")).unwrap();
        assert_match!(
            l.curr(),
            None);
    }

    #[test]
    fn reading_past_first_token_from_empty_lexer() {
        let mut l = Lexer::new(source::string::String::from("")).unwrap();
        assert_match!(
            l.next(),
            Ok(None));
    }

    #[test]
    fn reading_first_token_from_just_integer_in_source() {
        let l = Lexer::new(source::string::String::from("123")).unwrap();
        assert_match!(
            l.curr(),
            Some(token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            }));
    }

    #[test]
    fn skipping_comments_and_whitespaces() {
        let l = Lexer::new(source::string::String::from(r#"
            # comment tdg d dg 
        123
        "#)).unwrap();
        assert_match!(
            l.curr(),
            Some(token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            }));
    }

    #[test]
    fn get_zero_integer_literal() {
        let l = Lexer::new(source::string::String::from(r#"0"#)).unwrap();
        assert_match!(
            l.curr(),
            Some(token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(0),
                ..
            }));
    }

    #[test]
    #[should_panic]
    fn panic_on_integer_starting_from_zero() {
        Lexer::new(source::string::String::from(r#"01"#)).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_on_alpha_in_zero_literal() {
        Lexer::new(source::string::String::from(r#"0a"#)).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_on_alpha_in_nonzero_literal() {
        Lexer::new(source::string::String::from(r#"657457a"#)).unwrap();
    }

    #[test]
    fn read_multiple_integers() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        1 2 3
        5
        6
        "#)).unwrap();
        l.next().unwrap(); l.next().unwrap(); l.next().unwrap(); l.next().unwrap();
        assert_match!(
            l.curr(),
            Some(token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(5),
                ..
            }));
    }

    #[test]
    fn read_identifiers() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        some identifiers
        "#)).unwrap();
        l.next().unwrap(); 
        let tok = l.next().unwrap().unwrap();
        assert!(match tok {
            token::Token{
                kind: token::Kind::Identifier,
                value: token::Value::String(ref s),
                ..} => {
                s == "identifiers"
            },
            _ => false,
        });
    }

    #[test]
    fn read_keywords() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        som134e def
        "#)).unwrap();
        l.next().unwrap(); 
        let tok = l.next().unwrap().unwrap();
        assert!(match tok {
            token::Token{
                kind: token::Kind::FuncDef,
                value: token::Value::String(ref s),
                ..} => {
                s == "def"
            },
            _ => false,
        });
        
    }

    #[test]
    fn read_identifiers_alongside_integers() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        s_242_ome 123
        "#)).unwrap();
        l.next().unwrap(); l.next().unwrap();
        assert_match!(
            l.curr(),
            Some(token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(123),
                ..
            }));
    }

    #[test]
    fn read_multiple_operators() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        + - ++ -- , ()
        "#)).unwrap();
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Addition);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Substraction);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Increment);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Decrement);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Comma);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::LeftParenthesis);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::RightParenthesis);
    }

    #[test]
    fn read_multiple_operators_with_integers_before_and_after() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        12+0 - ++ -- , ()
        "#)).unwrap();
        assert_match!(l.next().unwrap().unwrap(),
            token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(12),
                ..
            }
        );
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Addition);
        assert_match!(l.next().unwrap().unwrap(),
            token::Token{
                kind: token::Kind::IntLiteral,
                value: token::Value::Integer(0),
                ..
            }
        );
    }

    #[test]
    fn read_multiple_operators_with_identifiers_before_and_after() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        _Adfaf_+_12_ - ++ -- , ()
        "#)).unwrap();
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Identifier);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Addition);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Identifier);
    }

    #[test]
    fn read_multiple_operators_each_being_part_of_itself() {
        let mut l = Lexer::new(source::string::String::from(r#"0
        ++--+++++
        "#)).unwrap();
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Increment);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Decrement);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Increment);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Increment);
        assert_match!(l.next().unwrap().unwrap().kind, token::Kind::Addition);        
    }
}
