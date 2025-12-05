//! A trait for getting the length of tuples at compile time.
//!
//! This crate provides the `TupleLen` trait, which allows getting the length
//! of a tuple as a compile-time `typenum::Unsigned` type.

#![no_std]

/// A trait for getting the compile-time length of a tuple.
///
/// This trait provides the length of a tuple as an associated type `Idx`
/// that implements `typenum::Unsigned`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_len]
pub trait TupleLen {
    /// The length of the tuple as a `typenum::Unsigned` type.
    type Idx: typenum::Unsigned;
}
