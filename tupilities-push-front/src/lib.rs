#![no_std]

//! Tupilities suite crate providing the `PushFront` trait.

#[tupilities_derive::impl_push_front]
/// A trait for tuples that allows pushing an element to the front.
pub trait PushFront<T> {
    /// The type of the tuple after adding `T` to the front.
    type Output;

    /// Consumes the tuple and prepends the given value, returning the new tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_push_front::PushFront;
    ///
    /// let tuple = ("world",);
    /// let new_tuple = tuple.push_front("hello");
    /// assert_eq!(new_tuple, ("hello", "world"));
    /// ```
    fn push_front(self, value: T) -> Self::Output;
}
