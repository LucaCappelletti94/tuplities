//! Procedural macros for the Tupilities suite.

use proc_macro::TokenStream;

mod tupilities_as_ref;
mod tupilities_clone;
mod tupilities_copy;
mod tupilities_debug;
mod tupilities_default;
mod tupilities_eq;
mod tupilities_hash;
mod tupilities_partial_eq;
mod tuple_size;

/// Generate `TableIndex` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_clone(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_clone::impl_tuple_clone());
    item.into()
}

/// Generate `TupleCopy` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_copy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_copy::impl_tuple_copy());
    item.into()
}

/// Generate `TupleDebug` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_debug(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_debug::impl_tuple_debug());
    item.into()
}

/// Generate `TupleDefault` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_default::impl_tuple_default());
    item.into()
}

/// Generate `TupleHash` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_hash(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_hash::impl_tuple_hash());
    item.into()
}

/// Generate `TupleAsRef` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_as_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_as_ref::impl_tuple_as_ref());
    item.into()
}

/// Generate `TuplePartialEq` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_partial_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_partial_eq::impl_tuple_partial_eq());
    item.into()
}

/// Generate `TupleEq` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_eq::impl_tuple_eq());
    item.into()
}
