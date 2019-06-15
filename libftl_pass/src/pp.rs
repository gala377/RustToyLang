use std::io::Write;
use std::io;

use std::collections::BTreeSet;

use ftl_parser::visitor::*;
use ftl_parser::ast::*;

use ftl_source::Pointer;

pub struct Printer {
    res: String,

    indent: usize,
    draw_line_at_indent: BTreeSet<usize>
}

impl Printer {

    pub fn new() -> Self {
        Printer{ 
            res: String::new(),
            indent: 0,
            draw_line_at_indent: BTreeSet::new(),
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self.res)
    }

    fn with_indent(&self, s: &str) -> String {
        let mut res = String::new();
        for i in 0..self.indent {
            res += "        ";
            res += if self.draw_line_at_indent.contains(&i) {
                "║"
            } else {
                " "
            }
            
        }
        res.pop();
        res += "╚═════>";
        format!("{}{}", res, s)
    }

    fn add(&mut self, s: &str) {
        self.res += &self.with_indent(s);
        self.res += "\n";
    }

    fn start_line(&mut self) {
        self.draw_line_at_indent.insert(self.indent);
    }

    #[allow(dead_code)]
    fn start_line_at(&mut self, indent: usize) {
        self.draw_line_at_indent.insert(indent);
    }

    fn stop_line(&mut self) {
        self.draw_line_at_indent.remove(&self.indent);
    }

    fn stop_line_at(&mut self, indent: usize) {
        self.draw_line_at_indent.remove(&indent);
    }

 }

impl<P: Pointer> Pass<P> for Printer {

    fn visit_module(&mut self, node: &Module<P>) {
        self.add("Module");
        self.start_line();
        self.indent += 1;
        for (i, decl) in node.decl.iter().enumerate() {
            if i == node.decl.len()-1 {
                self.stop_line_at(self.indent-1);
            }
            self.visit_top_level_decl(decl);
        }
        self.indent -= 1;
    }
    
    fn visit_func_def(&mut self, node: &FuncDef<P>) {
        let mut repr = format!("FuncDef {}", node.ident.symbol);
        for arg in &node.args {
            repr += &format!(" {}", arg.ident.symbol);
        }
        self.add(&repr);
        self.indent += 1;
        self.visit_expr(&node.body);
        self.indent -= 1;

    }
    
    fn visit_expr(&mut self, node: &Expr<P>) {
        walk_expr(self, node);
    }
    
    fn visit_bin_addition(&mut self, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add("AddExpr");
        self.start_line();
        self.indent += 1;
        walk_expr(self, lhs);
        walk_expr(self, rhs);
        self.indent -= 1;
        self.stop_line();
    }

    fn visit_bin_substraction(&mut self, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add("SubExpr");
        self.start_line();
        self.indent += 1;
        walk_expr(self, lhs);
        walk_expr(self, rhs);
        self.indent -= 1;
        self.stop_line();
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