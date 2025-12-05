//! Submodule providing the derive macro for the `TupleEq` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleEq` trait implementations for all tuple sizes.
pub fn impl_tuple_eq() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        let body = if size == 0 {
            quote! { true }
        } else {
            quote! { #(self.#indices == other.#indices)&&* }
        };

        quote! {
            impl<#(#type_params: Eq,)*> TupleEq for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_eq(&self, other: &Self) -> bool {
                    #body
                }
            }
        }
    })
}
