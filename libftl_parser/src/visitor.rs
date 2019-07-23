use crate::ast::*;
// use crate::visitor_mut::MutPass;

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
    
    fn visit_func_decl(&mut self, node: &FuncDecl<P>) {
        walk_func_decl(self, node);
    }

    fn visit_func_def(&mut self, node: &FuncDef<P>) {
        walk_func_def(self, node);
    }
    
    fn visit_infix_def(&mut self, node: &InfixDef<P>) {
        walk_infix_def(self, node);
    }

    fn visit_func_arg(&mut self, _node: &FuncArg<P>) {
        // todo walk, when its more than just an identifier
        self.nop()
    }
    
    fn visit_func_attr(&mut self, _node: &Ident<P>) {
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

    fn visit_type(&mut self, node: &Type<P>) {
        walk_type(self, node);
    }

    fn visit_func_type(&mut self, node: &FuncType<P>) {
        walk_func_type(self, node);
    }

    fn visit_lit_type(&mut self, _node: &LitType) {
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
        TopLevelDeclKind::InfixDef(ref infix_def) => {
            v.visit_infix_def(infix_def);
        },
        TopLevelDeclKind::FunctionDecl(ref func_decl) => {
            v.visit_func_decl(func_decl);
        },
    }
}

pub fn walk_func_decl<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &FuncDecl<Ptr>) {
    v.visit_ident(&node.ident);
    if let Some(ref ty) = node.ty {
        v.visit_type(ty);
    }
    // todo: should it be here? 
    // shouldn't walks be inside the visits 
    // and visits inside the walks? 
    walk_func_attrs(v, &node.attrs);
}



pub fn walk_func_def<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &FuncDef<Ptr>) {
    v.visit_func_decl(&node.decl); 
    for arg in &node.args {
        v.visit_func_arg(arg);
    }
    v.visit_expr(&node.body);
}

pub fn walk_func_attrs<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &Vec<Ident<Ptr>>) {
    for attr in node {
        v.visit_func_attr(attr);
    }
}

pub fn walk_infix_def<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &InfixDef<Ptr>) {
    // todo - for now skipping type
    // todo - what to do with the precedence
    v.visit_op(&node.op);
    v.visit_func_arg(&node.args.0);
    v.visit_func_arg(&node.args.1);
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

pub fn walk_type<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &Type<Ptr>) {
    use TypeKind::*;
    match node.kind {
        Function(ref func_t) => v.visit_func_type(func_t),
        Literal(ref lit_t) => v.visit_lit_type(lit_t),
    }
}

pub fn walk_func_type<Ptr: Pointer, P: Pass<Ptr>>(v: &mut P, node: &FuncType<Ptr>) {
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