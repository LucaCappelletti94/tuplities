//! Submodule providing the derive macro for the `TupleRowMut` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleRowMut` trait implementations for all tuple sizes.
pub fn impl_row_mut() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<Idx, #(#type_params: tuplities_index::TupleIndexMut<Idx>,)*> TupleRowMut<Idx> for (#(#type_params,)*)
            {
                fn tuple_row_mut(&mut self) -> <Self::RowType as tuplities_mut::TupleMut>::Mut<'_> {
                    (#(self.#indices.tuple_index_mut(),)*)
                }
            }
        }
    })
}
