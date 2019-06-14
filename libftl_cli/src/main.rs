use std::cell::RefCell;
use std::rc::Rc;

use ftl_source::string::String;
use ftl_error::Handler;
use ftl_lexer::Lexer;


fn main() {
    let src = Rc::new(RefCell::new(String::from("raw: &str 0")));
    let handler = Rc::new(RefCell::new(Handler::new(src.clone())));
    let mut lexer = Lexer::new(src, handler.clone());
    println!("CREATED NEW LEXER!");
    for i in 0..10 {
        println!("Reading token {}!", i);
        println!{"Read token: {:?}", lexer.next()};
    }
    println!("{}", handler.borrow().error_msg().unwrap());
}