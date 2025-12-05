#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleReverse` trait.

#[tuplities_derive::impl_reverse]
/// A trait for reversing the elements of a tuple.
///
/// This trait provides a method to reverse the order of elements in a tuple.
///
/// # Examples
///
/// ```rust
/// use tuplities_reverse::TupleReverse;
///
/// let tuple = (1, "hello", 3.14);
/// let reversed = tuple.reverse();
/// assert_eq!(reversed, (3.14, "hello", 1));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleReverse {
    /// The type of the tuple with elements in reverse order.
    type Output;

    /// Consumes the tuple and returns a new tuple with elements in reverse order.
    fn reverse(self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::TupleReverse;

    #[test]
    fn test_reverse_empty_tuple() {
        let tuple = ();
        let _reversed: () = tuple.reverse();
    }

    #[test]
    fn test_reverse_single_element() {
        let tuple = (42,);
        let reversed = tuple.reverse();
        assert_eq!(reversed, (42,));
    }

    #[test]
    fn test_reverse_two_elements() {
        let tuple = (1, 2);
        let reversed = tuple.reverse();
        assert_eq!(reversed, (2, 1));
    }

    #[test]
    fn test_reverse_three_elements() {
        let tuple = (1, 2, 3);
        let reversed = tuple.reverse();
        assert_eq!(reversed, (3, 2, 1));
    }
}
