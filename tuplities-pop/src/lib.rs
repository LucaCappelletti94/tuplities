//! A trait for popping elements at specific indices from tuples.
//!
//! This crate provides the `Pop<Idx>` trait, which allows popping an element
//! at a compile-time known index from a tuple, returning the element and the
//! remaining tuple.

#![no_std]

/// A trait for popping an element at a specific index from a tuple.
///
/// This trait allows popping an element at compile-time known index `Idx`
/// from a tuple, returning the element and the remaining tuple.
///
/// # Examples
///
/// ```
/// use tuplities_pop::Pop;
/// use typenum::U1;
///
/// let tuple = (1, "hello", 3.14);
/// let (popped, remainder) = Pop::<U1>::pop(tuple);
/// assert_eq!(popped, "hello");
/// assert_eq!(remainder, (1, 3.14));
/// ```
#[tuplities_derive::impl_pop]
pub trait Pop<Idx: typenum::Unsigned> {
    /// The type of the element being popped.
    type Type;

    /// The type of the remaining tuple after popping.
    type Remainder;

    /// Pops the element at index `Idx` from the tuple.
    ///
    /// Returns a tuple containing the popped element and the remaining tuple.
    fn pop(self) -> (Self::Type, Self::Remainder);
}
