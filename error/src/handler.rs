
use crate::LangError;
use source::{
    Source,
    Pointer,
};

pub struct Handler<S: Source> {
    errs: Vec<Box<dyn LangError<Ptr=S::Pointer>>>,
}

impl<S: Source> Handler<S> {

    pub fn  new() -> Self {
        Self {
            errs: Vec::new(),
        }
    }

    pub fn error(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>) {
        self.errs.push(err);
    }

    pub fn abort(&mut self, err: Box<dyn LangError<Ptr=S::Pointer>>, src: &S) {
        self.error(err);
        panic!(self.error_msg(src));
    }

    pub fn error_msg(&self, src: &S) -> Option<String> {
        if self.errs.is_empty() {
            return None; 
        }
        let mut mess = String::new();
        for err in &self.errs {
            let err_msg = Self::err_to_str(err.as_ref(), src);
            mess += &err_msg;
        }
        Some(mess)
    }

    fn err_to_str(err: &dyn LangError<Ptr=S::Pointer>, src: &S) -> String {
        String::from(
            format!(
                "[{}:{}] {}\n\n{}\n\n",
                err.begin().line(),
                err.begin().position(),
                err.desc(),
                src.source_between(err.begin(), err.end()))
        )
    }
}