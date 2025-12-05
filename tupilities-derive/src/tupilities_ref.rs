//! Submodule providing the derive macro for the `TupleRef` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleRef` trait implementations for all tuple sizes.
pub fn impl_tuple_ref() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params,)*> TupleRef for (#(#type_params,)*)
            {
                type Ref<'a> = (#(&'a #type_params,)*) where Self: 'a;

                #[inline]
                fn tuple_ref(&self) -> Self::Ref<'_> {
                    (#(&self.#indices,)*)
                }
            }
        }
    })
}
