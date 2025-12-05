//! Procedural macros for the Tupilities suite.

use proc_macro::TokenStream;

mod tupilities_clone;
mod tuple_size;

/// Generate `TableIndex` trait implementations for all tuple sizes.
#[proc_macro_attribute]
pub fn impl_tuple_clone(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = proc_macro2::TokenStream::from(item);
    item.extend(tupilities_clone::impl_tuple_clone());
    item.into()
}
