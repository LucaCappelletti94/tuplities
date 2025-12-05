#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleOrd` trait.

#[tuplities_derive::impl_tuple_ord]
/// A trait for comparing tuples for total ordering.
pub trait TupleOrd {
    /// Returns the total ordering of `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_ord::TupleOrd;
    /// use core::cmp::Ordering;
    ///
    /// let tuple1 = (1, 2);
    /// let tuple2 = (1, 3);
    /// let tuple3 = (2, 1);
    ///
    /// assert_eq!(tuple1.tuple_cmp(&tuple2), Ordering::Less);
    /// assert_eq!(tuple1.tuple_cmp(&tuple3), Ordering::Less);
    /// assert_eq!(tuple1.tuple_cmp(&tuple1), Ordering::Equal);
    /// ```
    fn tuple_cmp(&self, other: &Self) -> core::cmp::Ordering;
}
