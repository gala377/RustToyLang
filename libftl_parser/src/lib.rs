
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
        match self.parse_token(token::Kind::Colon) {
            Err(ParseErr::EOF) => 
                self.fatal(Self::msg_err(
                    "End of file reached".to_owned(), beg, self.curr_ptr())),
            Err(ParseErr::NotThisItem(tok)) => {
                self.err(Self::unexpected_token_err(
                    token::Kind::Colon,
                    token::Value::None, 
                    tok, "Colon expected".to_owned()))
            }
            _ => (),  
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
        self.parse_infix_expr()
    }

    fn parse_infix_expr(&mut self) -> PRes<ast::Expr<S::Pointer>, S::Pointer> {
        let mut lhs = self.parse_func_call()?;
        while let Ok(op) = self.one_of_tok(vec![
            token::Kind::InfixIdent,
            token::Kind::Operator]) 
        {
            let beg = self.curr_ptr();
            let rhs = match self.parse_func_call() {
                Ok(expr) => expr,
                _ => {
                    self.err(Self::msg_err(
                        "Expected primary expression after operator or infix call".to_owned(),
                        beg, self.curr_ptr()));
                    break;
                }
            };
            let sym = match op.value {
                token::Value::String(s) => s,
                _ => unreachable!(),
            };
            lhs = ast::Expr{
                id: self.next_node_id(),
                span: Span {
                    beg: lhs.span.clone().beg,
                    end: self.curr_ptr(),
                },
                kind: ast::ExprKind::Binary(
                    match op.kind {
                        token::Kind::InfixIdent => ast::BinOp::Ident(
                            ast::Ident{
                                symbol: sym,
                                span: op.span,
                        }),
                        token::Kind::Operator => ast::BinOp::Op(
                            ast::Op {
                                symbol: sym,
                                span: op.span,
                        }),
                        _ => unreachable!(),
                    },
                    Box::new(lhs),
                    Box::new(rhs),
                ),
            }
        }
        Ok(lhs)
    }

    fn parse_func_call(&mut self) -> PRes<ast::Expr<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        if let Err(_) = self.parse_token(token::Kind::At) {
            return self.parse_primary_expr();
        }
        let lhs = match self.parse_primary_expr() {
            Ok(e) => e,
            Err(ParseErr::NotThisItem(_)) =>
                self.fatal(Self::msg_err(
                    "Expected expression after call operator".to_owned(),
                    beg, 
                    self.curr_ptr())),
            Err(ParseErr::EOF) => self.eof_reached_fatal(beg, self.curr_ptr()),
        };
        let mut args = Vec::new();
        while let Ok(arg) = self.parse_primary_expr() {
            args.push(Box::new(arg));
        }
        Ok(ast::Expr {
            id: self.next_node_id(),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
            kind: ast::ExprKind::FunctionCall(
                ast::FuncCall{
                    lhs: Box::new(lhs),
                    args: args,
            }),
        })
    }

    // Primary expr

    fn parse_primary_expr(&mut self) -> PRes<ast::Expr<S::Pointer>, S::Pointer> {
        if let Ok(ident) = self.parse_ident() {
            return Ok(ast::Expr{
                id: self.next_node_id(),
                span: Span {
                    beg: ident.span.clone().beg,
                    end: ident.span.clone().end,
                },
                kind: ast::ExprKind::Identifier(ident),
            });
        }
        if let Ok(lit) = self.parse_lit() {
            return Ok(ast::Expr{
                id: self.next_node_id(),
                span: Span {
                    beg: lit.span.clone().beg,
                    end: lit.span.clone().end,
                },
                kind: ast::ExprKind::Literal(lit),
            });
        }
        if let expr @ Ok(_) = self.parse_parenthesis_expr() {
            return expr;
        }
        match self.lexer.curr() {
            None => Err(ParseErr::EOF),
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
        }
    }

    fn parse_parenthesis_expr(&mut self) -> PRes<ast::Expr<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        self.parse_token(token::Kind::LeftParenthesis)?;
        let expr = match self.parse_expr() {
            Ok(e) => e,
            Err(ParseErr::EOF) => 
                self.fatal(Self::msg_err(
                    "End of file reached".to_owned(),
                    beg,
                    self.curr_ptr(),
                )),
            Err(ParseErr::NotThisItem(_)) => 
                self.fatal(Self::msg_err(
                    "Expression expected after opening parenthesis '('".to_owned(),
                    beg,
                    self.curr_ptr()
                )),
        };
        match self.parse_token(token::Kind::RightParenthesis) {
            Err(ParseErr::EOF) => 
                self.err(Self::msg_err(
                    "End of file reached".to_owned(),
                    beg.clone(),
                    self.curr_ptr()
                )),
            Err(ParseErr::NotThisItem(tok)) => 
                self.err(Self::unexpected_token_err(
                    token::Kind::RightParenthesis,
                    token::Value::String(String::from(")")),
                    tok,
                    "Expected closing parenthesis".to_owned()
                )),
            _ => (),
        };
        Ok(ast::Expr{
            id: self.next_node_id(),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
            kind: ast::ExprKind::Parenthesed(Box::new(expr)),
        })
    }

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

    fn parse_lit(&mut self) -> PRes<ast::Lit<S::Pointer>, S::Pointer> {
        self.parse_int_lit()
    }

    fn parse_int_lit(&mut self) -> PRes<ast::Lit<S::Pointer>, S::Pointer> {
        let beg = self.curr_ptr();
        let tok = self.parse_token(token::Kind::IntLiteral)?;
        if let token::Value::Integer(v) = tok.value {
            Ok(ast::Lit {
                kind: ast::LitKind::Int(v),
                span: Span {
                    beg,
                    end: self.curr_ptr(),
                }
            })
        } else {
            unreachable!();
        }
    }

    // Helpers 

    fn parse_token(&mut self, kind: token::Kind) -> PRes<token::Token<S::Pointer>, S::Pointer> {
        match self.lexer.curr() {
            Some(tok) => 
                if tok.kind == kind {
                    self.lexer.next();
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

    fn one_of_tok(&mut self, kinds: Vec<token::Kind>) -> PRes<token::Token<S::Pointer>, S::Pointer> {
        for k in kinds {
            if let ret @ Ok(_) = self.parse_token(k) {
                return ret;
            }
        }
        match self.lexer.curr() {
            None => Err(ParseErr::EOF),
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
        }
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

    fn eof_reached_fatal(&mut self, beg: S::Pointer, end: S::Pointer) -> ! {
        self.fatal(Self::msg_err(
            "End of file reached".to_owned(), beg, end))
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