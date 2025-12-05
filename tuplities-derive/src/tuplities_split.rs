//! Submodule providing the derive macro for the `TupleSplit` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, indices, type_params};

/// Generate `TupleSplit<Idx>` trait implementations for all tuple sizes and indices.
pub fn impl_split() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        (0..=size)
            .map(|u_idx| {
                let type_params = type_params(size);
                let indices = indices(size);
                let left_types = &type_params[..u_idx];
                let right_types = &type_params[u_idx..];
                let left_indices = &indices[..u_idx];
                let right_indices = &indices[u_idx..];
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                quote! {
                    impl<#(#type_params,)*> TupleSplit<#typenum> for (#(#type_params,)*)
                    {
                        type Left = (#(#left_types,)*);
                        type Right = (#(#right_types,)*);

                        #[inline]
                        fn split(self) -> (Self::Left, Self::Right) {
                            ((#(self.#left_indices,)*), (#(self.#right_indices,)*))
                        }
                    }
                }
            })
            .collect()
    })
}
