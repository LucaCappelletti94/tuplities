//! Submodule providing the derive macro for the `TupleTryFrom` and `TupleTryInto` traits.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params, type_params_with_prefix};

/// Generate `TupleTryFrom` trait implementations for all tuple sizes.
pub fn impl_tuple_try_from() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let source_type_params = type_params(size);
        let target_type_params = type_params_with_prefix(size, "B");
        let indices = indices(size);

        if size == 0 {
            return quote! {
                impl<E> TupleTryFrom<(), E> for () {
                    fn tuple_try_from(_value: ()) -> Result<Self, E> {
                        Ok(())
                    }
                }
            };
        }

        let source_tuple = quote! { (#(#source_type_params,)*) };

        quote! {
            impl<#(#source_type_params,)* #(#target_type_params,)* E> TupleTryFrom<#source_tuple, E> for (#(#target_type_params,)*)
            where
                #(#target_type_params: core::convert::TryFrom<#source_type_params>,)*
                E: #(From<<#target_type_params as core::convert::TryFrom<#source_type_params>>::Error>)+*
            {
                fn tuple_try_from(value: #source_tuple) -> Result<Self, E> {
                    Ok((#(#target_type_params::try_from(value.#indices)?,)*))
                }
            }
        }
    })
}
