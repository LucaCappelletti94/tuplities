//! Submodule providing the derive macro for the `TupleRefMap` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleRefMap` trait implementations for all tuple sizes.
pub fn impl_tuple_ref_map() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: TupleRef,)*> TupleRefMap for (#(#type_params,)*)
            {
                type RefMap<'a> = (#(< #type_params as TupleRef >::Ref<'a>,)*) where Self: 'a;

                #[inline]
                fn tuple_ref_map(&self) -> Self::RefMap<'_> {
                    (#(self.#indices.tuple_ref(),)*)
                }
            }
        }
    })
}
