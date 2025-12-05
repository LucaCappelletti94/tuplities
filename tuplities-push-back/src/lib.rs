#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePushBack` trait.

#[tuplities_derive::impl_push_back]
/// A trait for tuples that allows pushing an element to the back.
pub trait TuplePushBack<T> {
    /// The type of the tuple after adding `T` to the back.
    type Output;

    /// Consumes the tuple and appends the given value, returning the new tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_push_back::TuplePushBack;
    ///
    /// let tuple = ("hello",);
    /// let new_tuple = tuple.push_back("world");
    /// assert_eq!(new_tuple, ("hello", "world"));
    /// ```
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn push_back(self, value: T) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::TuplePushBack;

    #[test]
    fn test_push_back_zero_sized_tuple() {
        let tuple = ();
        let result = tuple.push_back(42);
        // Check that the type is correct: () + i32 = (i32,)
        let expected: (i32,) = (42,);
        assert_eq!(result, expected);
    }
}
