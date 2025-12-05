//! Submodule providing the derive macro for the `TupleMut` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleMut` trait implementations for all tuple sizes.
pub fn impl_tuple_mut() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        let mut_tuple = if size == 0 {
            quote! { () }
        } else {
            let mut_refs = (0..size).map(|_| quote! { &'a mut });
            quote! { (#(#mut_refs #type_params,)*) }
        };

        quote! {
            impl<#(#type_params,)*> TupleMut for (#(#type_params,)*)
            {
                type Mut<'a> = #mut_tuple where Self: 'a;

                #[inline]
                fn tuple_mut(&mut self) -> Self::Mut<'_> {
                    (#(&mut self.#indices,)*)
                }
            }
        }
    })
}
