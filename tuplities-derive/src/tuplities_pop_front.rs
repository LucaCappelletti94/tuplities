//! Submodule providing the derive macro for the `TuplePopFront` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, indices, type_params};

/// Generate `TuplePopFront` trait implementations for all tuple sizes starting from 1.
pub fn impl_pop_front() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        let indices = indices(size);
        let (first, others) = type_params.split_first().unwrap();
        let (first_index, other_indices) = indices.split_first().unwrap();

        quote! {
            impl<#(#type_params,)*> TuplePopFront for (#(#type_params,)*)
            {
                type Front = #first;
                type Tail = (#(#others,)*);

                #[inline]
                fn pop_front(self) -> (Self::Front, Self::Tail) {
                    (self.#first_index, (#(self.#other_indices,)*))
                }
            }

            impl<#(#type_params,)*> TupleRefFront for (#(#type_params,)*)
            {
                #[inline]
                fn ref_front(&self) -> &Self::Front {
                    &self.#first_index
                }
            }

            impl<#(#type_params,)*> TupleMutFront for (#(#type_params,)*)
            {
                #[inline]
                fn mut_front(&mut self) -> &mut Self::Front {
                    &mut self.#first_index
                }
            }
        }
    })
}
