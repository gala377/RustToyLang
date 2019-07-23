use std::io;

use simplelog::*;

use ftl_source::string::String;
use ftl_lexer::Lexer;
use ftl_session::{
    Session, 
    Emitter,
};
use ftl_utility::RcRef;
use ftl_parser::{
    Parser,
};

mod helpers;
mod phase;

use helpers::*;
use phase::*;

static SOURCE: &str = r#"
    decl nop (lang_nop) : void
    
    decl add int int (lang_add inline) : int
    infix 5 + a b: @add a b

    decl foo int int: int
    def foo a b: a + b

    def bar: 1 -- 2 <=> 3 `foo_bar 4 $ 5 % 0
    
    def foo_bar: @bar @@ 1 + 2 + @foo 3 (2+2*2) $ 2

    infix 5 @@ a b: a + b
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
    
    let mut ppp = phase::ppp::PrettyPrint{};
    ppp.run_wrapped(&mut ast);

    print_errors(&emmiter)?;
    
    print_line();
    print_green("✔ Done");
    print_line();
    
    Ok(())
}

fn create_sess() -> RcRef<Session<ftl_source::string::String>> {
    print!("\n");
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
    CombinedLogger::init(
        vec![
            TermLogger::new(filter, Config::default()).unwrap(),
        ]
    ).unwrap();
}