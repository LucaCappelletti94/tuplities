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
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn push_front(self, value: T) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::TuplePushFront;

    #[test]
    fn test_push_front_zero_sized_tuple() {
        let tuple: () = ();
        let result: (i32,) = tuple.push_front(42);
        // Check that the type is correct: i32 + () = (i32,)
        let expected: (i32,) = (42,);
        assert_eq!(result, expected);
    }
}
