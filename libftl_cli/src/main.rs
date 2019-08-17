use std::io;

use simplelog::*;

use ftl_lexer::Lexer;
use ftl_parser::Parser;
use ftl_session::{Emitter, Session};
use ftl_source::string::String;
use ftl_utility::RcRef;

// test
use ftl_parser::visitor_mut::visit_ast_mut;
use ftl_pass::dm::DeclarationMerge;
use ftl_pass::epr::ExprPrecReassoc;
// test

mod helpers;
mod phase;

use helpers::*;
use phase::*;

static SOURCE: &str = r#"
    decl nop [lang_nop]: void

    decl add int int [lang_add inline]: int
    decl mult int int [lang_mult]: int

    infix 5 @@ a b: a + b
    infix 10 $ func expr: @func expr
    infix 50 - a b: @sub a b
    infix 50 + a b: @add a b
    infix 100 * a b: @mult a b

    def multiple a b c: a + b + c
    def call_mult: @multiple 1 2 3 + 2

    decl foo int int : int
    def foo a b: a + b

    def bar: 1 - 2 + 3 `foo_bar 4 $ 5 * 0

    def foo_bar: @bar @@ 1 + 2 + @foo 3 (2+2*2) $ 2

    def test: 2 + 2 * 2
    infix 5 <==> a b: 1 `foo 2 `foo 3 `foo 4 + (1)
    def test3 [test4 test1]: 2*2+2*2*2*2*2*2

    decl test3 [test1 test2]: int

    decl apply int (int)int [attr1]: (int) int
    decl rev_apply (int int)int (int)void: ()(int int)(int)int
"#;

fn main() -> io::Result<()> {
    init_logger(if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    });
    let sess = create_sess();

    print_green("✔ Created");
    print_line();
    print_red("🐿 Creating Lexer...");
    let lexer = Lexer::new(sess.clone());

    print_green("✔ Created");
    print_line();

    print_red("🐊 Creating parser...");
    let mut parser = Parser::new(lexer, sess.clone());

    print_green("✔ Created");
    print_line();
    print_red("🦎 Creating error emitter...");
    let emmiter = Emitter::new(sess.clone());

    print_green("✔ Created");

    print_line();
    print_red("🐉 Parsing source...");
    let mut ast = parser.parse();

    print_green("✔ Source parsed");

    let mut ppp = phase::ppp::PrettyPrint {};
    ppp.run_wrapped(&mut ast);

    print_line();
    print_red("Running EPR pass...");

    {
        let mut sess_ref = sess.borrow_mut();
        let mut epr = ExprPrecReassoc::new(&mut sess_ref);
        visit_ast_mut(&mut epr, &mut ast);
    }
    {
        let mut dm = DeclarationMerge::new();
        visit_ast_mut(&mut dm, &mut ast);
    }

    print_green("Done...");

    let mut ppp = phase::ppp::PrettyPrint {};
    ppp.run_wrapped(&mut ast);

    print_line();

    print_errors(&emmiter)?;

    print_line();
    print_green("✔ Done");
    print_line();

    Ok(())
}

fn create_sess() -> RcRef<Session<ftl_source::string::String>> {
    println!();
    print_red("🦊 Compilation starts...");
    print_line();
    print_red("🦒 Creating source and session...");
    RcRef::new(Session::new(String::from(SOURCE)))
}

fn print_errors<S: ftl_source::Source>(emmiter: &Emitter<S>) -> std::io::Result<()> {
    print_line();
    print_red("🐺 Printing errors...");
    print_line();
    let mut out = std::io::stdout();
    emmiter.emit_err(&mut out)?;
    print_line();
    print_green("✔ Done");
    Ok(())
}

fn init_logger(filter: LevelFilter) {
    CombinedLogger::init(vec![TermLogger::new(filter, Config::default()).unwrap()]).unwrap();
}
