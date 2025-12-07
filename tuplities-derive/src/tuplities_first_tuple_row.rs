//! Submodule providing the derive macro for the `FirstTupleRow` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `FirstTupleRow` trait implementations for all tuple sizes.
pub fn impl_first_tuple_row() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<#(#type_params: tuplities_pop_front::TupleRefFront,)*> FirstTupleRow for (#(#type_params,)*)
            {
                type FirstRowType = (#(< #type_params as tuplities_pop_front::TuplePopFront >::Front,)*);

                fn first_tuple_row(&self) -> <Self::FirstRowType as tuplities_ref::TupleRef>::Ref<'_> {
                    (#(self.#indices.ref_front(),)*)
                }
            }
        }
    })
}
