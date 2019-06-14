use ftl_source::string::String;
use ftl_lexer::Lexer;
use ftl_session::Session;
use ftl_utility::RcRef;

fn main() {
    let sess = RcRef::new(Session::new(String::from("raw: &str 0")));
    let mut lexer = Lexer::new(sess.clone());
    println!("CREATED NEW LEXER!");
    for i in 0..10 {
        println!("Reading token {}!", i);
        println!{"Read token: {:?}", lexer.next()};
    }
    println!("{}", sess.borrow_mut().handler.error_msg().unwrap());
}