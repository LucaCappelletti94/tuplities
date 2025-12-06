#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleReplicate` trait.

#[tuplities_derive::impl_tuple_replicate]
/// A trait for replicating a value into a tuple.
///
/// This trait allows creating a tuple where all elements are the same value.
/// The implementation is optimized to avoid unnecessary cloning: for empty tuples
/// and single-element tuples, no `Clone` bound is required. For tuples with 2+
/// elements, `Clone` is required but the original value is moved to the last
/// position to minimize clones.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleReplicate<T> {
    /// Creates a tuple where all elements are the provided value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_replicate::TupleReplicate;
    ///
    /// let tuple: (i32, i32, i32) = TupleReplicate::tuple_replicate(42);
    /// assert_eq!(tuple, (42, 42, 42));
    /// ```
    fn tuple_replicate(value: T) -> Self;
}
