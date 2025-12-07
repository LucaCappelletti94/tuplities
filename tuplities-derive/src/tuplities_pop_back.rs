//! Submodule providing the derive macro for the `TuplePopBack` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, indices, type_params};

/// Generate `TuplePopBack` trait implementations for all tuple sizes starting from 1.
pub fn impl_pop_back() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        let indices = indices(size);
        let (last, others) = type_params.split_last().unwrap();
        let (last_index, other_indices) = indices.split_last().unwrap();

        quote! {
            impl<#(#type_params,)*> TuplePopBack for (#(#type_params,)*)
            {
                type Init = (#(#others,)*);
                type Back = #last;

                #[inline]
                fn pop_back(self) -> (Self::Init, Self::Back) {
                    ((#(self.#other_indices,)*), self.#last_index)
                }
            }

            impl<#(#type_params,)*> TupleRefBack for (#(#type_params,)*)
            {
                #[inline]
                fn ref_back(&self) -> &Self::Back {
                    &self.#last_index
                }
            }

            impl<#(#type_params,)*> TupleMutBack for (#(#type_params,)*)
            {
                #[inline]
                fn mut_back(&mut self) -> &mut Self::Back {
                    &mut self.#last_index
                }
            }
        }
    })
}
