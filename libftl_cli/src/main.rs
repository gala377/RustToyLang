use std::io;

use termion::{
    style,
    color,
};

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
    def foo a b: 5

    def bar: 3 + 5
    
    def foo_bar: bar
"#;

fn main() -> io::Result<()> {
    let mut out = std::io::stdout();
    
    print_red("\nCompilation starts...");
    print_line();    
    print_red("Creating source and session...");
    let sess = RcRef::new(Session::new(String::from(SOURCE)));
    
    print_green("Created");
    print_line();
    print_red("Creating Lexer...");
    let lexer = Lexer::new(sess.clone());
    
    print_green("Created");
    print_line();    

    print_red("Creating parser...");
    let mut parser = Parser::new(lexer, sess.clone());
    
    print_green("Created");
    print_line();
    print_red("Creating error emitter...");
    let emmiter = Emitter::new(sess.clone());
    
    print_green("Created");
    print_line();
    print_red("Creating ppp...");
    let mut ppp = pp::Printer::new();
    
    print_green("Created");
    print_line();
    print_red("Parsing source...");
    let ast = parser.parse();
    
    print_green("Source parsed");
    print_line();
    print_red("Printing errors...");
    print_line();
    emmiter.emit_err(&mut out)?;
    print_line();
    print_green("Done");
    print_line();

    print_red("Walking ast with ppp...");
    visit_ast(&mut ppp, &ast);
    
    print_green("Done");
    print_line();
    print_red("Printing output to stdout...");
    print_line();
    ppp.write(&mut out)?;
    
    print_line();
    print_green("Printed");
    print_line();
    print_green("Done");
    
    Ok(())
}

fn print_red(s: &str) {
    println!("{}{}{}{}", style::Bold, color::Fg(color::Red), s, style::Reset);
}

fn print_green(s: &str) {
    println!("{}{}{}{}", style::Bold, color::Fg(color::Green), s, style::Reset);
}

fn print_line() {
    print!("\n");
    for _ in 0..100 {
        print!("=");
    }
    print!("\n\n");
}