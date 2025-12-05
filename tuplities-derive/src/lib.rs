//! Procedural macros for the tuplities suite.

use proc_macro::TokenStream;

mod tuplities_as_ref;
mod tuplities_clone;
mod tuplities_copy;
mod tuplities_debug;
mod tuplities_default;
mod tuplities_eq;
mod tuplities_hash;
mod tuplities_index;
mod tuplities_mut;
mod tuplities_option;
mod tuplities_ord;
mod tuplities_partial_eq;
mod tuplities_partial_ord;
mod tuplities_pop;
mod tuplities_pop_back;
mod tuplities_pop_front;
mod tuplities_push_back;
mod tuplities_push_front;
mod tuplities_ref;
mod tuple_size;

/// Generate `TableIndex` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_clone(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_clone::impl_tuple_clone());
    item.into()
}

/// Generate `TupleCopy` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_copy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_copy::impl_tuple_copy());
    item.into()
}

/// Generate `TupleDebug` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_debug(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_debug::impl_tuple_debug());
    item.into()
}

/// Generate `TupleDefault` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_default(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_default::impl_tuple_default());
    item.into()
}

/// Generate `TupleHash` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_hash(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_hash::impl_tuple_hash());
    item.into()
}

/// Generate `TupleAsRef` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_as_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_as_ref::impl_tuple_as_ref());
    item.into()
}

/// Generate `TuplePartialEq` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_partial_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_partial_eq::impl_tuple_partial_eq());
    item.into()
}

/// Generate `TupleEq` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_eq(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_eq::impl_tuple_eq());
    item.into()
}

/// Generate `TuplePartialOrd` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_partial_ord(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_partial_ord::impl_tuple_partial_ord());
    item.into()
}

/// Generate `TupleOrd` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_ord(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_ord::impl_tuple_ord());
    item.into()
}

/// Generate `PopFront` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_pop_front(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_pop_front::impl_pop_front());
    item.into()
}

/// Generate `PopBack` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_pop_back(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_pop_back::impl_pop_back());
    item.into()
}

/// Generate `Pop<Idx>` trait implementations for all tuple sizes and indices.
#[proc_macro_attribute]
pub fn impl_pop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_pop::impl_pop());
    item.into()
}

/// Generate `TupleIndex<Idx>` trait implementations for all tuple sizes and indices.
#[proc_macro_attribute]
pub fn impl_tuple_index(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_index::impl_tuple_index());
    item.into()
}

/// Generate `PushBack` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_push_back(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_push_back::impl_push_back());
    item.into()
}

/// Generate `PushFront` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_push_front(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_push_front::impl_push_front());
    item.into()
}

/// Generate `TupleRef` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_ref(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_ref::impl_tuple_ref());
    item.into()
}

/// Generate `TupleMut` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_mut(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_mut::impl_tuple_mut());
    item.into()
}

/// Generate `TupleOption` and `IntoTupleOption` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_option(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tuplities_option::impl_tuple_option());
    item.extend(tuplities_option::impl_into_tuple_option());
    item.into()
}
