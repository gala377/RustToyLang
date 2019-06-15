use std::fmt;
use std::io::Write;
use std::io;

use ftl_parser::visitor::*;
use ftl_parser::ast::*;

use ftl_source::Pointer;

pub struct Printer {
    res: String,

    indent: usize,
}

impl Printer {

    pub fn new() -> Self {
        Printer{ 
            res: String::new(),
            indent: 0,
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self.res)
    }

    fn with_indent(&self, s: &str) -> String {
        let mut res = String::new();
        for _ in 0..self.indent {
            res += "\t";
        }
        format!("{}{}", res, s)
    }

    fn add(&mut self, s: &str) {
        self.res += &self.with_indent(s);
    }
}

impl<P: Pointer> Pass<P> for Printer {

    fn visit_module(&mut self, node: &Module<P>) {
        self.add("Module");
        self.indent += 1;
        for decl in &node.decl {
            self.visit_top_level_decl(decl);
        }
        self.indent -= 1;
    }
    
    fn visit_func_def(&mut self, node: &FuncDef<P>) {
        self.add(&format!("FuncDef {}", node.ident.symbol));
        for arg in &node.args {
            self.res += &format!(" {}", arg.ident.symbol);
        }
        self.indent += 1;
        self.visit_expr(&node.body);
        self.indent -= 1;

    }
    
    fn visit_expr(&mut self, node: &Expr<P>) {
        walk_expr(self, node);
    }
    
    fn visit_bin_addition(&mut self, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add("AddExpr");
        self.indent += 1;
        walk_expr(self, lhs);
        walk_expr(self, rhs);
        self.indent -= 1;
    }

    fn visit_bin_substraction(&mut self, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add("SubExpr");
        self.indent += 1;
        walk_expr(self, lhs);
        walk_expr(self, rhs);
        self.indent -= 1;
    }
    
    fn visit_int_lit(&mut self, val: u64) {
        self.add(&format!("Int: {}", val))
    }

    fn visit_ident(&mut self, node: &Ident<P>) {
        self.add(&format!("Ident: {}", node.symbol))
    }

    fn nop(&mut self) {
        unimplemented!();
    }

}