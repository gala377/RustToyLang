use std::io::Write;
use std::io;

use std::collections::BTreeSet;

use ftl_parser::visitor::*;
use ftl_parser::ast::*;

use ftl_source::Pointer;

/// Pretty prints visited syntax tree. 
/// UTF-8 support in terminal is required. 
#[derive(Default)]
pub struct Printer {
    res: String,

    indent: usize,
    draw_line_at_indent: BTreeSet<usize>,

    run_already: bool,
}

impl Printer {

    /// Creates new pass ready to be run on
    /// a syntax tree. 
    pub fn new() -> Self {
        Printer{ 
            res: String::new(),
            indent: 0,
            draw_line_at_indent: BTreeSet::new(),
            run_already: false,
        }
    }

    /// Writes syntax tree representation to the writer. 
    /// This method should be invoced only after the visiting has been done. 
    /// If the pass hasn't visited any syntax tree yet it panics.
    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if !self.run_already {
            panic!("Printer needs to visit a syntax tree before writing its output.")
        }
        write!(w, "{}", self.res)
    }

    /// Restores default state (except the `run_already` flag).
    fn clear(&mut self) {
        self.res.clear();
        self.indent = 0;
        self.draw_line_at_indent.clear();
    }

    /// Adds identation and proper leading lines to the passed string.
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

    /// After adding indention to the passed string adds it to
    /// the final output.
    fn add(&mut self, s: &str) {
        self.res += &self.with_indent(s);
        self.res += "\n";
    }

    /// Starts drawing vertical line at current indention. 
    fn start_line(&mut self) {
        self.draw_line_at_indent.insert(self.indent);
    }

    /// Starts drawing vertical line at given indention.
    #[allow(dead_code)]
    fn start_line_at(&mut self, indent: usize) {
        self.draw_line_at_indent.insert(indent);
    }

    /// Stops drawing vertical line at current indention.
    #[allow(dead_code)]
    fn stop_line(&mut self) {
        self.draw_line_at_indent.remove(&self.indent);
    }

    /// Stops drawing vertival line at given indention.
    fn stop_line_at(&mut self, indent: usize) {
        self.draw_line_at_indent.remove(&indent);
    }

    /// Returns string representation of the type node. 
    fn strfy_type<P: Pointer>(ty: &Type<P>) -> String {
        match ty.kind {
            TypeKind::Literal(ref lit) => {
                match lit {
                    LitType::Int => String::from("int"),
                    LitType::Void => String::from("void"),
                }
            },
            TypeKind::Function(ref func_t) => {
                let mut repr = String::from("(");
                for arg in &func_t.args {
                    repr += &format!(" {}", Self::strfy_type(arg));
                } 
                repr += &format!("): {}", Self::strfy_type(&func_t.ret));
                repr 
            }
        } 
    }
 }

impl<P: Pointer> Pass<'_, P> for Printer {

    fn visit_module(&mut self, node: &Module<P>) {
        self.run_already = true;
        self.clear();
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

    fn visit_func_decl(&mut self, node: &FuncDecl<P>) {
        let mut repr = format!(
            "FuncDecl {} type({}) attrs(",
             node.ident.symbol,
             if let Some(ref ty) = node.ty {
                 Self::strfy_type(&ty)
             } else {
                 String::from("")
             }
        );
        for attr in &node.attrs {
            repr += &format!(" {},", &attr.ident.symbol)
        }
        repr += ")";
        self.add(&repr);
    }
    
    fn visit_func_def(&mut self, node: &FuncDef<P>) {
        let mut repr = format!("FuncDef {} args(", node.decl.ident.symbol);
        for arg in &node.args {
            repr += &format!(" {},", arg.ident.symbol);
        }
        repr += ")";
        self.add(&repr);
        self.indent += 1;
        self.visit_func_decl(&node.decl);
        self.visit_expr(&node.body);
        self.indent -= 1;

    }
    
    fn visit_infix_def(&mut self, node: &InfixDef<P>) {
        let repr = format!("Infix({}) {} args({}, {})",
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
    
    fn visit_infix_func_call(&mut self, node: &InfixFuncCall<P>) {
        self.add(&format!("InfCall {}", node.ident.symbol));
        self.start_line();
        self.indent += 1;
        self.visit_expr(&node.lhs);
        self.start_line_at(self.indent-1);
        self.visit_expr(&node.rhs);
        self.indent -= 1;
    }

    fn visit_func_call(&mut self, node: &FuncCall<P>) {
        self.add("FuncCall");
        self.start_line();
        self.indent += 1;
        self.visit_expr(&node.lhs);
        for (i, arg) in node.args.iter().enumerate() {
            if i == node.args.len()-1 {
                self.stop_line_at(self.indent-1);
            }
            self.visit_expr(arg);
        }
        self.indent -= 1;
    }

    fn visit_infix_op_call(&mut self, node: &InfixOpCall<P>) {
        self.add(&format!("BinOp {}", node.op.symbol));
        self.start_line();
        self.indent += 1;
        self.visit_expr(&node.lhs);
        self.stop_line_at(self.indent-1);
        self.visit_expr(&node.rhs);
        self.indent -= 1;
    }

    fn visit_parenthesed(&mut self,  node: &Paren<P>) {
        self.add("Parenthesed");
        self.indent += 1;
        self.visit_expr(&node.expr);
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