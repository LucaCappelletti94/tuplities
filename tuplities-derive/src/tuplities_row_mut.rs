//! Submodule providing the derive macro for the `RowMut` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `RowMut` trait implementations for all tuple sizes.
pub fn impl_row_mut() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<Idx: typenum::Unsigned, #(#type_params: tuplities_index::TupleIndexMut<Idx>,)*> RowMut<Idx> for (#(#type_params,)*)
            {
                fn row_mut(&mut self) -> <Self::RowType as tuplities_mut::TupleMut>::Mut<'_> {
                    (#(self.#indices.tuple_index_mut(),)*)
                }
            }
        }
    })
}
