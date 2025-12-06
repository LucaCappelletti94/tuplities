//! A trait for getting the length of tuples at compile time.
//!
//! This crate provides the `TupleLen` trait, which allows getting the length
//! of a tuple as a compile-time `typenum::Unsigned` type.

#![no_std]

/// A trait for getting the compile-time length of a tuple.
///
/// This trait provides the length of a tuple as an associated type `Len`
/// that implements `typenum::Unsigned`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_len]
pub trait TupleLen {
    /// The length of the tuple as a `typenum::Unsigned` type.
    type Len: typenum::Unsigned;
}

/// A marker trait for empty tuples (size 0).
///
/// This trait is implemented for the unit type `()` and can be used
/// for type-level programming to identify empty tuples.
///
/// # Examples
///
/// ```
/// use tuplities_len::{UnitTuple, TupleLen};
/// use typenum::U0;
///
/// fn is_empty<T: UnitTuple>(_tuple: T) {
///     // This function only accepts empty tuples
/// }
///
/// is_empty(()); // This works
/// // is_empty((1,)); // This would not compile
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait UnitTuple: TupleLen<Len = typenum::U0> {}

impl UnitTuple for () {}

/// A marker trait for single-element tuples (size 1).
///
/// This trait is implemented for single-element tuples `(T,)` and can be used
/// for type-level programming to identify single-element tuples.
///
/// # Examples
///
/// ```
/// use tuplities_len::{SingletonTuple, TupleLen};
/// use typenum::U1;
///
/// fn is_single<T: SingletonTuple>(_tuple: T) {
///     // This function only accepts single-element tuples
/// }
///
/// is_single((42,)); // This works
/// // is_single((1, 2)); // This would not compile
/// // is_single(()); // This would not compile
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait SingletonTuple: TupleLen<Len = typenum::U1> {}

impl<T> SingletonTuple for (T,) {}
