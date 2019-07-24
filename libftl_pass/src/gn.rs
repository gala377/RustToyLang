use ftl_parser::visitor::*;
use ftl_parser::ast::*;

use ftl_source::Pointer;

pub struct GetNode<'a, T> {
    node: Option<&'a T>,
    id: usize, 
}

impl<'a, T> GetNode<'a, T> {

    pub fn new(id: usize) -> Self {
        Self {
            node: None,
            id, 
        }
    }

    pub fn get(&self) -> &Option<&'a T> {
        &self.node
    }

    fn set_node(&mut self, node: &'a T) {
        self.node = Some(node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Module<P>> {
    fn visit_module(&mut self, node: &'a Module<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_module(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, TopLevelDecl<P>> {
    fn visit_top_level_decl(&mut self, node: &'a TopLevelDecl<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_top_level_decl(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncDecl<P>> {
    fn visit_func_decl(&mut self, node: &'a FuncDecl<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_decl(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncDef<P>> {
    fn visit_func_def(&mut self, node: &'a FuncDef<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_def(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, InfixDef<P>> {
    fn visit_infix_def(&mut self, node: &'a InfixDef<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_infix_def(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncArg<P>> {
    fn visit_func_arg(&mut self, node: &'a FuncArg<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }    
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Ident<P>> {

    // todo refactor it later when func attr becomes 
    // its own type 
    fn visit_func_attr(&mut self, node: &'a Ident<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }

    fn visit_ident(&mut self, node: &'a Ident<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Expr<P>> {
    fn visit_expr(&mut self, node: &'a Expr<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_expr(self, node);
    }
}

//
// TODO BECAUSE BIN EXPR AND PARENTHESED ARE NO STRUCTS WE HAVE KIND
// OF A PROBLEM HERE
//

// impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Expr<P>> {
//     fn visit_expr(&mut self, node: &'a Expr<P>) {
//         if let ExprKind::Binary(ref id, ..) = node.kind {
//             if *id == self.id {
//                 self.set_node(node);
//                 return;
//             }
//         }
//         walk_expr(self, node);
//     }
// }

// impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncCall<P>> {
//     // the same as higher just trating function call as 
// }
