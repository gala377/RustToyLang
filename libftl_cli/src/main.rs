use std::io;
use std::env;

use simplelog::*;

use ftl_lexer::Lexer;
use ftl_parser::Parser;
use ftl_session::{Emitter, Session};
use ftl_source::file::File;
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

fn create_sess() -> RcRef<Session<File>> {
    println!();
    print_red("🦊 Compilation starts...");
    print_line();
    print_red("🦒 Creating source and session...");
    let source = if let Some(file) = get_file() {
        file 
    } else {
        panic!("Empty source")
    };
    RcRef::new(Session::new(source))
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

fn get_file() -> Option<File> {
    let arg_iter = env::args();
    let mut path = None;
    let mut file_present = false;
    for arg in arg_iter {
        if file_present {
            file_present = false;
            path = Some(arg);
        } else if arg == "--file" {
            file_present = true;
        }
    }    
    if let Some(path) = path {
        Some(File::new(&path).unwrap())
    } else {
        None
    }
}