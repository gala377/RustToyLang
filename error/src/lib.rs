use source::Pointer;

pub mod handler;

pub trait LangError {

    type Ptr: Pointer;

    fn desc(&self) -> &str;
    fn begin(&self) -> &Self::Ptr; 
    fn end(&self) -> &Self::Ptr;

    fn span(&self) -> (&Self::Ptr, &Self::Ptr) {
        (self.begin(), self.end())
    }
}