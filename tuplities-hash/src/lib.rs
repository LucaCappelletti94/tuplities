#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleHash` trait.

use tuplities_eq::TupleEq;

#[tuplities_derive::impl_tuple_hash]
/// A trait for hashing tuples with a generic hasher.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleHash: TupleEq {
    /// Hashes the tuple into the given hasher.
    fn tuple_hash<H: core::hash::Hasher>(&self, state: &mut H);
}
