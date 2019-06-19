use crate::ast::*;
use ftl_source::{
    Pointer,
    Source,
};

pub trait Pass<P: Pointer>: Sized {
    
    fn visit_module(&mut self, node: &Module<P>) {
        walk_module(self, node);
    }
    
    fn visit_top_level_decl(&mut self, node: &TopLevelDecl<P>) {
        walk_top_level_decl(self, node);
    }
    
    fn visit_func_def(&mut self, node: &FuncDef<P>) {
        walk_func_def(self, node);
    }
    
    fn visit_func_arg(&mut self, _node: &FuncArg<P>) {
        // todo walk, when its more than just an identifier
        self.nop()
    }
    
    fn visit_expr(&mut self, node: &Expr<P>) {
        walk_expr(self, node);
    }
    
    fn visit_infix_func_call(&mut self, ident: &Ident<P>, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.visit_ident(ident);
        walk_expr(self, lhs);
        walk_expr(self, rhs);
    }

    fn visit_bin_expr(&mut self, op: &Op<P>, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.visit_op(op);
        walk_expr(self, lhs);
        walk_expr(self, rhs);
    }

    fn visit_func_call(&mut self, node: &FuncCall<P>) {
        walk_func_call(self, node);
    }
    
    fn visit_parenthesed(&mut self, node: &Expr<P>) {
        walk_expr(self, node);
    }

    fn visit_lit(&mut self, node: &Lit<P>) {
        walk_lit(self, node);
    }

    fn visit_int_lit(&mut self, _val: u64) {
        self.nop()
    }

    fn visit_ident(&mut self, _node: &Ident<P>) {
        self.nop()
    }

    fn visit_op(&mut self, _node: &Op<P>) {
        self.nop()
    }

    fn nop(&mut self) {}
}


pub fn visit_ast<S: Source, P: Pass<S::Pointer>>(p: &mut P, ast: &AST<S>) {
    p.visit_module(&ast.root);
}


pub fn walk_module<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &Module<Ptr>) {
    for decl in &node.decl {
        v.visit_top_level_decl(decl);
    }
}

pub fn walk_top_level_decl<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &TopLevelDecl<Ptr>) {
    match node.kind {
        TopLevelDeclKind::FunctionDef(ref func_def) => {
            v.visit_func_def(func_def);
        },
    }
}

pub fn walk_func_def<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &FuncDef<Ptr>) {
    // todo - for now skipping type 
    v.visit_ident(&node.ident);
    for arg in &node.args {
        v.visit_func_arg(arg);
    }
    v.visit_expr(&node.body);
}

pub fn walk_expr<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &Expr<Ptr>) {
    match node.kind {
        ExprKind::FunctionCall(ref call) => {
            v.visit_func_call(call);
        },
        ExprKind::Literal(ref lit) => {
            v.visit_lit(lit);
        },
        ExprKind::Identifier(ref ident) => {
            v.visit_ident(ident);
        },
        ExprKind::Binary(ref op, ref lhs, ref rhs) => {
            match op {
                BinOp::Ident(ref ident) => v.visit_infix_func_call(ident, lhs, rhs),
                BinOp::Op(ref op) => v.visit_bin_expr(op, lhs, rhs),
            }
        },
        ExprKind::Parenthesed(ref expr) => {
            v.visit_parenthesed(expr);
        },
    }
}

pub fn walk_func_call<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &FuncCall<Ptr>) {
    v.visit_expr(&node.lhs);
    for arg in &node.args {
        v.visit_expr(arg);
    }
}

pub fn walk_lit<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &Lit<Ptr>) {
    match node.kind {
        LitKind::Int(val) => v.visit_int_lit(val),
    }
}