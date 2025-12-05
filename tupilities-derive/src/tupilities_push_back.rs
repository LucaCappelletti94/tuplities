//! Submodule providing the derive macro for the `PushBack` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `PushBack` trait implementations for all tuple sizes.
pub fn impl_push_back() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let indices = indices(size);

        quote! {
            impl<T, #(#type_params,)*> PushBack<T> for (#(#type_params,)*)
            {
                type Output = (#(#type_params,)* T);

                fn push_back(self, value: T) -> Self::Output {
                    (#(self.#indices,)* value)
                }
            }
        }
    })
}
