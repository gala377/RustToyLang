use std::io;

use termion::{
    style,
    color,
};
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
    visitor::{
        visit_ast,
    },
};
use ftl_pass::{
    pp,
};


static SOURCE: &str = r#"
    def  nop (lang_nop) : 0
    def add a b (lang_add inline hide) : @nop

    infix 5 + a b: @add a b

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
    let mut out = std::io::stdout();
    print!("\n");
    print_red("🦊 Compilation starts...");
    print_line();    
    print_red("🦒 Creating source and session...");
    let sess = RcRef::new(Session::new(String::from(SOURCE)));
    
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
    print_red("🦂 Creating passes...");

    print_red("🐌 Creating PrettyPrintPass...");
    let mut ppp = pp::Printer::new();
    print_green("✔ Done");

    print_green("✔ All passes created");


    print_line();
    print_red("🐉 Parsing source...");
    let ast = parser.parse();
    
    print_green("✔ Source parsed");
    print_line();
    print_red("🦖 Applying passes...");
    print_red("🦋 PrettyPrintPass...");
    visit_ast(&mut ppp, &ast);
    print_green("✔ Done...");


    print_line();
    print_red("🐺 Printing errors...");
    print_line();
    emmiter.emit_err(&mut out)?;
    print_line();
    print_green("✔ Done");

    print_line();
    print_red("🐙 Printing PrettyPrintPass output to stdout...");
    print_line();
    ppp.write(&mut out)?;
    
    print_line();
    print_green("✔ Printed");
    print_line();
    print_green("✔ Done");
    print_line();
    Ok(())
}

fn print_red(s: &str) {
    println!("\t{}{}{}{}", style::Bold, color::Fg(color::Red), s, style::Reset);
}

fn print_green(s: &str) {
    println!("\t{}{}{}{}", style::Bold, color::Fg(color::Green), s, style::Reset);
}

fn print_line() {
    print!("{}{}\n", style::Bold, color::Fg(color::Yellow));
    for _ in 0..100 {
        print!("=");
    }
    print!("\n\n{}", style::Reset);
}


fn init_logger(filter: LevelFilter) {
    CombinedLogger::init(
        vec![
            TermLogger::new(filter, Config::default()).unwrap(),
        ]
    ).unwrap();
}