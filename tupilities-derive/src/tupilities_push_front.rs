//! Submodule providing the derive macro for the `PushFront` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `PushFront` trait implementations for all tuple sizes.
pub fn impl_push_front() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<T, #(#type_params,)*> PushFront<T> for (#(#type_params,)*)
            {
                type Output = (T, #(#type_params,)*);

                #[inline]
                fn push_front(self, value: T) -> Self::Output {
                    (value, #(self.#indices,)*)
                }
            }
        }
    })
}
