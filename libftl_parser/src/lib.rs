use log::debug;

use ftl_error::LangError;
use ftl_lexer::{token, Lexer};
use ftl_session::Session;
use ftl_source::{Pointer, Source, Span};
use ftl_utility::RcRef;

pub mod ast;
pub mod errors;
pub mod utility;
pub mod visitor;
pub mod visitor_mut;

mod combinators;

use combinators::utility::pres_lift_fn;
use combinators::*;

type PRes<T, P> = Result<T, ParseErr<P>>;

pub struct Parser<S: Source> {
    sess: RcRef<Session<S>>,
    lexer: Lexer<S>,

    node_id: ast::NodeId,
    saved_ptrs: Vec<S::Pointer>,
}

impl<S, P> Parser<S>
where
    P: 'static + Pointer,
    S: 'static + Source<Pointer = P>,
{
    // Public interface

    pub fn new(lexer: Lexer<S>, sess: RcRef<Session<S>>) -> Self {
        Self {
            lexer,
            sess,
            node_id: 0,
            saved_ptrs: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> ast::AST<S> {
        match self.parse_module() {
            Ok(module) => ast::AST::new(module),
            Err(_) => unreachable!(),
        }
    }

    // Parsing methods

    fn parse_module(&mut self) -> PRes<ast::Module<P>, P> {
        let mut module = ast::Module {
            id: self.next_node_id(),
            decl: Vec::new(),
        };
        while let Ok(node) = self.parse_top_level_decl() {
            module.decl.push(node);
        }
        if let Err(ParseErr::NotThisItem(tok)) = self.parse_eof() {
            self.err(Self::msg_err(
                "End of file expected".to_owned(),
                tok.span.beg,
                tok.span.end,
            ));
        }
        Ok(module)
    }

    fn parse_eof(&mut self) -> PRes<(), P> {
        match self.lexer.next() {
            None => Ok(()),
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
        }
    }

    // Top level decl

    fn parse_top_level_decl(&mut self) -> PRes<ast::TopLevelDecl<P>, P> {
        self.push_ptr();
        let kind = if let Ok(func_decl) = self.parse_func_decl() {
            ast::TopLevelDeclKind::FunctionDecl(func_decl)
        } else if let Ok(func_def) = self.parse_func_def() {
            ast::TopLevelDeclKind::FunctionDef(func_def)
        } else if let Ok(infix_def) = self.parse_infix_decl() {
            ast::TopLevelDeclKind::InfixDef(infix_def)
        } else {
            return Err(match self.lexer.curr() {
                Some(tok) => ParseErr::NotThisItem(tok),
                None => ParseErr::EOF,
            });
        };
        Ok(ast::TopLevelDecl {
            id: self.next_node_id(),
            kind,
            span: Span {
                beg: self.pop_ptr(),
                end: self.curr_ptr(),
            },
        })
    }

    // Function

    fn parse_infix_decl(&mut self) -> PRes<ast::InfixDef<P>, P> {
        self.parse_token(token::Kind::InfixDef)?;
        let precedence = Comb(self)
            .r#try(Self::parse_int_lit)
            .map(pres_lift_fn(
                |ast::Lit {
                     kind: ast::LitKind::Int(val),
                     ..
                 }| val as usize,
            ))
            .fail_unex_tok(
                token::Kind::IntLiteral,
                token::Value::None,
                "Infix declaration needs to have its precendence.".to_owned(),
            )
            .run();
        let op = Comb(self)
            .r#try(Self::parse_op)
            .fail_unex_tok(
                token::Kind::Operator,
                token::Value::None,
                "An infix needs an operator as its name.".to_owned(),
            )
            .run();
        let (arg_1, arg_2) = self.parse_infix_decl_args();
        self.try_parse_token_rec(
            token::Kind::Colon,
            "Colon expected".to_owned(),
            token::Value::String(";".to_owned()),
        );
        let body = Comb(self)
            .r#try(Self::parse_expr)
            .fail_msg("Infix needs a body definition".to_owned())
            .run();
        Ok(ast::InfixDef {
            id: self.next_node_id(),
            ty: None,
            op,
            body,
            args: (arg_1, arg_2),
            precedence,
        })
    }

    fn parse_infix_decl_args(&mut self) -> (ast::FuncArg<P>, ast::FuncArg<P>) {
        let arg_1 = Comb(self)
            .r#try(Self::parse_func_arg)
            .fail_msg("Infix needs 2 arguments".to_owned())
            .run();
        let arg_2 = Comb(self)
            .r#try(Self::parse_func_arg)
            .fail_msg("Infix needs 2 arguments".to_owned())
            .run();
        (arg_1, arg_2)
    }

    fn parse_func_decl(&mut self) -> PRes<ast::FuncDecl<P>, P> {
        self.parse_token(token::Kind::FuncDecl)?;
        let ident =
            self.try_parse_ident_fail("A function needs an identifier as its name.".to_owned());
        let args_t = self.parse_func_args_types().unwrap_or_default();
        let attrs = self.parse_func_attrs().unwrap_or_default();
        self.try_parse_token_rec(
            token::Kind::Colon,
            "Colon expected".to_owned(),
            token::Value::String(")".to_owned()),
        );
        let ret_t = self.parse_type()?;
        Ok(ast::FuncDecl {
            id: self.next_node_id(),
            ty: Some(ast::Type {
                id: self.next_node_id(),
                span: Span {
                    beg: if args_t.is_empty() {
                        ret_t.span.beg.clone()
                    } else {
                        args_t[0].span.beg.clone()
                    },
                    end: ret_t.span.end.clone(),
                },
                kind: ast::TypeKind::Function(ast::FuncType {
                    id: self.next_node_id(),
                    ret: Box::new(ret_t),
                    args: args_t.into_iter().collect(),
                }),
            }),
            attrs,
            ident,
        })
    }

    fn parse_func_args_types(&mut self) -> PRes<Vec<ast::Type<P>>, P> {
        let mut args = Vec::new();
        let beg = self.peek_ptr().clone();
        while let Ok(t) = self.parse_type() {
            if let ast::TypeKind::Literal(ast::LitType::Void) = t.kind {
                self.err(Self::msg_err(
                    "Void can only be used as function return argument".to_owned(),
                    beg.clone(),
                    t.span.end,
                ))
            } else {
                args.push(t);
            }
        }
        Ok(args)
    }

    fn parse_type(&mut self) -> PRes<ast::Type<P>, P> {
        Comb(self)
            .r#try(Self::parse_simple_type)
            .or(Self::parse_func_type)
            .run()
    }

    fn parse_simple_type(&mut self) -> PRes<ast::Type<P>, P> {
        let ident = self.parse_ident()?;
        if let Some(lit) = ast::is_lit_type(&ident.symbol) {
            Ok(ast::Type {
                id: self.next_node_id(),
                kind: ast::TypeKind::Literal(lit),
                span: ident.span,
            })
        } else {
            match self.lexer.curr() {
                Some(tok) => Err(ParseErr::NotThisItem(tok)),
                None => Err(ParseErr::EOF),
            }
        }
    }

    fn parse_func_type(&mut self) -> PRes<ast::Type<P>, P> {
        let beg = self.curr_ptr();
        self.parse_token(token::Kind::LeftParenthesis)?;
        let mut args = Vec::new();
        while let Ok(t) = self.parse_type() {
            args.push(t);
        }
        self.try_parse_token_rec(
            token::Kind::RightParenthesis,
            "Unclosed parenthesis for function type".to_owned(),
            token::Value::String(")".to_owned()),
        );
        let ret = Comb(self)
            .r#try(Self::parse_type)
            .map(pres_lift_fn(Box::new))
            .fail_msg("Missing function type return type".to_owned())
            .run();
        Ok(ast::Type {
            id: self.next_node_id(),
            kind: ast::TypeKind::Function(ast::FuncType {
                id: self.next_node_id(),
                ret,
                args,
            }),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
        })
    }

    fn parse_func_attrs(&mut self) -> PRes<Vec<ast::FuncAttr<P>>, P> {
        self.parse_token(token::Kind::LeftBracket)?;
        let mut attrs = Vec::new();
        while let Ok(attr) = self.parse_ident() {
            attrs.push(ast::FuncAttr {
                id: self.next_node_id(),
                ident: attr,
            });
        }
        self.try_parse_token_rec(
            token::Kind::RightBracket,
            "Unclosed attributes parenthesis".to_owned(),
            token::Value::String("]".to_owned()),
        );
        Ok(attrs)
    }

    fn parse_func_def(&mut self) -> PRes<ast::FuncDef<P>, P> {
        self.parse_token(token::Kind::FuncDef)?;
        let ident =
            self.try_parse_ident_fail("A function needs an identifier as its name.".to_owned());
        let args = self.parse_func_args();
        let attrs = self.parse_func_attrs().unwrap_or_default();
        self.try_parse_token_rec(
            token::Kind::Colon,
            "Colon expected".to_owned(),
            token::Value::String(";".to_owned()),
        );
        let body = Comb(self)
            .r#try(Self::parse_expr)
            .fail_msg("Function needs a body definition".to_owned())
            .run();
        Ok(ast::FuncDef {
            id: self.next_node_id(),
            decl: ast::FuncDecl {
                id: self.next_node_id(),
                ident,
                attrs,
                ty: None,
            },
            args,
            body,
        })
    }

    fn parse_func_args(&mut self) -> Vec<ast::FuncArg<P>> {
        let mut args = Vec::new();
        while let Ok(arg) = self.parse_func_arg() {
            args.push(arg);
        }
        args
    }

    fn parse_func_arg(&mut self) -> PRes<ast::FuncArg<P>, P> {
        let ident = self.parse_ident()?;
        Ok(ast::FuncArg {
            id: self.next_node_id(),
            ty: None,
            span: Span {
                beg: ident.span.clone().beg,
                end: ident.span.clone().end,
            },
            ident,
        })
    }

    // Expr

    fn parse_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        self.parse_infix_expr()
    }

    fn parse_infix_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        let mut lhs = self.parse_func_call()?;
        while let Ok(op) = self.one_of_tok(vec![token::Kind::InfixIdent, token::Kind::Operator]) {
            let beg = self.curr_ptr();
            let rhs = match self.parse_func_call() {
                Ok(expr) => expr,
                _ => {
                    self.err(Self::msg_err(
                        "Expected primary expression after operator or infix call".to_owned(),
                        beg,
                        self.curr_ptr(),
                    ));
                    break;
                }
            };
            let sym = match op.value {
                token::Value::String(s) => s,
                _ => unreachable!(),
            };
            lhs = ast::Expr {
                id: self.next_node_id(),
                span: Span {
                    beg: lhs.span.clone().beg,
                    end: self.curr_ptr(),
                },
                kind: match op.kind {
                    token::Kind::InfixIdent => ast::ExprKind::InfixFuncCall(ast::InfixFuncCall {
                        id: self.next_node_id(),
                        ident: ast::Ident {
                            id: self.next_node_id(),
                            symbol: sym,
                            span: op.span,
                        },
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }),
                    token::Kind::Operator => ast::ExprKind::InfixOpCall(ast::InfixOpCall {
                        id: self.next_node_id(),
                        op: ast::Op {
                            id: self.next_node_id(),
                            symbol: sym,
                            span: op.span,
                        },
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }),
                    _ => unreachable!(),
                },
            }
        }
        Ok(lhs)
    }

    fn parse_func_call(&mut self) -> PRes<ast::Expr<P>, P> {
        let beg = self.curr_ptr();
        if self.parse_token(token::Kind::At).is_err() {
            return self.parse_primary_expr();
        }
        let lhs = Comb(self)
            .r#try(Self::parse_primary_expr)
            .fail_msg("Expected expression after call operator".to_owned())
            .run();
        let mut args = Vec::new();
        while let Ok(arg) = self.parse_primary_expr() {
            args.push(arg);
        }
        Ok(ast::Expr {
            id: self.next_node_id(),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
            kind: ast::ExprKind::FunctionCall(ast::FuncCall {
                id: self.next_node_id(),
                lhs: Box::new(lhs),
                args,
            }),
        })
    }

    // Primary expr

    fn parse_primary_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        if let expr @ Ok(_) = Comb(self)
            .r#try(Self::parse_ident_expr)
            .or(Self::parse_lit_expr)
            .or(Self::parse_parenthesis_expr)
            .run()
        {
            return expr;
        }
        match self.lexer.curr() {
            None => Err(ParseErr::EOF),
            Some(tok) => Err(ParseErr::NotThisItem(tok)),
        }
    }

    fn parse_ident_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        self.parse_ident().and_then(|ident: ast::Ident<P>| {
            Ok(ast::Expr {
                id: self.next_node_id(),
                span: Span {
                    beg: ident.span.clone().beg,
                    end: ident.span.clone().end,
                },
                kind: ast::ExprKind::Identifier(ident),
            })
        })
    }

    fn parse_lit_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        self.parse_lit().and_then(|lit: ast::Lit<P>| {
            Ok(ast::Expr {
                id: self.next_node_id(),
                span: Span {
                    beg: lit.span.clone().beg,
                    end: lit.span.clone().end,
                },
                kind: ast::ExprKind::Literal(lit),
            })
        })
    }

    fn parse_parenthesis_expr(&mut self) -> PRes<ast::Expr<P>, P> {
        let beg = self.curr_ptr();
        self.parse_token(token::Kind::LeftParenthesis)?;
        let expr = Comb(self)
            .r#try(Self::parse_expr)
            .fail_msg("Expression expected after opening parenthesis '('".to_owned())
            .run();
        self.try_parse_token_rec(
            token::Kind::RightParenthesis,
            "Expected closing parenthesis".to_owned(),
            token::Value::String(String::from(")")),
        );
        Ok(ast::Expr {
            id: self.next_node_id(),
            span: Span {
                beg,
                end: self.curr_ptr(),
            },
            kind: ast::ExprKind::Parenthesed(ast::Paren {
                id: self.next_node_id(),
                expr: Box::new(expr),
            }),
        })
    }

    fn parse_ident(&mut self) -> PRes<ast::Ident<P>, P> {
        let tok = self.parse_token(token::Kind::Identifier)?;
        if let token::Value::String(s) = tok.value {
            Ok(ast::Ident {
                id: self.next_node_id(),
                symbol: s,
                span: tok.span,
            })
        } else {
            unreachable!();
        }
    }

    fn parse_op(&mut self) -> PRes<ast::Op<P>, P> {
        let tok = self.parse_token(token::Kind::Operator)?;
        if let token::Value::String(s) = tok.value {
            Ok(ast::Op {
                id: self.next_node_id(),
                symbol: s,
                span: tok.span,
            })
        } else {
            unreachable!();
        }
    }

    // Literals

    fn parse_lit(&mut self) -> PRes<ast::Lit<P>, P> {
        self.parse_int_lit()
    }

    fn parse_int_lit(&mut self) -> PRes<ast::Lit<P>, P> {
        let beg = self.curr_ptr();
        let tok = self.parse_token(token::Kind::IntLiteral)?;
        if let token::Value::Integer(v) = tok.value {
            Ok(ast::Lit {
                id: self.next_node_id(),
                kind: ast::LitKind::Int(v),
                span: Span {
                    beg,
                    end: self.curr_ptr(),
                },
            })
        } else {
            unreachable!();
        }
    }

    // Parse helpers

    fn parse_token(&mut self, kind: token::Kind) -> PRes<token::Token<P>, P> {
        match self.lexer.curr() {
            Some(tok) => {
                if tok.kind == kind {
                    self.lexer.next();
                    Ok(tok)
                } else {
                    Err(ParseErr::NotThisItem(tok))
                }
            }
            _ => Err(ParseErr::EOF),
        }
    }

    fn try_parse_token_rec(
        &mut self,
        kind: token::Kind,
        error_msg: String,
        val: token::Value,
    ) -> token::Token<P> {
        let beg = self.peek_ptr().clone();
        self.parse_token(kind.clone())
            .unwrap_or_else(|err| match err {
                ParseErr::EOF => self.eof_reached_fatal(beg, self.curr_ptr()),
                ParseErr::NotThisItem(tok) => {
                    self.err(Self::unexpected_token_err(
                        kind.clone(),
                        val.clone(),
                        tok.clone(),
                        error_msg,
                    ));
                    token::Token {
                        span: tok.span,
                        kind,
                        value: val,
                    }
                }
            })
    }

    #[allow(dead_code)]
    fn try_parse_token_fail(
        &mut self,
        kind: token::Kind,
        error_msg: String,
        val: token::Value,
    ) -> token::Token<P> {
        let kind_ = kind.clone();
        Comb(self)
            .r#try(move |self_: &mut Self| self_.parse_token(kind_.clone()))
            .fail_unex_tok(kind, val, error_msg)
            .run()
    }

    fn try_parse_ident_fail(&mut self, error_msg: String) -> ast::Ident<P> {
        Comb(self)
            .r#try(Self::parse_ident)
            .fail_unex_tok(token::Kind::Identifier, token::Value::None, error_msg)
            .run()
    }

    pub fn next_node_id(&mut self) -> ast::NodeId {
        let tmp = self.node_id;
        self.node_id += 1;
        tmp
    }

    fn one_of_tok(&mut self, kinds: Vec<token::Kind>) -> PRes<token::Token<P>, P> {
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

    // Utility Helpers

    fn push_ptr(&mut self) {
        debug!("Pushing ptr onto the ptr stack");
        self.saved_ptrs.push(self.curr_ptr())
    }

    fn pop_ptr(&mut self) -> P {
        debug!("Poping value from the ptr stack");
        self.saved_ptrs
            .pop()
            .expect("Poping from empty pointer stack")
    }

    fn peek_ptr(&mut self) -> &P {
        debug!("Peeping value on the ptr stack");
        self.saved_ptrs
            .last()
            .expect("Peeking from empty pointer stack")
    }

    // Delegations

    fn curr_ptr(&self) -> P {
        self.lexer.curr_ptr()
    }

    fn err(&mut self, err: Box<dyn LangError<Ptr = P>>) {
        self.sess.borrow_mut().err(err)
    }

    fn fatal(&mut self, err: Box<dyn LangError<Ptr = P>>) -> ! {
        self.sess.borrow_mut().fatal(err)
    }

    fn eof_reached_fatal(&mut self, beg: P, end: P) -> ! {
        self.fatal(Self::msg_err("End of file reached".to_owned(), beg, end))
    }

    // Errors

    #[allow(dead_code)]
    fn token_expected_err(
        kind: token::Kind,
        value: token::Value,
        beg: P,
        end: P,
        msg: String,
    ) -> Box<dyn LangError<Ptr = P>> {
        let tok = token::Token {
            kind,
            value,
            span: Span { beg, end },
        };
        let err: Box<errors::ParserError<P>> = Box::new(errors::ParserError {
            msg: format!("Expected token {}. {}", tok, msg),
            kind: errors::ParserErrorKind::TokenExpected(tok),
        });
        err
    }

    fn unexpected_token_err(
        kind: token::Kind,
        value: token::Value,
        actual: token::Token<P>,
        msg: String,
    ) -> Box<dyn LangError<Ptr = P>> {
        let expected = token::Token {
            kind,
            value,
            span: actual.span.clone(),
        };
        let err: Box<errors::ParserError<P>> = Box::new(errors::ParserError {
            msg: format!("Expected token {}, got {}. {}", expected, actual, msg),
            kind: errors::ParserErrorKind::UnexpectedToken { expected, actual },
        });
        err
    }

    fn msg_err(msg: String, beg: P, end: P) -> Box<dyn LangError<Ptr = P>> {
        let err: Box<errors::ParserError<P>> = Box::new(errors::ParserError {
            msg,
            kind: errors::ParserErrorKind::Msg(Span { beg, end }),
        });
        err
    }
}

#[derive(Debug)]
pub enum ParseErr<P: ftl_source::Pointer> {
    EOF,
    NotThisItem(token::Token<P>),
}
