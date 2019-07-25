// FIXME: WIP 
use log::debug;

use std::collections::{
    HashMap,
};

use ftl_parser::visitor::*;
use ftl_parser::visitor_mut::*;
use ftl_parser::ast::*;

use ftl_source::{
    Pointer,
    Source,
};

use ftl_session::Session;


pub struct DeclarationMerge<'s, S: Source> {
    sess: &'s mut Session<S>,
}

impl<'a, 's, S: Source> DeclarationMerge<'s, S> {
    pub fn new(sess: &'s mut Session<S>) -> Self {
        Self{
            sess,
        }
    }
}

impl<'a, 's, S: Source> MutPass<'a, S::Pointer> for DeclarationMerge<'s, S> where S::Pointer: 'static {

    fn visit_module(&mut self, node: &'a mut Module<S::Pointer>) {
        let mut merger = DefMerger::new(&mut self.sess);
        merger.visit_module(node);
        let decls_to_rem = merger.get();
        let mut remover = DeclRemover::new(decls_to_rem.values().map(|x| {x.id}).collect());
        remover.visit_module(node);
    }
}

struct DefMerger<'s, S: Source> {
    decls: HashMap<String, FuncDecl<S::Pointer>>,
    sess: &'s mut Session<S>,
}

impl<'a, 's, S: Source> DefMerger<'s, S> {
    pub fn new(sess: &'s mut Session<S>) -> Self {
        Self{
            decls: HashMap::new(),
            sess,
        }
    }

    fn merge(decl: &FuncDecl<S::Pointer>, def: &mut FuncDef<S::Pointer>) {
        def.decl.attrs.extend_from_slice(&decl.attrs);
        def.decl.attrs.sort_by(|a, b| { a.ident.symbol.cmp(&b.ident.symbol) });
        def.decl.attrs.dedup_by(|a, b| { a.ident.symbol == b.ident.symbol });

        def.decl.ty = decl.ty.clone();
        // FIXME: check if the type corresponds to the definition arguments
        // thats what we need session for actually, because this is where an 
        // error can occur.
    }

    fn get(self) -> HashMap<String, FuncDecl<S::Pointer>> {
        self.decls
    }
}

impl<'a, 's, S: Source> MutPass<'a, S::Pointer> for DefMerger<'s, S> where S::Pointer: 'static {

    fn visit_module(&mut self, node: &'a mut Module<S::Pointer>) {
        let mut decls = DeclCollector::new();
        decls.visit_module(node);
        self.decls = decls.get();
        noop_module(self, node);
        self.decls.retain(|_, v| {
            for attr in &v.attrs {
                if attr.ident.symbol == "remove" {
                    return true;
                }
            }
            false
        });
    }

    fn visit_func_def(&mut self, node: &'a mut FuncDef<S::Pointer>) {
        if let Some(ref mut decl) = self.decls.get_mut(&node.decl.ident.symbol) {
            Self::merge(&decl, node);
            decl.attrs.push(FuncAttr{
                id: 0,
                ident: Ident {
                    id: 0, 
                    symbol: "remove".to_owned(),
                    span: decl.ident.span.clone(),
                },
            })
        }
    }

}


/// Runs on the syntax tree once and collects 
/// map of top level function declaration identifier symbols to their
/// respective nodes copies.
struct DeclCollector<P: Pointer> {
    decls: HashMap<String, FuncDecl<P>>,
    run_already: bool,
}

impl<P: Pointer> DeclCollector<P> {
    pub fn new() -> Self {
        Self {
            decls: HashMap::new(),
            run_already: false,
        }
    }

    pub fn get(self) -> HashMap<String, FuncDecl<P>> {
        if !self.run_already {
            panic!("DeclCollector needs to visit a syntax tree before 
                returning its result.")
        }
        self.decls
    }
}

impl <'a, P: Pointer> Pass<'a, P> for DeclCollector<P> {
    fn visit_module(&mut self, node: &'a Module<P>) {
        self.run_already = true;
        walk_module(self, node);
    }

    fn visit_top_level_decl(&mut self, node: &'a TopLevelDecl<P>) {
        if let TopLevelDeclKind::FunctionDecl(ref decl) = node.kind {
            self.decls.insert(decl.ident.symbol.clone(), decl.clone());
        }
    }
}

struct DeclRemover {
    decls: Vec<NodeId>,
}

impl DeclRemover {
    fn new(decls: Vec<NodeId>) -> Self {
        Self {
            decls,
        }
    }
}

impl<'a, P: Pointer> MutPass<'a, P> for DeclRemover {

    fn visit_module(&mut self, node: &'a mut Module<P>) {
        for id in &self.decls {
            debug!("To remove decl id: {}", id);
        }
        node.decl.retain(|x| { 
            if let TopLevelDeclKind::FunctionDecl(ref func_decl) = x.kind {
                if self.decls.contains(&func_decl.id) {
                    debug!("Fund decl id {} removed", func_decl.id);
                    return false;
                }
            }
            return true;
        });
    }

}