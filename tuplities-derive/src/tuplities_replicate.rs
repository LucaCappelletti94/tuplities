//! Submodule providing the derive macro for the `TupleReplicate` trait.

use quote::quote;

use crate::tuple_size::generate_all_sizes;

/// Generate `TupleReplicate` trait implementations for all tuple sizes.
pub fn impl_tuple_replicate() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        if size == 0 {
            // Empty tuple: no Clone bound needed
            return quote! {
                impl<T> TupleReplicate<T> for () {
                    #[inline]
                    fn tuple_replicate(_value: T) -> Self {
                        ()
                    }
                }
            };
        }

        if size == 1 {
            // Single element: no Clone bound needed, just move the value
            return quote! {
                impl<T> TupleReplicate<T> for (T,) {
                    #[inline]
                    fn tuple_replicate(value: T) -> Self {
                        (value,)
                    }
                }
            };
        }

        let tuple_elements = (0..size).map(|_| quote! { T }).collect::<Vec<_>>();

        // Multiple elements: Clone bound needed, clone for all but last, move for last
        let mut replicated_values = Vec::new();
        for i in 0..size {
            if i == size - 1 {
                replicated_values.push(quote! { value });
            } else {
                replicated_values.push(quote! { value.clone() });
            }
        }

        quote! {
            impl<'a, 'b, T: Clone> TupleReplicate<T> for (#(#tuple_elements,)*)
                where
                    T: 'a,
                    Self: 'b,
                    'a: 'b,
            {
                fn tuple_replicate(value: T) -> Self {
                    (#(#replicated_values,)*)
                }
            }
        }
    })
}
