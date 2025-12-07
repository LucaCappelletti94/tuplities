//! Submodule providing the derive macro for the `TupleMutMap` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleMutMap` trait implementations for all tuple sizes.
pub fn impl_tuple_mut_map() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: TupleMut,)*> TupleMutMap for (#(#type_params,)*)
            {
                type MutMap<'a> = (#(< #type_params as TupleMut >::Mut<'a>,)*) where Self: 'a;

                #[inline]
                fn tuple_mut_map(&mut self) -> Self::MutMap<'_> {
                    (#(self.#indices.tuple_mut(),)*)
                }
            }
        }
    })
}
