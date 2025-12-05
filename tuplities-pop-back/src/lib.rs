#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePopBack` trait.

#[tuplities_derive::impl_pop_back]
/// A trait for tuples that defines the `Init` and `Back` types, and provides a method to pop the back element.
pub trait TuplePopBack {
    /// The type of the tuple without the last element.
    type Init;

    /// The type of the last element.
    type Back;

    /// Consumes the tuple and returns the initial part and the last element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_pop_back::TuplePopBack;
    ///
    /// let tuple = (1, 2, 3);
    /// let (front, back) = tuple.pop_back();
    /// assert_eq!(front, (1, 2));
    /// assert_eq!(back, 3);
    /// ```
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn pop_back(self) -> (Self::Init, Self::Back);
}

#[cfg(test)]
mod tests {
    use super::TuplePopBack;

    #[test]
    fn test_pop_back_single_element_tuple() {
        let tuple = (42,);
        let (_init, back): ((), i32) = tuple.pop_back();
        // Check that the types are correct: (T,) -> Init = (), Back = T
        let expected_back: i32 = 42;
        assert_eq!(back, expected_back);
    }

    #[test]
    fn test_pop_back_two_element_tuple() {
        let tuple = (1, 2);
        let (init, back) = tuple.pop_back();
        // Check that the types are correct: (T1, T2) -> Init = (T1,), Back = T2
        let expected_init: (i32,) = (1,);
        let expected_back: i32 = 2;
        assert_eq!(init, expected_init);
        assert_eq!(back, expected_back);
    }
}
