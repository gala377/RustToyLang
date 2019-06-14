
use ftl_utility::RcRef;
use ftl_lexer::Lexer;
use ftl_session::Session;
use ftl_source::Source;


pub struct Parser<S: Source> {

    sess: RcRef<Session<S>>,
    
    lexer: Lexer<S>,

}

impl<S: Source> Parser<S> {

    pub fn new(lexer: Lexer<S>, sess: RcRef<Session<S>>) -> Self {
        Self {
            lexer,
            sess,
        }
    }

}