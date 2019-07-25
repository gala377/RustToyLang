

use crate::ast::*;
use ftl_source::{
    Pointer,
    Source,
};


pub fn visit_ast_mut<'ast, S: Source, P: MutPass<'ast, S::Pointer>>(p: &mut P, ast: &'ast mut AST<S>) {
    p.visit_module(&mut ast.root);
}

pub trait MutPass<'ast, P: Pointer>: Sized {
    
    fn visit_module(&mut self, node: &'ast mut Module<P>) {
        noop_module(self, node);
    }
    
    fn visit_top_level_decl(&mut self, node: &'ast mut TopLevelDecl<P>) {
        noop_top_level_decl(self, node);
    }
    
    fn visit_func_decl(&mut self, node: &'ast mut FuncDecl<P>) {
        noop_func_decl(self, node);
    }

    fn visit_func_def(&mut self, node: &'ast mut FuncDef<P>) {
        noop_func_def(self, node);
    }
    
    fn visit_infix_def(&mut self, node: &'ast mut InfixDef<P>) {
        noop_infix_def(self, node);
    }

    fn visit_func_arg(&mut self, _node: &'ast mut FuncArg<P>) {
        // todo walk, when its more than just an identifier
        self.nop()
    }
    
    fn visit_func_attr(&mut self, _node: &'ast mut FuncAttr<P>) {
        // todo walk, when its more than just an identifier        
        self.nop()
    }

    fn visit_expr(&mut self, node: &'ast mut Expr<P>) {
        noop_expr(self, node);
    }
    
    fn visit_infix_func_call(&mut self, node: &'ast mut InfixFuncCall<P>) {
        noop_infix_func_call(self, node);
    }

    fn visit_infix_op_call(&mut self, node: &'ast mut InfixOpCall<P>) {
        noop_infix_op_call(self, node);
    }

    fn visit_func_call(&mut self, node: &'ast mut FuncCall<P>) {
        noop_func_call(self, node);
    }
    
    fn visit_parenthesed(&mut self, node: &'ast mut Paren<P>) {
        noop_paren(self, node);
    }

    fn visit_lit(&mut self, node: &'ast mut Lit<P>) {
        noop_lit(self, node);
    }

    fn visit_int_lit(&mut self, _val: &'ast mut u64) {
        self.nop()
    }

    fn visit_ident(&mut self, _node: &'ast mut Ident<P>) {
        self.nop()
    }

    fn visit_op(&mut self, _node: &'ast mut Op<P>) {
        self.nop()
    }

    fn visit_type(&mut self, node: &'ast mut Type<P>) {
        noop_type(self, node);
    }

    fn visit_func_type(&mut self, node: &'ast mut FuncType<P>) {
        noop_func_type(self, node);
    }

    fn visit_lit_type(&mut self, _node: &'ast mut LitType) {
        self.nop()
    }

    fn nop(&mut self) {}
}


pub fn visit_ast<'ast, S: Source, P: MutPass<'ast, S::Pointer>>(p: &mut P, ast: &'ast mut AST<S>) {
    p.visit_module(&mut ast.root);
}


pub fn noop_module<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Module<Ptr>) {
    for decl in &mut node.decl {
        v.visit_top_level_decl(decl);
    }
}

pub fn noop_top_level_decl<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut TopLevelDecl<Ptr>) {
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

pub fn noop_func_decl<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut FuncDecl<Ptr>) {
    v.visit_ident(&mut node.ident);
    if let Some(ref mut ty) = node.ty {
        v.visit_type(ty);
    }
    // todo: should it be here? 
    // shouldn't walks be inside the visits 
    // and visits inside the walks? 
    walk_func_attrs(v, &mut node.attrs);
}



pub fn noop_func_def<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut FuncDef<Ptr>) {
    v.visit_func_decl(&mut node.decl); 
    for arg in &mut node.args {
        v.visit_func_arg(arg);
    }
    v.visit_expr(&mut node.body);
}

pub fn walk_func_attrs<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Vec<FuncAttr<Ptr>>) {
    for attr in node {
        v.visit_func_attr(attr);
    }
}

pub fn noop_infix_def<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut InfixDef<Ptr>) {
    // todo - for now skipping type
    // todo - what to do with the precedence
    v.visit_op(&mut node.op);
    v.visit_func_arg(&mut node.args.0);
    v.visit_func_arg(&mut node.args.1);
    v.visit_expr(&mut node.body);
}

pub fn noop_expr<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Expr<Ptr>) {
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
        ExprKind::InfixOpCall(ref mut infix_op_call) => {
            v.visit_infix_op_call(infix_op_call)
        },
        ExprKind::InfixFuncCall(ref mut infix_call) => {
            v.visit_infix_func_call(infix_call);
        },
        ExprKind::Parenthesed(ref mut paren) => {
            v.visit_parenthesed(paren);
        },
    }
}

pub fn noop_func_call<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut FuncCall<Ptr>) {
    v.visit_expr(&mut node.lhs);
    for arg in &mut node.args {
        v.visit_expr(arg);
    }
}

pub fn noop_infix_func_call<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut InfixFuncCall<Ptr>) {
    v.visit_ident(&mut node.ident);
    v.visit_expr(&mut node.lhs);
    v.visit_expr(&mut node.rhs);
}

pub fn noop_infix_op_call<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut InfixOpCall<Ptr>) {
    v.visit_op(&mut node.op);
    v.visit_expr(&mut node.lhs);
    v.visit_expr(&mut node.rhs);
}

pub fn noop_paren<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Paren<Ptr>) {
    v.visit_expr(&mut node.expr);  
} 

pub fn noop_lit<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Lit<Ptr>) {
    match node.kind {
        LitKind::Int(ref mut val) => v.visit_int_lit(val),
    }
}

pub fn noop_type<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut Type<Ptr>) {
    use TypeKind::*;
    match node.kind {
        Function(ref mut func_t) => v.visit_func_type(func_t),
        Literal(ref mut lit_t) => v.visit_lit_type(lit_t),
    }
}

pub fn noop_func_type<'ast, Ptr: Pointer, P: MutPass<'ast, Ptr>>(v: &mut P, node: &'ast mut FuncType<Ptr>) {
    for arg_t in &mut node.args {
        v.visit_type(arg_t);
    }
    v.visit_type(&mut node.ret);
}