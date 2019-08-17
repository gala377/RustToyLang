use ftl_parser::{ast::AST, visitor::visit_ast};
use ftl_pass::pp;
use ftl_source::Source;

use crate::phase::Phase;

pub struct PrettyPrint;

impl<S: Source> Phase<S> for PrettyPrint {
    fn msg_done() -> &'static str {
        "Done..."
    }

    fn msg_init() -> &'static str {
        "🐌 Applying PrettyPrintPass..."
    }

    fn run(&mut self, ast: &mut AST<S>) {
        let mut ppp = pp::Printer::new();
        visit_ast(&mut ppp, &ast);

        let mut out = std::io::stdout();
        ppp.write(&mut out).unwrap();
    }
}
