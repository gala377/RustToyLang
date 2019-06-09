use std::cell;

use source::string::String;
use error::Handler;
use lexer::Lexer;


fn main() {
    let src = cell::RefCell::new(String::from("raw: &str 0"));
    let handler = cell::RefCell::new(Handler::new(&src));
    let mut lexer = Lexer::new(&src, &handler);
    println!("CREATED NEW LEXER!");
    for i in 0..10 {
        println!("Reading token {}!", i);
        println!{"Read token: {:?}", lexer.next()};
    }
    println!("{}", handler.borrow().error_msg().unwrap());
}