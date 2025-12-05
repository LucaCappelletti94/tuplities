//! Submodule providing the derive macro for the `TupleOption` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleOption` trait implementations for all tuple sizes.
pub fn impl_tuple_option() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
		let indices = indices(size);

        quote! {
            impl<#(#type_params,)*> TupleOption for (#(Option<#type_params>,)*)
            {
                type Transposed = (#(#type_params,)* );

                #[inline]
                fn transpose(self) -> Option<Self::Transposed> {
                    Some((#(self.#indices?,)*))
                }
            }
        }
    })
}

/// Generate `IntoTupleOption` trait implementations for all tuple sizes.
pub fn impl_into_tuple_option() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
		let indices = indices(size);

        quote! {
            impl<#(#type_params,)*> IntoTupleOption for (#(#type_params,)*)
            {
                type AsOptions = (#(Option<#type_params>,)*);

                #[inline]
                fn as_options(self) -> Self::AsOptions {
                    ( #(Some(self.#indices),)* )
                }
            }
        }
    })
}