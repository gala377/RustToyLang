use crate::ast::*;
// use crate::visitor_mut::MutPass;

use ftl_source::{
    Pointer,
    Source,
};


pub trait Pass<'ast, P: Pointer>: Sized {
    
    fn visit_module(&mut self, node: &'ast Module<P>) {
        walk_module(self, node);
    }
    
    fn visit_top_level_decl(&mut self, node: &'ast TopLevelDecl<P>) {
        walk_top_level_decl(self, node);
    }
    
    fn visit_func_decl(&mut self, node: &'ast FuncDecl<P>) {
        walk_func_decl(self, node);
    }

    fn visit_func_def(&mut self, node: &'ast FuncDef<P>) {
        walk_func_def(self, node);
    }
    
    fn visit_infix_def(&mut self, node: &'ast InfixDef<P>) {
        walk_infix_def(self, node);
    }

    fn visit_func_arg(&mut self, _node: &'ast FuncArg<P>) {
        // todo walk, when its more than just an identifier
        self.nop()
    }
    
    fn visit_func_attr(&mut self, _node: &'ast Ident<P>) {
        // todo walk, when its more than just an identifier        
        self.nop()
    }

    fn visit_expr(&mut self, node: &'ast Expr<P>) {
        walk_expr(self, node);
    }
    
    fn visit_infix_func_call(&mut self, node: &'ast InfixFuncCall<P>) {
        walk_infix_func_call(self, node);
    }

    fn visit_infix_op_call(&mut self, node: &'ast InfixOpCall<P>) {
        walk_infix_op_call(self, node);
    }

    fn visit_func_call(&mut self, node: &'ast FuncCall<P>) {
        walk_func_call(self, node);
    }
    
    fn visit_parenthesed(&mut self, node: &'ast Paren<P>) {
        walk_paren_expr(self, node);
    }

    fn visit_lit(&mut self, node: &'ast Lit<P>) {
        walk_lit(self, node);
    }

    fn visit_int_lit(&mut self, _val: u64) {
        self.nop()
    }

    fn visit_ident(&mut self, _node: &'ast Ident<P>) {
        self.nop()
    }

    fn visit_op(&mut self, _node: &'ast Op<P>) {
        self.nop()
    }

    fn visit_type(&mut self, node: &'ast Type<P>) {
        walk_type(self, node);
    }

    fn visit_func_type(&mut self, node: &'ast FuncType<P>) {
        walk_func_type(self, node);
    }

    fn visit_lit_type(&mut self, _node: &'ast LitType) {
        self.nop()
    }

    fn nop(&mut self) {}
}


pub fn visit_ast<'ast, S: Source, P: Pass<'ast, S::Pointer>>(p: &mut P, ast: &'ast AST<S>) {
    p.visit_module(&ast.root);
}


pub fn walk_module<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Module<Ptr>) {
    for decl in &node.decl {
        v.visit_top_level_decl(decl);
    }
}

pub fn walk_top_level_decl<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast TopLevelDecl<Ptr>) {
    match node.kind {
        TopLevelDeclKind::FunctionDef(ref func_def) => {
            v.visit_func_def(func_def);
        },
        TopLevelDeclKind::InfixDef(ref infix_def) => {
            v.visit_infix_def(infix_def);
        },
        TopLevelDeclKind::FunctionDecl(ref func_decl) => {
            v.visit_func_decl(func_decl);
        },
    }
}

pub fn walk_func_decl<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast FuncDecl<Ptr>) {
    v.visit_ident(&node.ident);
    if let Some(ref ty) = node.ty {
        v.visit_type(ty);
    }
    // todo: should it be here? 
    // shouldn't walks be inside the visits 
    // and visits inside the walks? 
    walk_func_attrs(v, &node.attrs);
}



pub fn walk_func_def<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast FuncDef<Ptr>) {
    v.visit_func_decl(&node.decl); 
    for arg in &node.args {
        v.visit_func_arg(arg);
    }
    v.visit_expr(&node.body);
}

pub fn walk_func_attrs<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Vec<Ident<Ptr>>) {
    for attr in node {
        v.visit_func_attr(attr);
    }
}

pub fn walk_infix_def<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast InfixDef<Ptr>) {
    // todo - for now skipping type
    // todo - what to do with the precedence
    v.visit_op(&node.op);
    v.visit_func_arg(&node.args.0);
    v.visit_func_arg(&node.args.1);
    v.visit_expr(&node.body);
}

pub fn walk_expr<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Expr<Ptr>) {
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
        ExprKind::InfixOpCall(ref infix_op_call) => {
            v.visit_infix_op_call(infix_op_call);
        },
        ExprKind::InfixFuncCall(ref infix_call) => {
            v.visit_infix_func_call(infix_call);
        },
        ExprKind::Parenthesed(ref paren) => {
            v.visit_parenthesed(paren);
        },
    }
}

pub fn walk_paren_expr<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Paren<Ptr>) {
    v.visit_expr(&node.expr);
}

pub fn walk_infix_func_call<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast InfixFuncCall<Ptr>) {
    v.visit_ident(&node.ident);
    v.visit_expr(&node.lhs);
    v.visit_expr(&node.rhs);
}

pub fn walk_infix_op_call<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast InfixOpCall<Ptr>) {
    v.visit_op(&node.op);
    v.visit_expr(&node.lhs);
    v.visit_expr(&node.rhs);
}

pub fn walk_func_call<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast FuncCall<Ptr>) {
    v.visit_expr(&node.lhs);
    for arg in &node.args {
        v.visit_expr(arg);
    }
}

pub fn walk_lit<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Lit<Ptr>) {
    match node.kind {
        LitKind::Int(val) => v.visit_int_lit(val),
    }
}

pub fn walk_type<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast Type<Ptr>) {
    use TypeKind::*;
    match node.kind {
        Function(ref func_t) => v.visit_func_type(func_t),
        Literal(ref lit_t) => v.visit_lit_type(lit_t),
    }
}

pub fn walk_func_type<'ast, Ptr: Pointer, P: Pass<'ast, Ptr>>(v: &mut P, node: &'ast FuncType<Ptr>) {
    for arg_t in &node.args {
        v.visit_type(arg_t);
    }
    v.visit_type(&node.ret);
}

// Apparently this doesn't work as I tough it would
// impl<P: Pointer, V: Pass<P>> MutPass<P> for V {

//     fn visit_module(&mut self, node: &mut Module<P>) {
//         <Self as Pass<P>>::visit_module(self, node);
//     }
    
//     fn visit_top_level_decl(&mut self, node: &mut TopLevelDecl<P>) {
//         <Self as Pass<P>>::visit_top_level_decl(self, node);
//     }
    
//     fn visit_func_decl(&mut self, node: &mut FuncDecl<P>) {
//         <Self as Pass<P>>::visit_func_decl(self, node);
//     }

//     fn visit_func_def(&mut self, node: &mut FuncDef<P>) {
//         <Self as Pass<P>>::visit_func_def(self, node);
//     }
    
//     fn visit_infix_def(&mut self, node: &mut InfixDef<P>) {
//         <Self as Pass<P>>::visit_infix_def(self, node);
//     }

//     fn visit_func_arg(&mut self, node: &mut FuncArg<P>) {
//         // todo walk, when its more than just an identifier
//         <Self as Pass<P>>::visit_func_arg(self, node);
//     }
    
//     fn visit_func_attr(&mut self, node: &mut Ident<P>) {
//         // todo walk, when its more than just an identifier        
//         <Self as Pass<P>>::visit_func_attr(self, node);
//     }

//     fn visit_expr(&mut self, node: &mut Expr<P>) {
//         <Self as Pass<P>>::visit_expr(self, node);
//     }
    
//     fn visit_infix_func_call(&mut self, ident: &mut Ident<P>, lhs: &mut Expr<P>, rhs: &mut Expr<P>) {
//         <Self as Pass<P>>::visit_infix_func_call(self, ident, lhs, rhs);    
//     }

//     fn visit_bin_expr(&mut self, op: &mut Op<P>, lhs: &mut Expr<P>, rhs: &mut Expr<P>) {
//         <Self as Pass<P>>::visit_bin_expr(self, op, lhs, rhs);
//     }

//     fn visit_func_call(&mut self, node: &mut FuncCall<P>) {
//         <Self as Pass<P>>::visit_func_call(self, node);
//     }
    
//     fn visit_parenthesed(&mut self, node: &mut Expr<P>) {
//         <Self as Pass<P>>::visit_parenthesed(self, node);
//     }

//     fn visit_lit(&mut self, node: &mut Lit<P>) {
//         <Self as Pass<P>>::visit_lit(self, node);
//     }

//     fn visit_int_lit(&mut self, val: &mut u64) {
//         <Self as Pass<P>>::visit_int_lit(self, *val);
//     }

//     fn visit_ident(&mut self, node: &mut Ident<P>) {
//         <Self as Pass<P>>::visit_ident(self, node);
//     }

//     fn visit_op(&mut self, node: &mut Op<P>) {
//         <Self as Pass<P>>::visit_op(self, node);        
//     }

//     fn visit_type(&mut self, node: &mut Type<P>) {
//         <Self as Pass<P>>::visit_type(self, node);        
//     }

//     fn visit_func_type(&mut self, node: &mut FuncType<P>) {
//         <Self as Pass<P>>::visit_func_type(self, node);        
//     }

//     fn visit_lit_type(&mut self, node: &mut LitType) {
//         <Self as Pass<P>>::visit_lit_type(self, node);                
//     }
// }