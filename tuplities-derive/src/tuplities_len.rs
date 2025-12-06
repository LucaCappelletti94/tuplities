//! Submodule providing the derive macro for the `TupleLen` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, type_params};

/// Generate `TupleLen` trait implementations for all tuple sizes.
pub fn impl_len() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);
        let typenum_ident = syn::Ident::new(&format!("U{size}"), proc_macro2::Span::call_site());
        let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
        quote! {
            impl<#(#type_params,)*> TupleLen for (#(#type_params,)*) {
                type Len = #typenum;
            }
        }
    })
}
