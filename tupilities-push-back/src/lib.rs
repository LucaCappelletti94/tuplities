#![no_std]

//! Tupilities suite crate providing the `PushBack` trait.

#[tupilities_derive::impl_push_back]
/// A trait for tuples that allows pushing an element to the back.
pub trait PushBack<T> {
    /// The type of the tuple after adding `T` to the back.
    type Output;

    /// Consumes the tuple and appends the given value, returning the new tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_push_back::PushBack;
    ///
    /// let tuple = ("hello",);
    /// let new_tuple = tuple.push_back("world");
    /// assert_eq!(new_tuple, ("hello", "world"));
    /// ```
    fn push_back(self, value: T) -> Self::Output;
}
