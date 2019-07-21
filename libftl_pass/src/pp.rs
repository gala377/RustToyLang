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
    
    #[allow(dead_code)]
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
    
    fn visit_infix_def(&mut self, node: &InfixDef<P>) {
        let repr = format!("Infix{} {}({}, {})",
            node.precedence,
            node.op.symbol,
            node.args.0.ident.symbol,
            node.args.1.ident.symbol);
        self.add(&repr);
        self.indent += 1;
        self.visit_expr(&node.body);
        self.indent -= 1;
    }

    fn visit_expr(&mut self, node: &Expr<P>) {
        walk_expr(self, node);
    }
    
    fn visit_infix_func_call(&mut self, ident: &Ident<P>, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add(&format!("InfCall {}", ident.symbol));
        self.start_line();
        self.indent += 1;
        walk_expr(self, lhs);
        self.start_line_at(self.indent-1);
        walk_expr(self, rhs);
        self.indent -= 1;
    }

    fn visit_func_call(&mut self, node: &FuncCall<P>) {
        self.add(&format!("FuncCall"));
        self.start_line();
        self.indent += 1;
        walk_expr(self, &node.lhs);
        for (i, arg) in node.args.iter().enumerate() {
            if i == node.args.len()-1 {
                self.stop_line_at(self.indent-1);
            }
            walk_expr(self, arg);
        }
        self.indent -= 1;
    }

    fn visit_bin_expr(&mut self, op: &Op<P>, lhs: &Expr<P>, rhs: &Expr<P>) {
        self.add(&format!("BinOp {}", op.symbol));
        self.start_line();
        self.indent += 1;
        walk_expr(self, lhs);
        self.stop_line_at(self.indent-1);
        walk_expr(self, rhs);
        self.indent -= 1;
    }

    fn visit_parenthesed(&mut self, node: &Expr<P>) {
        self.add("Parenthesed");
        self.indent += 1;
        walk_expr(self, node);
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