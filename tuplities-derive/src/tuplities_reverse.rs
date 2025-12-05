//! Submodule providing the derive macro for the `TupleReverse` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleReverse` trait implementations for all tuple sizes.
pub fn impl_reverse() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);
        let reversed_indices: Vec<_> = indices.iter().rev().collect();
        let reversed_type_params: Vec<_> = type_params.iter().rev().collect();

        quote! {
            impl<#(#type_params,)*> TupleReverse for (#(#type_params,)*)
            {
                type Output = (#(#reversed_type_params,)*);

                #[inline]
                fn reverse(self) -> Self::Output {
                    (#(self.#reversed_indices,)*)
                }
            }
        }
    })
}
