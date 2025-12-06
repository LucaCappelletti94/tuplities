//! Submodule providing the derive macro for the `TupleFrom` and `TupleInto` traits.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params, type_params_with_prefix};

/// Generate `TupleFrom` trait implementations for all tuple sizes.
pub fn impl_tuple_from() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let source_type_params = type_params(size);
        let target_type_params = type_params_with_prefix(size, "B");
        let indices = indices(size);

        if size == 0 {
            return quote! {
                impl TupleFrom<()> for () {
                    fn from(_value: ()) -> Self {}
                }
            };
        }

        let source_tuple = quote! { (#(#source_type_params,)*) };

        quote! {
            impl<#(#source_type_params,)* #(#target_type_params: From<#source_type_params>,)*> TupleFrom<#source_tuple> for (#(#target_type_params,)*)
            {
                fn from(value: #source_tuple) -> Self {
                    (#(From::from(value.#indices),)*)
                }
            }
        }
    })
}
