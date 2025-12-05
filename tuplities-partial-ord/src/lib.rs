#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePartialOrd` trait.

#[tuplities_derive::impl_tuple_partial_ord]
/// A trait for comparing tuples for partial ordering.
pub trait TuplePartialOrd {
    /// Returns the partial ordering of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_partial_ord::TuplePartialOrd;
    /// use core::cmp::Ordering;
    ///
    /// let tuple1 = (1, 2);
    /// let tuple2 = (1, 3);
    /// let tuple3 = (2, 1);
    ///
    /// assert_eq!(tuple1.tuple_partial_cmp(&tuple2), Some(Ordering::Less));
    /// assert_eq!(tuple1.tuple_partial_cmp(&tuple3), Some(Ordering::Less));
    /// assert_eq!(tuple1.tuple_partial_cmp(&tuple1), Some(Ordering::Equal));
    /// ```
    fn tuple_partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>;
}
