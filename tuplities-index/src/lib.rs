//! A trait for indexing into tuples at compile-time known positions.
//!
//! This crate provides the `TupleIndex<Idx>` trait, which allows accessing
//! elements at compile-time known indices from tuples using `typenum::Unsigned` types.

#![no_std]

use tuplities_len::TupleLen;

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
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_tuple_index]
pub trait TupleIndex<Idx>: TupleLen {
    /// The type of the element at index `Idx`.
    type Element;

    /// Returns a reference to the element at index `Idx`.
    fn tuple_index(&self) -> &Self::Element;
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
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleIndexMut<Idx>: TupleIndex<Idx> {
    /// Returns a mutable reference to the element at index `Idx`.
    fn tuple_index_mut(&mut self) -> &mut Self::Element;
}

/// A convenience trait for accessing the first element (index 0) in tuples.
///
/// This trait is automatically implemented for any tuple that implements `TupleIndex<U0>`.
///
/// # Examples
///
/// ```
/// use tuplities_index::FirstTupleIndex;
///
/// let tuple = (1, "hello", 3.14);
/// let first = tuple.first_tuple_index();
/// assert_eq!(*first, 1);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FirstTupleIndex: TupleIndex<typenum::U0> {
    /// Returns a reference to the first element in the tuple.
    fn first_tuple_index(&self) -> &Self::Element {
        self.tuple_index()
    }
}

impl<T: TupleIndex<typenum::U0>> FirstTupleIndex for T {}

/// A convenience trait for accessing the last element in tuples.
///
/// This trait is automatically implemented for any tuple that implements `TupleIndex<LastIndex>`
/// where `LastIndex` is calculated as `TupleLen::Len - 1`.
///
/// # Examples
///
/// ```
/// use tuplities_index::LastTupleIndex;
///
/// let tuple = (1, "hello", 3.14);
/// let last = tuple.last_tuple_index();
/// assert_eq!(*last, 3.14);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait LastTupleIndex:
    TupleIndex<
        <<Self as TupleLen>::Len as core::ops::Sub<typenum::U1>>::Output,
        Len: core::ops::Sub<typenum::U1, Output: typenum::Unsigned>,
    >
{
    /// Returns a reference to the last element in the tuple.
    fn last_tuple_index(&self) -> &Self::Element;
}

impl<T> LastTupleIndex for T
where
    T: TupleIndex<
            <<T as TupleLen>::Len as core::ops::Sub<typenum::U1>>::Output,
            Len: core::ops::Sub<typenum::U1, Output: typenum::Unsigned>,
        >,
{
    fn last_tuple_index(&self) -> &Self::Element {
        self.tuple_index()
    }
}
