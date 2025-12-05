#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePartialEq` trait.

#[tuplities_derive::impl_tuple_partial_eq]
/// A trait for comparing tuples for partial equality.
pub trait TuplePartialEq {
    /// Returns `true` if `self` and `other` are equal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_partial_eq::TuplePartialEq;
    ///
    /// let tuple1 = (1, "hello");
    /// let tuple2 = (1, "hello");
    /// let tuple3 = (2, "world");
    ///
    /// assert!(tuple1.tuple_eq(&tuple2));
    /// assert!(!tuple1.tuple_eq(&tuple3));
    /// ```
    fn tuple_eq(&self, other: &Self) -> bool;
}
