// todo definition mergin pass

use std::collections::HashMap;

use ftl_parser::visitor::*;
use ftl_parser::visitor_mut::*;
use ftl_parser::ast::*;

use ftl_source::{
    Pointer,
    Source,
};

use ftl_session::Session;

pub struct DefMerge<'a, 's, S: Source> {
    decl: HashMap<String, &'a FuncDecl<S::Pointer>>,
    sess: &'s mut Session<S>,
}

impl<'a, 's, S: Source> DefMerge<'a, 's, S> {
    pub fn new(sess: &'s mut Session<S>) -> Self {
        Self{
            decl: HashMap::new(),
            sess,
        }
    }

    pub fn insert(&mut self, key: String, decl: &'a FuncDecl<S::Pointer>) {
        self.decl.insert(key, decl);
    }
}
