#[macro_use]
pub mod macros;

use std::{
    rc::Rc,
    cell::RefCell,
    ops::Deref,
    clone::Clone,
};

pub struct RcRef<T>(Rc<RefCell<T>>);

impl<T> RcRef<T> {

    pub fn new(val: T) -> Self {
        Self(Rc::new(RefCell::new(val)))
    }

}

impl<T> Deref for RcRef<T> {

    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

} 

impl<T> Clone for RcRef<T> {

    fn clone(&self) -> Self {
        RcRef(self.0.clone())
    }
}
