//! Generic utility structures and functions.
//!
//! Items in this module can be related to
//! the FTL project but not to the specific
//! part of it for example:
//! AST factory for testing purposes.

use std::{cell::RefCell, clone::Clone, ops::Deref, rc::Rc};

#[macro_use]
pub mod macros;
pub mod utf8;

/// Newtype for the `Rc<RefCell<T>>`.
///
/// Expresses mutable object with shared ownership.
/// Used mostly because writing `Rc<RefCell<T>>` is tedious.
///
/// Mind that because the internals use
/// [`RefCell`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
/// to allow for the mutability of the contents, the borrowing rules
/// are enforced on the runtime rather than the compilation
/// so borrowing can panic. As with the
/// [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
/// [`try_borrow`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#try_borrow)
/// and
/// [`try_borrow_mut`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#try_borrow_mut)
/// might be used to avoid unexpected panics.
///
/// Implements
/// [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
/// trait and dereferences to
/// [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
/// so that the
/// [`borrow`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#borrow_mut)
/// and
/// [`borrow_mut`](https://doc.rust-lang.org/std/cell/struct.RefCell.html#borrow_mut)
/// methods can be called directly on the object.
///
/// Implements
/// [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
/// trait which creates new
/// instance with cloned
/// [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc)
/// so it works just like normal
/// [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc)
/// would.
///
/// # Examples
///
/// Creating new RcRef and cloning sharing ownership:
///
/// ```
/// use ftl_utility::RcRef;
///
/// let counter: RcRef<usize> = RcRef::new(0);
/// let shared_counter = counter.clone();
/// ```
///
/// Borrowing content:
///
/// ```
/// # use ftl_utility::RcRef;
/// #
/// # let counter: RcRef<usize> = RcRef::new(0);
/// # let shared_counter = counter.clone();
/// {
///     let mut val = shared_counter.borrow_mut();
///     *val += 1;
/// }
/// {
///     let mut val = counter.borrow_mut();
///     *val += 1;
/// }
/// ```
pub struct RcRef<T>(Rc<RefCell<T>>);

impl<T> RcRef<T> {
    /// Creates new `RcRef` with its content
    /// being `val`.
    ///
    /// # Examples
    /// ```
    /// use ftl_utility::RcRef;
    ///
    /// let rc_ref = RcRef::new(0);
    /// ```
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

// FIXME: Trivial implementation. Possibly could be derived.
impl<T> Clone for RcRef<T> {
    fn clone(&self) -> Self {
        RcRef(self.0.clone())
    }
}
