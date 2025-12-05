#![no_std]

//! tuplities suite crate providing the `PopFront` trait.

#[tuplities_derive::impl_pop_front]
/// A trait for tuples that defines the `Front` and `Tail` types, and provides a method to pop the front element.
pub trait PopFront {
    /// The type of the first element.
    type Front;

    /// The type of the tuple after removing the first element.
    type Tail;

    /// Consumes the tuple and returns the first element and the remaining tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_pop_front::PopFront;
    ///
    /// let tuple = (1, 2, 3);
    /// let (first, rest) = tuple.pop_front();
    /// assert_eq!(first, 1);
    /// assert_eq!(rest, (2, 3));
    /// ```
    fn pop_front(self) -> (Self::Front, Self::Tail);
}
