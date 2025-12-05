#![no_std]

//! Tupilities suite crate providing the `TuplePartialEq` trait.

#[tupilities_derive::impl_tuple_partial_eq]
/// A trait for comparing tuples for partial equality.
pub trait TuplePartialEq {
    /// Returns `true` if `self` and `other` are equal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_partial_eq::TuplePartialEq;
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
