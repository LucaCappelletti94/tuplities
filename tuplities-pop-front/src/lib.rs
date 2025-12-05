#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePopFront` trait.

#[tuplities_derive::impl_pop_front]
/// A trait for tuples that defines the `Front` and `Tail` types, and provides a method to pop the front element.
pub trait TuplePopFront {
    /// The type of the first element.
    type Front;

    /// The type of the tuple after removing the first element.
    type Tail;

    /// Consumes the tuple and returns the first element and the remaining tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_pop_front::TuplePopFront;
    ///
    /// let tuple = (1, 2, 3);
    /// let (first, rest) = tuple.pop_front();
    /// assert_eq!(first, 1);
    /// assert_eq!(rest, (2, 3));
    /// ```
    fn pop_front(self) -> (Self::Front, Self::Tail);
}

#[cfg(test)]
mod tests {
    use super::TuplePopFront;

    #[test]
    fn test_pop_front_single_element_tuple() {
        let tuple = (42,);
        let (front, _tail): (i32, ()) = tuple.pop_front();
        // Check that the types are correct: (T,) -> Front = T, Tail = ()
        let expected_front: i32 = 42;
        assert_eq!(front, expected_front);
    }

    #[test]
    fn test_pop_front_two_element_tuple() {
        let tuple = (1, 2);
        let (front, tail) = tuple.pop_front();
        // Check that the types are correct: (T1, T2) -> Front = T1, Tail = (T2,)
        let expected_front: i32 = 1;
        let expected_tail: (i32,) = (2,);
        assert_eq!(front, expected_front);
        assert_eq!(tail, expected_tail);
    }
}
