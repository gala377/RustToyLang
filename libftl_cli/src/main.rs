use std::io;

use ftl_source::string::String;
use ftl_lexer::Lexer;
use ftl_session::{Session, Emitter};
use ftl_utility::RcRef;

fn main() -> io::Result<()> {

    let sess = RcRef::new(Session::new(String::from("raw: &str 0")));
    let mut lexer = Lexer::new(sess.clone());
    let emmiter = Emitter::new(sess.clone());

    println!("CREATED NEW LEXER!");
    for i in 0..10 {
        println!("Reading token {}!", i);
        println!{"Read token: {:?}", lexer.next()};
    }
    println!("\n\n\n");

    let mut out = std::io::stdout();
    emmiter.emit_err(&mut out)?;

    Ok(())
}