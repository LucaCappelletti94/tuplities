//! Submodule providing the derive macro for the `Row` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `Row` trait implementations for all tuple sizes.
pub fn impl_row() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<Idx: typenum::Unsigned, #(#type_params: tuplities_index::TupleIndex<Idx>,)*> Row<Idx> for (#(#type_params,)*)
            {
                type RowType = (#(#type_params::Type,)*);

                fn row(&self) -> <Self::RowType as tuplities_ref::TupleRef>::Ref<'_> {
                    (#(self.#indices.tuple_index(),)*)
                }
            }
        }
    })
}
