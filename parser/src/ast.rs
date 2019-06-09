use source;


pub trait Node {}


pub struct AST {
    root: Box<dyn Node>,
}