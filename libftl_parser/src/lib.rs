
use ftl_utility::RcRef;
use ftl_lexer::{
    Lexer,
    token,
};
use ftl_session::Session;
use ftl_source::{
    Source,
    Span,
};
use ftl_error::LangError;

pub mod ast;
pub mod errors;

type PRes<T> = Result<T, ParseErr>;


pub struct Parser<S: Source> {
    
    sess: RcRef<Session<S>>, 

    lexer: Lexer<S>,

    node_id: ast::NodeId,

}

impl<S> Parser<S> where S: Source, S::Pointer: 'static {


    pub fn new(lexer: Lexer<S>, sess: RcRef<Session<S>>) -> Self {
        Self {
            lexer,
            sess,
            node_id: 0,
        }
    }

    pub fn parse(&mut self) -> ast::AST<S> {
        match self.parse_module() {
            Ok(module) => ast::AST::new(module),
            Err(ParseErr::NotThisItem) => unreachable!(),
        }
    }

    fn parse_module(&mut self) -> PRes<ast::Module<S::Pointer>> {
        unimplemented!()
        // TODO implement
    }

    fn parse_top_level_decl(&mut self) -> PRes<ast::TopLevelDecl<S::Pointer>> {
        let beg = self.lexer.curr_ptr();
        match self.parse_func_decl() {
            Ok(func_def) => {
                let end = self.lexer.curr_ptr();
                Ok(ast::TopLevelDecl{
                    id: self.next_node_id(),
                    kind: ast::TopLevelDeclKind::FunctionDef(func_def),
                    span: Span {
                        beg, 
                        end,
                    },
                })
            },
            Err(err) => Err(err),
        }
    }

    fn parse_func_decl(&mut self) -> PRes<ast::FuncDef<S::Pointer>> {
        let beg = self.lexer.curr_ptr();
        if let None = self.parse_token(token::Kind::FuncDef) {
            return Err(ParseErr::NotThisItem);
        }
        let id = match self.parse_ident() {
            Ok(id) => id,
            Err(ParseErr::NotThisItem) => {
                self.fatal(Self::token_expected_err(
                    token::Kind::Identifier, token::Value::None, beg, self.lexer.curr_ptr()));
            }
        };

        // TODO - parse func args
        // TODO - parse func body (parse_expr)

        unimplemented!()
    }

    fn parse_ident(&mut self) -> PRes<ast::Ident<S::Pointer>> {
        match self.parse_token(token::Kind::Identifier) {
            None => Err(ParseErr::NotThisItem),
            Some(tok) => 
                if let token::Value::String(s) = tok.value {
                    Ok(ast::Ident {
                        symbol: s,
                        span: tok.span,
                    })
                } else {
                    unreachable!();
                }
        }
    }

    fn parse_int_lit(&mut self) -> PRes<ast::Lit> {
        match self.parse_token(token::Kind::IntLiteral) {
            None => Err(ParseErr::NotThisItem),
            Some(tok) => 
                if let token::Value::Integer(v) = tok.value {
                    Ok(ast::Lit::Int(v))
                } else {
                    unreachable!();
                }
        }


    }

    fn parse_token(&mut self, kind: token::Kind) -> Option<token::Token<S::Pointer>> {
        match self.lexer.curr() {
            Some(tok) => 
                if tok.kind == kind {
                    Some(tok)
                } else {
                    None
                },
            _ => None, 
        }
    }

    fn next_node_id(&mut self) -> ast::NodeId {
        let tmp = self.node_id;
        self.node_id += 1;
        tmp
    }
    
    fn err(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>) {
        self.sess.borrow_mut().err(err)
    }

    fn fatal(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>) -> ! {
        self.sess.borrow_mut().fatal(err)
    }

    fn token_expected_err(
        kind: token::Kind,
        value: token::Value,
        beg: S::Pointer,
        end: S::Pointer) -> Box<dyn LangError<Ptr=S::Pointer>> 
    {
        let err: Box<errors::ParserError<S::Pointer>> = Box::new(
            errors::ParserError::TokenExpected(
                token::Token{
                    kind,
                    value,
                    span: Span {
                        beg,
                        end,
                    },
                }
            )
        );
        err 
    }
}

pub enum ParseErr {
    NotThisItem,
}