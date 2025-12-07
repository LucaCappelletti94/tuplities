//! Submodule providing the derive macro for the `TupleIndex` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, type_params};

/// Generate `TupleIndex<Idx>` trait implementations for all tuple sizes and indices.
pub fn impl_tuple_index() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        (0..size)
            .map(|u_idx| {
                let indexed_type = &type_params[u_idx];
                let indexed_index = syn::Index::from(u_idx);
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                quote! {
                    impl<#(#type_params,)*> TupleIndex<#typenum> for (#(#type_params,)*)
                    {
                        type Element = #indexed_type;

                        #[inline]
                        fn tuple_index(&self) -> &Self::Element {
                            &self.#indexed_index
                        }
                    }
                    impl<#(#type_params,)*> TupleIndexMut<#typenum> for (#(#type_params,)*)
                    {
                        #[inline]
                        fn tuple_index_mut(&mut self) -> &mut Self::Element {
                            &mut self.#indexed_index
                        }
                    }
                }
            })
            .collect()
    })
}
