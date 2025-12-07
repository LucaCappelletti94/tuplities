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
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn pop_front(self) -> (Self::Front, Self::Tail);
}

/// A trait for accessing a reference to the first element of a tuple.
///
/// This trait provides a method to get a reference to the first element without consuming the tuple.
///
/// # Examples
///
/// ```rust
/// use tuplities_pop_front::TupleRefFront;
///
/// let tuple = (1, 2, 3);
/// let first = tuple.ref_front();
/// assert_eq!(*first, 1);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleRefFront: TuplePopFront {
    /// Returns a reference to the first element of the tuple.
    fn ref_front(&self) -> &Self::Front;
}

/// A trait for accessing a mutable reference to the first element of a tuple.
///
/// This trait provides a method to get a mutable reference to the first element without consuming the tuple.
///
/// # Examples
///
/// ```rust
/// use tuplities_pop_front::TupleMutFront;
///
/// let mut tuple = (1, 2, 3);
/// let first = tuple.mut_front();
/// *first = 42;
/// assert_eq!(tuple, (42, 2, 3));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleMutFront: TupleRefFront {
    /// Returns a mutable reference to the first element of the tuple.
    fn mut_front(&mut self) -> &mut Self::Front;
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
