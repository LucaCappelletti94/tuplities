//! A trait for indexing into tuples at compile-time known positions.
//!
//! This crate provides the `TupleIndex<Idx>` trait, which allows accessing
//! elements at compile-time known indices from tuples using `typenum::Unsigned` types.

#![no_std]

/// A trait for indexing into tuples at compile-time known positions.
///
/// This trait allows accessing elements at specific indices `Idx`
/// from tuples, where `Idx` is a compile-time constant from `typenum`.
///
/// # Examples
///
/// ```
/// use tuplities_index::TupleIndex;
/// use typenum::U1;
///
/// let tuple = (1, "hello", 3.14);
/// let element = TupleIndex::<U1>::tuple_index(&tuple);
/// assert_eq!(*element, "hello");
/// ```
#[tuplities_derive::impl_tuple_index]
pub trait TupleIndex<Idx: typenum::Unsigned> {
    /// The type of the element at index `Idx`.
    type Type;

    /// Returns a reference to the element at index `Idx`.
    fn tuple_index(&self) -> &Self::Type;
}

/// A trait for mutable indexing into tuples at compile-time known positions.
///
/// This trait allows mutable access to elements at specific indices `Idx`
/// from tuples, where `Idx` is a compile-time constant from `typenum`.
///
/// # Examples
///
/// ```
/// use tuplities_index::TupleIndexMut;
/// use typenum::U1;
///
/// let mut tuple = (1, "hello", 3.14);
/// let element = TupleIndexMut::<U1>::tuple_index_mut(&mut tuple);
/// *element = "world";
/// assert_eq!(tuple.1, "world");
/// ```
pub trait TupleIndexMut<Idx: typenum::Unsigned>: TupleIndex<Idx> {
    /// Returns a mutable reference to the element at index `Idx`.
    fn tuple_index_mut(&mut self) -> &mut Self::Type;
}
