//! Submodule providing the derive macro for the `TupleRow` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleRow` trait implementations for all tuple sizes.
pub fn impl_row() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<Idx, #(#type_params: tuplities_index::TupleIndex<Idx>,)*> TupleRow<Idx> for (#(#type_params,)*)
            {
                type RowType = (#(#type_params::Element,)*);

                fn tuple_row(&self) -> <Self::RowType as tuplities_ref::TupleRef>::Ref<'_> {
                    (#(self.#indices.tuple_index(),)*)
                }
            }
        }
    })
}
