//! Submodule providing the derive macro for the `TupleClone` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TableIndex` trait marker implementations for all tuple sizes.
pub fn impl_tuple_clone() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: Clone,)*> TupleClone for (#(#type_params,)*)
            {
                #[inline]

                fn tuple_clone(&self) -> Self {
                    (#(self.#indices.clone(),)*)
                }
            }
        }
    })
}
