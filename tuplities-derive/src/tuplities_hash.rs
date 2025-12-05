//! Submodule providing the derive macro for the `TupleHash` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleHash` trait implementations for all tuple sizes.
pub fn impl_tuple_hash() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: core::hash::Hash + Eq,)*> TupleHash for (#(#type_params,)*)
            {
                #[inline]
                fn tuple_hash<H: core::hash::Hasher>(&self, state: &mut H) {
                    #(
                        self.#indices.hash(state);
                    )*
                }
            }
        }
    })
}
