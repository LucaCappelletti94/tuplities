//! Submodule providing the derive macro for the `LastTupleRow` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `LastTupleRow` trait implementations for all tuple sizes.
pub fn impl_last_tuple_row() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: tuplities_pop_back::TupleRefBack,)*> LastTupleRow for (#(#type_params,)*)
            {
                type LastRowType = (#(< #type_params as tuplities_pop_back::TuplePopBack >::Back,)*);

                fn last_tuple_row(&self) -> <Self::LastRowType as tuplities_ref::TupleRef>::Ref<'_> {
                    (#(self.#indices.ref_back(),)*)
                }
            }
        }
    })
}
