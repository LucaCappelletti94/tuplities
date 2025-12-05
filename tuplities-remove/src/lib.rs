//! A trait for removing elements at specific indices from tuples.
//!
//! This crate provides the `TupleRemove<Idx>` trait, which allows removing an element
//! at a compile-time known index from a tuple, returning the element and the
//! remaining tuple.

#![no_std]

/// A trait for removing an element at a specific index from a tuple.
///
/// This trait allows removing an element at compile-time known index `Idx`
/// from a tuple, returning the element and the remaining tuple.
///
/// # Examples
///
/// ```
/// use tuplities_remove::TupleRemove;
/// use typenum::U1;
///
/// let tuple = (1, "hello", 3.14);
/// let (removed, remainder) = TupleRemove::<U1>::remove(tuple);
/// assert_eq!(removed, "hello");
/// assert_eq!(remainder, (1, 3.14));
/// ```
#[tuplities_derive::impl_remove]
pub trait TupleRemove<Idx: typenum::Unsigned> {
    /// The type of the element being removed.
    type Type;

    /// The type of the remaining tuple after removing.
    type Remainder;

    /// Removes the element at index `Idx` from the tuple.
    ///
    /// Returns a tuple containing the removed element and the remaining tuple.
    fn remove(self) -> (Self::Type, Self::Remainder);
}

#[cfg(test)]
mod tests {
    use super::TupleRemove;
    use typenum::{U0, U1};

    #[test]
    fn test_remove_single_element_tuple() {
        let tuple = (42,);
        let (removed, _remainder): (i32, ()) = TupleRemove::<U0>::remove(tuple);
        // Check types: (i32,) remove U0 -> Type = i32, Remainder = ()
        let expected_removed: i32 = 42;
        assert_eq!(removed, expected_removed);
    }

    #[test]
    fn test_remove_two_element_tuple() {
        // Remove at U0
        let tuple = (1, 2);
        let (removed0, remainder0) = TupleRemove::<U0>::remove(tuple);
        let expected_removed0: i32 = 1;
        let expected_remainder0: (i32,) = (2,);
        assert_eq!(removed0, expected_removed0);
        assert_eq!(remainder0, expected_remainder0);

        // Remove at U1
        let tuple = (1, 2);
        let (removed1, remainder1) = TupleRemove::<U1>::remove(tuple);
        let expected_removed1: i32 = 2;
        let expected_remainder1: (i32,) = (1,);
        assert_eq!(removed1, expected_removed1);
        assert_eq!(remainder1, expected_remainder1);
    }
}
