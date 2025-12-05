#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePushFront` trait.

#[tuplities_derive::impl_push_front]
/// A trait for tuples that allows pushing an element to the front.
pub trait TuplePushFront<T> {
    /// The type of the tuple after adding `T` to the front.
    type Output;

    /// Consumes the tuple and prepends the given value, returning the new tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_push_front::TuplePushFront;
    ///
    /// let tuple = ("world",);
    /// let new_tuple = tuple.push_front("hello");
    /// assert_eq!(new_tuple, ("hello", "world"));
    /// ```
    fn push_front(self, value: T) -> Self::Output;
}
