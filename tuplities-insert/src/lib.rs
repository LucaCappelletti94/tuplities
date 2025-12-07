//! A trait for inserting elements at specific indices into tuples.
//!
//! This crate provides the `TupleInsert<Idx, T>` trait, which allows inserting an element
//! at a compile-time known index into a tuple, returning the tuple with the element inserted.

#![no_std]

/// A trait for inserting an element at a specific index into a tuple.
///
/// This trait allows inserting an element at compile-time known index `Idx`
/// into a tuple, returning the tuple with the element inserted.
///
/// # Examples
///
/// ```
/// use tuplities_insert::TupleInsert;
/// use typenum::U1;
///
/// let tuple = (1, 3.14);
/// let inserted = TupleInsert::<U1, _>::insert(tuple, "hello");
/// assert_eq!(inserted, (1, "hello", 3.14));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_insert]
pub trait TupleInsert<Idx, T> {
    /// The type of the tuple after inserting the element.
    type Output;

    /// Inserts the element at index `Idx` into the tuple.
    ///
    /// Returns the tuple with the element inserted at the specified index.
    fn insert(self, value: T) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::TupleInsert;
    use typenum::{U0, U1, U2};

    #[test]
    fn test_insert_zero_elements() {
        let tuple = ();
        let result = TupleInsert::<U0, _>::insert(tuple, 42);
        // () insert at U0 -> (i32,)
        let expected: (i32,) = (42,);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_insert_single_element() {
        let tuple = (1,);
        // Insert at U0
        let result0 = TupleInsert::<U0, _>::insert(tuple, 42);
        let expected0: (i32, i32) = (42, 1);
        assert_eq!(result0, expected0);

        // Insert at U1
        let tuple = (1,);
        let result1 = TupleInsert::<U1, _>::insert(tuple, 42);
        let expected1: (i32, i32) = (1, 42);
        assert_eq!(result1, expected1);
    }

    #[test]
    fn test_insert_two_elements() {
        let tuple = (1, 2);
        // Insert at U0
        let result0 = TupleInsert::<U0, _>::insert(tuple, 42);
        let expected0: (i32, i32, i32) = (42, 1, 2);
        assert_eq!(result0, expected0);

        // Insert at U1
        let tuple = (1, 2);
        let result1 = TupleInsert::<U1, _>::insert(tuple, 42);
        let expected1: (i32, i32, i32) = (1, 42, 2);
        assert_eq!(result1, expected1);

        // Insert at U2
        let tuple = (1, 2);
        let result2 = TupleInsert::<U2, _>::insert(tuple, 42);
        let expected2: (i32, i32, i32) = (1, 2, 42);
        assert_eq!(result2, expected2);
    }
}
