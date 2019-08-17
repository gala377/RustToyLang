use log::{debug, trace};

use std::collections::HashMap;

use ftl_error::LangError;
use ftl_session::Session;

use ftl_parser::ast::*;
use ftl_parser::visitor::*;
use ftl_parser::visitor_mut::*;

use ftl_source::Pointer;
use ftl_source::Source;

/// Error for when the pass couldn't find
/// the precedence for the encountered operator.
///
/// Implements the LangError traits and is reporter as fatal.
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

/// Mutable pass reorganizing infix operator calls based
/// on their precedence.
pub struct ExprPrecReassoc<'a, S: Source> {
    sess: &'a mut Session<S>,
}

impl<'a, S: Source> ExprPrecReassoc<'a, S>
where
    S::Pointer: 'static,
{
    // Returns new ExprPrecReassoc pass ready to be run on
    // the syntax tree.
    pub fn new(sess: &'a mut Session<S>) -> Self {
        debug!("Running EPR Pass");
        ExprPrecReassoc { sess }
    }
}
impl<'a, S: Source> MutPass<'a, S::Pointer> for ExprPrecReassoc<'a, S>
where
    S::Pointer: 'static,
{
    fn visit_module(&mut self, node: &mut Module<S::Pointer>) {
        loop {
            let mut epr = ExprReassocIteration::new(&mut self.sess);
            epr.visit_module(node);
            if let IterationRes::Done = epr.result() {
                break;
            }
        }
    }
}

/// Single iteration of reassociation.
/// In its run it can only surface expressions one level to the top
/// if there is some deeply nested, low precedence expression
/// then multiple runs of this pass are needed to fully
/// complete transformation.
///
/// To know if next iteration is needed
/// method `result` needs to be called.
/// If next iteration is needed this pass should be
/// created anew and not reused.
struct ExprReassocIteration<'a, S: Source> {
    op: HashMap<String, usize>,
    sess: &'a mut Session<S>,
    result: IterationRes,
}

/// Tells us if last iteration of the reassociacion pass
/// was the final one or do we need to run one more.
enum IterationRes {
    RunAgain,
    Done,
}

impl<'a, S: Source> ExprReassocIteration<'a, S>
where
    S::Pointer: 'static,
{
    /// Returns new ExprReassocIteration pass ready to
    /// visit a syntax tree.
    ///
    /// Further initialization is done upon visiting
    /// syntaxes tree module node.
    pub fn new(sess: &'a mut Session<S>) -> Self {
        ExprReassocIteration {
            op: HashMap::new(),
            sess,
            result: IterationRes::Done,
        }
    }

    /// After visiting syntax tree result tells
    /// if the iteration was the last one or do
    /// we need to run another one.
    pub fn result(self) -> IterationRes {
        self.result
    }

    /// Returns precedence for the given operator.
    /// Fatals if the precedence could not be found.
    fn get_op_prec(&mut self, op: &Op<S::Pointer>) -> usize {
        match self.op.get(&op.symbol) {
            None => self.sess.fatal(Box::new(UnknownPrecedense {
                e_beg: op.span.beg.clone(),
                e_end: op.span.end.clone(),
                ident: op.symbol.clone(),
            })),
            Some(prec) => *prec,
        }
    }

    /// Returns precedence for the given expression.
    /// Fatals if the precedence could not be found.
    fn get_expr_prec(&mut self, expr: &Expr<S::Pointer>) -> usize {
        let mut prec = InferPrec::new(&self.op);
        prec.visit_expr(expr);
        match prec.get() {
            (Some(ident), None) => {
                self.sess.fatal(Box::new(UnknownPrecedense {
                    e_beg: expr.span.beg.clone(),
                    e_end: expr.span.end.clone(),
                    ident,
                }));
            }
            (_, Some(prec)) => prec,
            _ => unreachable!(),
        }
    }

    /// Checks if the precedenses for the current node
    /// and its lhs are reversed if so swaps them and returns true
    /// otherwise returns false.
    fn try_prec_switch(&mut self, node: &mut Expr<S::Pointer>) -> bool {
        // FIXME: Refactor
        match node.kind {
            ExprKind::InfixOpCall(ref infix_call) => {
                let op_prec = self.get_op_prec(&infix_call.op);
                let lhs_prec = self.get_expr_prec(&infix_call.lhs);
                trace!(
                    "Op {}, prec: {}, lhs_prec: {}",
                    infix_call.op.symbol,
                    op_prec,
                    lhs_prec
                );
                if lhs_prec < op_prec {
                    let mut new_kind = infix_call.lhs.kind.clone();
                    if let ExprKind::InfixOpCall(ref mut new_infix_call) = new_kind {
                        new_infix_call.rhs = Box::new(Expr {
                            id: infix_call.lhs.id, // we dropped this id earlier so we can reuse it now
                            kind: ExprKind::InfixOpCall(InfixOpCall {
                                lhs: new_infix_call.rhs.clone(),
                                ..infix_call.clone()
                            }),
                            span: node.span.clone(),
                        });
                    } else {
                        unreachable!();
                    }
                    node.kind = new_kind;
                    true
                } else {
                    false
                }
            }
            ExprKind::InfixFuncCall(ref infix_call) => {
                let expr_prec = self.get_expr_prec(&node);
                let lhs_prec = self.get_expr_prec(&infix_call.lhs);
                trace!(
                    "Op {}, prec: {}, lhs_prec: {}",
                    infix_call.ident.symbol,
                    expr_prec,
                    lhs_prec
                );
                if lhs_prec < expr_prec {
                    let mut new_kind = infix_call.lhs.kind.clone();
                    if let ExprKind::InfixOpCall(ref mut new_infix_call) = new_kind {
                        new_infix_call.rhs = Box::new(Expr {
                            id: infix_call.lhs.id, // we dropped this id earlier so we can reuse it now
                            kind: ExprKind::InfixFuncCall(InfixFuncCall {
                                lhs: new_infix_call.rhs.clone(),
                                ..infix_call.clone()
                            }),
                            span: node.span.clone(),
                        });
                    } else {
                        unreachable!();
                    }
                    node.kind = new_kind;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

impl<'a, S: Source> MutPass<'a, S::Pointer> for ExprReassocIteration<'a, S>
where
    S::Pointer: 'static,
{
    fn visit_module(&mut self, node: &'a mut Module<S::Pointer>) {
        debug!("Running EPR Pass iteration");
        let mut prec = InfixPrec::new();
        prec.visit_module(&node);
        self.op = prec.get();

        noop_module(self, node);
    }

    fn visit_expr(&mut self, node: &'a mut Expr<S::Pointer>) {
        if self.try_prec_switch(node) {
            self.result = IterationRes::RunAgain;
            // FIXME: does it even matter?
            self.visit_expr(node);
        } else {
            noop_expr(self, node);
        }
    }
}

/// Goes through the abstract syntax tree once
/// creating map mapping operator to its precedence.
struct InfixPrec {
    /// Map operator symbols to their precedence.
    op: HashMap<String, usize>,

    /// If the pass has visited any syntax tree already.
    run_already: bool,
}

impl InfixPrec {
    /// Creates empty InfixPrec struct with empty map.
    pub fn new() -> Self {
        Self {
            op: HashMap::new(),
            run_already: false,
        }
    }

    /// Clears operators map.
    fn clear(&mut self) {
        self.op.clear();
    }

    /// Consumes record returning map of operators with their precedences.
    /// If run before visiting a syntax tree it panics.
    pub fn get(self) -> HashMap<String, usize> {
        if !self.run_already {
            panic!(
                "InfixPrec pass needs to be run on a syntax tree
                before trying to access its value"
            );
        }
        self.op
    }
}

impl<P: Pointer> Pass<'_, P> for InfixPrec {
    fn visit_module(&mut self, node: &Module<P>) {
        self.run_already = true;
        self.clear();
        walk_module(self, node);
    }

    fn visit_infix_def(&mut self, node: &InfixDef<P>) {
        let op = node.op.symbol.clone();
        let prec = node.precedence;
        self.op.insert(op, prec);
    }
}

/// Infers precedence of the visited node.
/// Visited node should be an expression or any of its kinds.
/// Trying to visit any other node panics.
struct InferPrec<'a> {
    op: &'a HashMap<String, usize>,
    prec: Option<usize>,
    symbol: Option<String>,

    run_already: bool,
}

impl<'a> InferPrec<'a> {
    /// Returns new pass ready to visit a node.
    pub fn new(op: &'a HashMap<String, usize>) -> Self {
        Self {
            op,
            prec: None,
            symbol: None,
            run_already: false,
        }
    }

    /// Returns the tuple of options.
    /// First being the identifier for the function call or
    /// the operator call.
    /// Second being its precedence which is usize::max() for any
    /// literal value and function calls and precedence set in
    /// infix declaration for the operator call.
    ///
    /// If the precedence for the operator call could not be found
    /// the precedence part of the returned value is None while the
    /// first is still operators symbol.
    ///
    /// This method should be called only after a syntax tree
    /// has been visited already. If not the method panics.
    pub fn get(&self) -> (Option<String>, Option<usize>) {
        if !self.run_already {
            panic!(
                "Getting the result of the InferPrec pass should be done
                only after the syntax tree has already been visited."
            );
        }
        (self.symbol.clone(), self.prec)
    }

    fn wrong_node(&self) -> ! {
        panic!("InferPrec can only be used on expression nodes.")
    }
}

impl<'a, P: Pointer> Pass<'a, P> for InferPrec<'a> {
    fn visit_infix_op_call(&mut self, node: &InfixOpCall<P>) {
        self.run_already = true;
        match self.op.get(&node.op.symbol) {
            None => {
                self.prec = None;
                self.symbol = Some(node.op.symbol.clone());
            }
            Some(prec) => {
                self.prec = Some(*prec);
                self.symbol = Some(node.op.symbol.clone());
            }
        }
    }

    fn visit_func_call(&mut self, _: &FuncCall<P>) {
        self.run_already = true;
        self.prec = Some(usize::max_value());
    }

    fn visit_lit(&mut self, _: &Lit<P>) {
        self.run_already = true;
        self.prec = Some(usize::max_value());
    }

    fn visit_parenthesed(&mut self, _: &Paren<P>) {
        self.run_already = true;
        self.prec = Some(usize::max_value());
    }

    fn visit_ident(&mut self, _: &Ident<P>) {
        self.run_already = true;
        self.prec = Some(usize::max_value());
    }

    fn visit_infix_func_call(&mut self, _: &InfixFuncCall<P>) {
        self.run_already = true;
        self.prec = Some(usize::max_value());
    }

    fn visit_module(&mut self, _node: &Module<P>) {
        self.wrong_node();
    }

    fn visit_top_level_decl(&mut self, _node: &TopLevelDecl<P>) {
        self.wrong_node();
    }

    fn visit_func_decl(&mut self, _node: &FuncDecl<P>) {
        self.wrong_node();
    }

    fn visit_func_def(&mut self, _node: &FuncDef<P>) {
        self.wrong_node();
    }

    fn visit_infix_def(&mut self, _node: &InfixDef<P>) {
        self.wrong_node();
    }

    fn visit_func_arg(&mut self, _node: &FuncArg<P>) {
        self.wrong_node();
    }

    fn visit_func_attr(&mut self, _node: &FuncAttr<P>) {
        self.wrong_node();
    }

    fn visit_op(&mut self, _node: &Op<P>) {
        self.wrong_node();
    }

    fn visit_type(&mut self, _node: &Type<P>) {
        self.wrong_node();
    }

    fn visit_func_type(&mut self, _node: &FuncType<P>) {
        self.wrong_node();
    }

    fn visit_lit_type(&mut self, _node: &LitType) {
        self.wrong_node();
    }
}
