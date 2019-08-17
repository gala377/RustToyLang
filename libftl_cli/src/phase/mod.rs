use ftl_parser::ast::AST;
use ftl_source::Source;

use crate::helpers::*;

pub mod ppp;

pub trait Phase<S: Source> {
    fn run(&mut self, ast: &mut AST<S>);

    fn msg_init() -> &'static str;
    fn msg_done() -> &'static str;

    fn run_wrapped(&mut self, ast: &mut AST<S>) {
        print_line();
        print_red(Self::msg_init());

        self.run(ast);
        print_green(&format!("✔ {}", Self::msg_done()));
    }
}
