

use crate::ast::*;
use ftl_source::{
    Pointer,
    Source,
};

pub trait MutPass<P: Pointer>: Sized {
    
    fn visit_module(&mut self, node: &mut Module<P>) {
        noop_module(self, node);
    }
    
    fn visit_top_level_decl(&mut self, node: &mut TopLevelDecl<P>) {
        noop_top_level_decl(self, node);
    }
    
    fn visit_func_decl(&mut self, node: &mut FuncDecl<P>) {
        noop_func_decl(self, node);
    }

    fn visit_func_def(&mut self, node: &mut FuncDef<P>) {
        noop_func_def(self, node);
    }
    
    fn visit_infix_def(&mut self, node: &mut InfixDef<P>) {
        noop_infix_def(self, node);
    }

    fn visit_func_arg(&mut self, _node: &mut FuncArg<P>) {
        // todo walk, when its more than just an identifier
        self.nop()
    }
    
    fn visit_func_attr(&mut self, _node: &mut Ident<P>) {
        // todo walk, when its more than just an identifier        
        self.nop()
    }

    fn visit_expr(&mut self, node: &mut Expr<P>) {
        noop_expr(self, node);
    }
    
    fn visit_infix_func_call(&mut self, ident: &mut Ident<P>, lhs: &mut Expr<P>, rhs: &mut Expr<P>) {
        self.visit_ident(ident);
        noop_expr(self, lhs);
        noop_expr(self, rhs);
    }

    fn visit_bin_expr(&mut self, op: &mut Op<P>, lhs: &mut Expr<P>, rhs: &mut Expr<P>) {
        self.visit_op(op);
        noop_expr(self, lhs);
        noop_expr(self, rhs);
    }

    fn visit_func_call(&mut self, node: &mut FuncCall<P>) {
        noop_func_call(self, node);
    }
    
    fn visit_parenthesed(&mut self, node: &mut Expr<P>) {
        noop_expr(self, node);
    }

    fn visit_lit(&mut self, node: &mut Lit<P>) {
        noop_lit(self, node);
    }

    fn visit_int_lit(&mut self, _val: &mut u64) {
        self.nop()
    }

    fn visit_ident(&mut self, _node: &mut Ident<P>) {
        self.nop()
    }

    fn visit_op(&mut self, _node: &mut Op<P>) {
        self.nop()
    }

    fn visit_type(&mut self, node: &mut Type<P>) {
        noop_type(self, node);
    }

    fn visit_func_type(&mut self, node: &mut FuncType<P>) {
        noop_func_type(self, node);
    }

    fn visit_lit_type(&mut self, _node: &mut LitType) {
        self.nop()
    }

    fn nop(&mut self) {}
}


pub fn visit_ast<S: Source, P: MutPass<S::Pointer>>(p: &mut P, ast: &mut AST<S>) {
    p.visit_module(&mut ast.root);
}


pub fn noop_module<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut Module<Ptr>) {
    for decl in &mut node.decl {
        v.visit_top_level_decl(decl);
    }
}

pub fn noop_top_level_decl<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut TopLevelDecl<Ptr>) {
    match node.kind {
        TopLevelDeclKind::FunctionDef(ref mut func_def) => {
            v.visit_func_def(func_def);
        },
        TopLevelDeclKind::InfixDef(ref mut infix_def) => {
            v.visit_infix_def(infix_def);
        },
        TopLevelDeclKind::FunctionDecl(ref mut func_decl) => {
            v.visit_func_decl(func_decl);
        },
    }
}

pub fn noop_func_decl<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut FuncDecl<Ptr>) {
    v.visit_ident(&mut node.ident);
    if let Some(ref mut ty) = node.ty {
        v.visit_type(ty);
    }
    // todo: should it be here? 
    // shouldn't walks be inside the visits 
    // and visits inside the walks? 
    walk_func_attrs(v, &mut node.attrs);
}



pub fn noop_func_def<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut FuncDef<Ptr>) {
    v.visit_func_decl(&mut node.decl); 
    for arg in &mut node.args {
        v.visit_func_arg(arg);
    }
    v.visit_expr(&mut node.body);
}

pub fn walk_func_attrs<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut Vec<Ident<Ptr>>) {
    for attr in node {
        v.visit_func_attr(attr);
    }
}

pub fn noop_infix_def<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut InfixDef<Ptr>) {
    // todo - for now skipping type
    // todo - what to do with the precedence
    v.visit_op(&mut node.op);
    v.visit_func_arg(&mut node.args.0);
    v.visit_func_arg(&mut node.args.1);
    v.visit_expr(&mut node.body);
}

pub fn noop_expr<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut Expr<Ptr>) {
    match node.kind {
        ExprKind::FunctionCall(ref mut call) => {
            v.visit_func_call(call);
        },
        ExprKind::Literal(ref mut lit) => {
            v.visit_lit(lit);
        },
        ExprKind::Identifier(ref mut ident) => {
            v.visit_ident(ident);
        },
        ExprKind::Binary(ref mut op, ref mut lhs, ref mut rhs) => {
            match op {
                BinOp::Ident(ref mut ident) => v.visit_infix_func_call(ident, lhs, rhs),
                BinOp::Op(ref mut op) => v.visit_bin_expr(op, lhs, rhs),
            }
        },
        ExprKind::Parenthesed(ref mut expr) => {
            v.visit_parenthesed(expr);
        },
    }
}

pub fn noop_func_call<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut FuncCall<Ptr>) {
    v.visit_expr(&mut node.lhs);
    for arg in &mut node.args {
        v.visit_expr(arg);
    }
}

pub fn noop_lit<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut Lit<Ptr>) {
    match node.kind {
        LitKind::Int(ref mut val) => v.visit_int_lit(val),
    }
}

pub fn noop_type<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut Type<Ptr>) {
    use TypeKind::*;
    match node.kind {
        Function(ref mut func_t) => v.visit_func_type(func_t),
        Literal(ref mut lit_t) => v.visit_lit_type(lit_t),
    }
}

pub fn noop_func_type<Ptr: Pointer, P: MutPass<Ptr>>(v: &mut P, node: &mut FuncType<Ptr>) {
    for arg_t in &mut node.args {
        v.visit_type(arg_t);
    }
    v.visit_type(&mut node.ret);
}