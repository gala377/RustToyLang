use log::debug;

use std::collections::HashMap;

use ftl_session::{
    Session,
};
use ftl_error::LangError;

use ftl_parser::visitor::*;
use ftl_parser::visitor_mut::*;
use ftl_parser::ast::*;

use ftl_source::Pointer;
use ftl_source::Source;


pub struct ExprPrecReassoc<'a, S: Source> {
    op: HashMap<String, usize>,
    sess: &'a mut Session<S>,
}

pub struct UnknownPrecedense<P: Pointer> {
    pub e_beg: P,
    pub e_end: P,
    pub ident: String,
}

impl<P: Pointer> LangError for UnknownPrecedense<P> {

    type Ptr = P;

    fn desc(&self) -> String {
        format!("Uknown operator precedence: {}", self.ident)
    } 

    fn begin(&self) -> &Self::Ptr {
        &self.e_beg
    }

    fn end(&self) -> &Self::Ptr {
        &self.e_end
    }

}

impl<'a, S: Source>  ExprPrecReassoc<'a, S> where S::Pointer: 'static {
    pub fn new(sess: &'a mut Session<S>) -> Self {
        Self{
            op: HashMap::new(),
            sess,
        }
    }

    fn get_op_prec(&mut self, op: &Op<S::Pointer>) -> usize {
        match self.op.get(&op.symbol) {
            None =>
                self.sess.fatal(Box::new(
                    UnknownPrecedense{
                        e_beg: op.span.beg.clone(),
                        e_end: op.span.end.clone(),
                        ident: op.symbol.clone(),
                    }
                )),
            Some(prec) => *prec,
        }
    }

    fn get_expr_prec(&mut self, expr: &Expr<S::Pointer>) -> usize {
        let mut prec = InferPrec::new(&self.op);
        prec.visit_expr(expr);
        match prec.get() {
            (Some(ident), None) => {
                self.sess.fatal(Box::new(
                    UnknownPrecedense{
                        e_beg: expr.span.beg.clone(),
                        e_end: expr.span.end.clone(),
                        ident,
                    }
                ));
            },
            (_, Some(prec)) => prec,
            _ => unreachable!(),
        }
    }
}

impl<'a, S: Source> MutPass<S::Pointer> for ExprPrecReassoc<'a, S> where S::Pointer: 'static {

    fn visit_module(&mut self, node: &mut Module<S::Pointer>) {
        let mut prec = InfixPrec::new();
        prec.visit_module(&node);
        self.op = prec.get();
    }

    fn visit_bin_expr(&mut self, op: &mut Op<S::Pointer>, _lhs: &mut Expr<S::Pointer>, rhs: &mut Expr<S::Pointer>) {
        let op_prec = self.get_op_prec(&op);
        let rhs_prec = self.get_expr_prec(&rhs);
        debug!("Op prec: {}, rhs prec: {}", op_prec, rhs_prec);
        
    }

}



struct InfixPrec {
    op: HashMap<String, usize>,
}

impl InfixPrec {

    pub fn new() -> Self {
        Self{
            op: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.op.clear();
    }

    pub fn get(self) -> HashMap<String, usize> {
        self.op
    }
}

impl<P: Pointer> Pass<P> for InfixPrec {

    fn visit_module(&mut self, node: &Module<P>) {
        self.clear();
        walk_module(self, node);
    }

    fn visit_infix_def(&mut self, node: &InfixDef<P>) {
        let op = node.op.symbol.clone();
        let prec = node.precedence;
        self.op.insert(op, prec);
    }

}


struct InferPrec<'a> {
    op: &'a HashMap<String, usize>,
    prec: Option<usize>,
    symbol: Option<String>,
}

impl<'a> InferPrec<'a> {
    pub fn new(op: &'a HashMap<String, usize>) -> Self {
        Self {
            op,
            prec: None,
            symbol: None,
        }
    }

    pub fn get(&self) -> (Option<String>, Option<usize>) {
        (self.symbol.clone(), self.prec)
    }
}

impl<'a, P: Pointer> Pass<P> for InferPrec<'a> {

    fn visit_bin_expr(&mut self, op: &Op<P>, _lhs: &Expr<P>, _rhs: &Expr<P>) {
        match self.op.get(&op.symbol) {
            None => {
                self.prec = None;
                self.symbol = Some(op.symbol.clone());
            },
            Some(prec) => {
                self.prec = Some(*prec);
                self.symbol = Some(op.symbol.clone());
            },
        }
    }

    fn visit_func_call(&mut self, _: &FuncCall<P>) {
        self.prec = Some(usize::max_value());
    }

    fn visit_lit(&mut self, _: &Lit<P>) {
        self.prec = Some(usize::max_value());
    }

    fn visit_parenthesed(&mut self, _: &Expr<P>) {
        self.prec = Some(usize::max_value());
    }

    fn visit_ident(&mut self, _: &Ident<P>) {
        self.prec = Some(usize::max_value());
    }

    fn visit_infix_func_call(&mut self, _ident: &Ident<P>, _lhs: &Expr<P>, _rhs: &Expr<P>) {
        self.prec = Some(usize::max_value());
    }
}