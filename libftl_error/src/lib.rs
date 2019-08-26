use ftl_source::{Pointer, Source};
use ftl_utility::RcRef;

pub trait LangError {
    type Ptr: Pointer;

    fn desc(&self) -> String;
    fn begin(&self) -> &Self::Ptr;
    fn end(&self) -> &Self::Ptr;

    fn span(&self) -> (&Self::Ptr, &Self::Ptr) {
        (self.begin(), self.end())
    }
}

pub struct Handler<S: Source> {
    errs: Vec<Box<dyn LangError<Ptr = S::Pointer>>>,
    src: RcRef<S>,
}

impl<S, P> Handler<S>
where
    P: Pointer,
    S: Source<Pointer = P>,
{
    pub fn new(src: RcRef<S>) -> Self {
        Self {
            errs: Vec::new(),
            src,
        }
    }

    pub fn err(&mut self, err: Box<dyn LangError<Ptr = P>>) {
        self.errs.push(err);
    }

    pub fn fatal(&mut self, err: Box<dyn LangError<Ptr = P>>) -> ! {
        self.err(err);
        if let Some(msg) = self.error_msg() {
            println!("{}", msg);
        }
        panic!("Fatal error");
    }

    pub fn error_msg(&self) -> Option<String> {
        if self.errs.is_empty() {
            return None;
        }
        let mut mess = String::new();
        for err in &self.errs {
            let err_msg = self.err_to_str(err.as_ref());
            mess += &err_msg;
        }
        Some(mess)
    }

    fn err_to_str(&self, err: &dyn LangError<Ptr = P>) -> String {
        format!(
            "[{}:{}] {}\n\n{}\n\n",
            err.begin().line(),
            err.begin().position(),
            err.desc(),
            self.src.borrow().source_between(err.begin(), err.end())
        )
    }
}
