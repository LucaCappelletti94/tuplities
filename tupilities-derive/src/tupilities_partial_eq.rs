//! Submodule providing the derive macro for the `TuplePartialEq` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TuplePartialEq` trait implementations for all tuple sizes.
pub fn impl_tuple_partial_eq() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        let body = if size == 0 {
            quote! { true }
        } else {
            quote! { #(self.#indices == other.#indices)&&* }
        };

        quote! {
            impl<#(#type_params: PartialEq,)*> TuplePartialEq for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_eq(&self, other: &Self) -> bool {
                    #body
                }
            }
        }
    })
}
