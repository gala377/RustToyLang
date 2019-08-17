use ftl_parser::ast::*;
use ftl_parser::visitor::*;

use ftl_source::Pointer;

/// Traverses the syntax tree in search of
/// the node with the given id.
/// If exists holds a reference to it.
///
/// Type of the searched node should be passed
/// as the generic argument.
pub struct GetNode<'a, T> {
    node: Option<&'a T>,
    id: usize,

    run_already: bool,
}

impl<'a, T> GetNode<'a, T> {
    /// Returns new GetNode pass which upon
    /// visiting the syntax tree will search for the
    /// node with the passed id.
    pub fn new(id: usize) -> Self {
        Self {
            node: None,
            id,
            run_already: false,
        }
    }

    /// If the node with the given id was found
    /// returns Some with the reference to it.
    /// If not None is returned.
    ///
    /// This method should be run only after a syntax tree
    /// has already been visited. Otherwise it panics.
    pub fn get(&self) -> &Option<&'a T> {
        if !self.run_already {
            panic!(
                "Accessing result of the GetNode pass should only be done after 
                visiting a sytax tree."
            );
        }
        &self.node
    }

    /// Sets the nodes value for Some with the given reference.
    fn set_node(&mut self, node: &'a T) {
        self.node = Some(node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Module<P>> {
    fn visit_module(&mut self, node: &'a Module<P>) {
        self.run_already = true;
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_module(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, TopLevelDecl<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_top_level_decl(&mut self, node: &'a TopLevelDecl<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_top_level_decl(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncDecl<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_decl(&mut self, node: &'a FuncDecl<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_decl(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncDef<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_def(&mut self, node: &'a FuncDef<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_def(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, InfixDef<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_infix_def(&mut self, node: &'a InfixDef<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_infix_def(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncArg<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_arg(&mut self, node: &'a FuncArg<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Ident<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_ident(&mut self, node: &'a Ident<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncAttr<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_attr(&mut self, node: &'a FuncAttr<P>) {
        if node.id == self.id {
            self.set_node(node);
        }
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Expr<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_expr(&mut self, node: &'a Expr<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_expr(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, InfixFuncCall<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_infix_func_call(&mut self, node: &'a InfixFuncCall<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_infix_func_call(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, InfixOpCall<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_infix_op_call(&mut self, node: &'a InfixOpCall<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_infix_op_call(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncCall<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_call(&mut self, node: &'a FuncCall<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_call(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Paren<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_parenthesed(&mut self, node: &'a Paren<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_paren_expr(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Lit<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_lit(&mut self, node: &'a Lit<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_lit(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Op<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_op(&mut self, node: &'a Op<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, Type<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_type(&mut self, node: &'a Type<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_type(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, FuncType<P>> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_func_type(&mut self, node: &'a FuncType<P>) {
        if node.id == self.id {
            self.set_node(node);
            return;
        }
        walk_func_type(self, node);
    }
}

impl<'a, P: Pointer> Pass<'a, P> for GetNode<'a, LitType> {
    fn visit_module(&mut self, _node: &'a Module<P>) {
        self.run_already = true;
    }

    fn visit_type(&mut self, node: &'a Type<P>) {
        if let TypeKind::Literal(ref lit) = node.kind {
            if node.id == self.id {
                self.set_node(lit);
                return;
            }
        }
        walk_type(self, node);
    }
}
