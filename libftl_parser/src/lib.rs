
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
pub mod visitor;

type PRes<T, P> = Result<T, ParseErr<P>>;


pub struct Parser<S: Source> {
    
    sess: RcRef<Session<S>>, 

    lexer: Lexer<S>,

    node_id: ast::NodeId,

}

impl<S> Parser<S> where S: Source, S::Pointer: 'static {

    //
    // Public interface 
    //

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
            Err(_) => unreachable!(),
        }
    }

    //
    // Parsing methods
    //

    fn parse_module(&mut self) -> PRes<ast::Module<S::Pointer>, S::Pointer> {
        let mut module = ast::Module{ id: self.next_node_id(), decl: Vec::new() };
        while let Ok(node) = self.parse_top_level_decl() {
            module.decl.push(node);
        }
        if let Err(ParseErr::NotThisItem(tok)) = self.parse_eof() {
            self.err(Self::msg_err("End of file expected".to_owned(), tok.span.beg, tok.span.end));
        }
        Ok(module)
    }

    fn parse_eof(&mut self) -> PRes<(), S::Pointer> {
        match self.lexer.next() {
            None => Ok(()),
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
        }
    }

    // Top level decl

    fn parse_top_level_decl(&mut self) -> PRes<ast::TopLevelDecl<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        let func_def = self.parse_func_decl()?;
        let end = self.curr_ptr();
        Ok(ast::TopLevelDecl{
            id: self.next_node_id(),
            kind: ast::TopLevelDeclKind::FunctionDef(func_def),
            span: Span {
                beg, 
                end,
            },
        })
    }

    // Function

    fn parse_func_decl(&mut self) -> PRes<ast::FuncDef<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        if let Err(err) = self.parse_token(token::Kind::FuncDef) {
            return Err(err);
        }
        let ident = match self.parse_ident() {
            Ok(id) => id,
            Err(ParseErr::NotThisItem(tok)) => {
                self.fatal(Self::unexpected_token_err(
                    token::Kind::Identifier,
                    token::Value::None,
                    tok, 
                    "A function needs an identifier as its name.".to_owned()
                ));
            },
            Err(ParseErr::EOF) => {
                self.fatal(Self::msg_err(
                    "End of file reached".to_owned(),
                    beg,
                    self.curr_ptr()
                ));
            }
        };
        let args = match self.parse_func_args() {
            Ok(args) => args,
            Err(_) => unreachable!(),
        };
        let body = match self.parse_expr() {
            Ok(expr) => expr,
            Err(ParseErr::NotThisItem(_)) =>
                self.fatal(Self::msg_err(
                    "Function needs a body definition".to_owned(),
                    beg,
                    self.curr_ptr()
                )),
            Err(ParseErr::EOF) =>
                self.fatal(Self::msg_err(
                        "End of file reached".to_owned(),
                        beg,
                        self.curr_ptr()
                    )),
        };
        Ok(ast::FuncDef{
            ident,
            args,
            body,
            ty: None,
        })
    }

    fn parse_func_args(&mut self) -> PRes<Vec<ast::FuncArg<S::Pointer>>, S::Pointer> {
        // TODO - For now no type, comma or parenthesis support
        let mut args = Vec::new();
        while let Ok(ident) = self.parse_ident() {
            args.push(
                ast::FuncArg{
                    ty: None,
                    span: Span{
                        beg: ident.span.clone().beg,
                        end: ident.span.clone().end,
                    },
                    ident: ident,
                });
        }
        Ok(args)
    }

    // Expr 

    fn parse_expr(&mut self) -> PRes<ast::Expr<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        if let Ok(lit) = self.parse_lit() {
            let end = self.curr_ptr();
            return Ok(ast::Expr{
                id: self.next_node_id(),
                kind: ast::ExprKind::Literal(lit),
                span: Span {
                    beg,
                    end,
                },
            })
        }
        if let Ok(id) = self.parse_ident() {
            let end = self.curr_ptr();
            return Ok(ast::Expr{
                id: self.next_node_id(),
                kind: ast::ExprKind::Identifier(id),
                span: Span {
                    beg,
                    end,
                },
            })
        }
        match self.lexer.curr() {
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
            None => Err(ParseErr::EOF),
        }
    }

    // Primary expr

    fn parse_ident(&mut self) -> PRes<ast::Ident<S::Pointer>, S::Pointer> {
        let tok = self.parse_token(token::Kind::Identifier)?;
        if let token::Value::String(s) = tok.value {
            Ok(ast::Ident {
                symbol: s,
                span: tok.span,
            })
        } else {
            unreachable!();
        }
    }

    // Literals

    fn parse_lit(&mut self) -> PRes<ast::Lit, S::Pointer> {
        self.parse_int_lit()
    }

    fn parse_int_lit(&mut self) -> PRes<ast::Lit, S::Pointer> {
        let tok = self.parse_token(token::Kind::IntLiteral)?;
        if let token::Value::Integer(v) = tok.value {
            Ok(ast::Lit::Int(v))
        } else {
            unreachable!();
        }
    }

    // Helpers 

    fn parse_token(&mut self, kind: token::Kind) -> PRes<token::Token<S::Pointer>, S::Pointer> {
        match self.lexer.curr() {
            Some(tok) => 
                if tok.kind == kind {
                    Ok(tok)
                } else {
                    Err(ParseErr::NotThisItem(tok))
                },
            _ => Err(ParseErr::EOF), 
        }
    }

    pub fn next_node_id(&mut self) -> ast::NodeId {
        let tmp = self.node_id;
        self.node_id += 1;
        tmp
    }

    // Delegations

    pub fn curr_ptr(&self) -> S::Pointer {
        self.lexer.curr_ptr()
    }
    
    fn err(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>) {
        self.sess.borrow_mut().err(err)
    }

    fn fatal(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>) -> ! {
        self.sess.borrow_mut().fatal(err)
    }

    // Errors

    #[allow(dead_code)]
    fn token_expected_err(
        kind: token::Kind,
        value: token::Value,
        beg: S::Pointer,
        end: S::Pointer,
        msg: String) -> Box<dyn LangError<Ptr=S::Pointer>> 
    {
        let tok = token::Token{
            kind,
            value,
            span: Span {
                beg,
                end,
            },
        };
        let err: Box<errors::ParserError<S::Pointer>> = Box::new(
            errors::ParserError{
                msg: format!("Expected token {}. {}", tok, msg),
                kind: errors::ParserErrorKind::TokenExpected(tok),
            }
        );
        err 
    }

    fn unexpected_token_err(
        kind: token::Kind,
        value: token::Value,
        actual: token::Token<S::Pointer>,
        msg: String) -> Box<dyn LangError<Ptr=S::Pointer>> 
    {
        let expected = token::Token{
            kind,
            value,
            span: actual.span.clone(),
        };
        let err: Box<errors::ParserError<S::Pointer>> = Box::new(
            errors::ParserError{
                msg: format!("Expected token {}, got {}. {}", expected, actual, msg),
                kind: errors::ParserErrorKind::UnexpectedToken{
                    expected, 
                    actual,
                },
            }
        );
        err  
    }

    fn msg_err(msg: String, beg: S::Pointer, end: S::Pointer) -> Box<dyn LangError<Ptr=S::Pointer>> {
        let err: Box<errors::ParserError<S::Pointer>> = Box::new(
            errors::ParserError{
                msg,
                kind: errors::ParserErrorKind::Msg(Span{beg: beg, end: end}),
            }
        );
        err
    }

}

pub enum ParseErr<P: ftl_source::Pointer> {
    EOF,
    NotThisItem(token::Token<P>),
}