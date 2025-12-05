#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleCopy` trait.

#[tuplities_derive::impl_tuple_copy]
/// A trait for copying tuples.
pub trait TupleCopy {
    #[must_use]
    /// Copies `self` into a new instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_copy::TupleCopy;
    ///
    /// let tuple = (1, "hello", 3.14);
    /// let copied_tuple = tuple.tuple_copy();
    ///
    /// assert_eq!(tuple, copied_tuple);
    /// ```
    fn tuple_copy(&self) -> Self;
}
