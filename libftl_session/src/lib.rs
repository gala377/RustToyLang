use ftl_error::Handler;
use ftl_source::{
    Source,
};
use ftl_utility::RcRef;

pub struct Session<S: Source> {
    pub handler: Handler<S>, 
    pub src: RcRef<S>,
}

impl<S: Source> Session<S> {
    
    pub fn new(src: S) -> Self {
        let src = RcRef::new(src);
        Session {
            src: src.clone(),
            handler: Handler::new(src.clone()),
        }
    }

}