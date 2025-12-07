//! Submodule providing the derive macro for the `NestedTupleIndex` trait.

use quote::quote;

use crate::tuple_size::{generate_non_empty, type_params};

/// Generate `NestedTupleIndex<Idx>` trait implementations for all nested tuple sizes and indices.
pub fn impl_nested_tuple_index() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        let nested_input_type = generate_nested_input_type(&type_params);
        (0..size)
            .map(|u_idx| {
                let indexed_type = &type_params[u_idx];
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                let access = generate_nested_access(u_idx);
                quote! {
                    impl<#(#type_params,)*> NestedTupleIndex<#typenum> for #nested_input_type
                    {
                        type Element = #indexed_type;

                        #[inline]
                        fn nested_index(&self) -> &Self::Element {
                            #access
                        }
                    }
                }
            })
            .collect()
    })
}

/// Generate `NestedTupleIndexMut<Idx>` trait implementations for all nested tuple sizes and indices.
pub fn impl_nested_tuple_index_mut() -> proc_macro2::TokenStream {
    generate_non_empty(|size| {
        let type_params = type_params(size);
        let nested_input_type = generate_nested_input_type(&type_params);
        (0..size)
            .map(|u_idx| {
                let indexed_type = &type_params[u_idx];
                let typenum_ident =
                    syn::Ident::new(&format!("U{u_idx}"), proc_macro2::Span::call_site());
                let typenum: syn::Path = syn::parse_quote!(typenum::#typenum_ident);
                let access = generate_nested_access_mut(u_idx);
                quote! {
                    impl<#(#type_params,)*> NestedTupleIndexMut<#typenum> for #nested_input_type
                    {
                        type Element = #indexed_type;

                        #[inline]
                        fn nested_index_mut(&mut self) -> &mut Self::Element {
                            #access
                        }
                    }
                }
            })
            .collect()
    })
}

/// Generate the nested input type for FlattenNestedTuple.
/// For size N with types A, B, C, ..., N, generates (A, (B, (C, (...(N,)))))
fn generate_nested_input_type(type_params: &[proc_macro2::Ident]) -> proc_macro2::TokenStream {
    match type_params {
        [] => quote! { () },
        [single] => quote! { (#single,) },
        _ => {
            // Build nested structure: (T0, (T1, (T2, (...(T{N-1},))))
            // Start with innermost (T{N-1},) and wrap outwards
            let last_ty = &type_params[type_params.len() - 1];
            let mut result = quote! { (#last_ty,) };
            for ty in type_params.iter().rev().skip(1) {
                result = quote! { (#ty, #result) };
            }
            result
        }
    }
}

/// Generate the access expression for a given flat index in the nested structure
fn generate_nested_access(flat_idx: usize) -> proc_macro2::TokenStream {
    if flat_idx == 0 {
        quote! { &self.0 }
    } else {
        let mut access = quote! { self.1 };
        for _ in 1..flat_idx {
            access = quote! { #access .1 };
        }
        quote! { &#access .0 }
    }
}

/// Generate the mutable access expression for a given flat index in the nested structure
fn generate_nested_access_mut(flat_idx: usize) -> proc_macro2::TokenStream {
    if flat_idx == 0 {
        quote! { &mut self.0 }
    } else {
        let mut access = quote! { self.1 };
        for _ in 1..flat_idx {
            access = quote! { #access .1 };
        }
        quote! { &mut #access .0 }
    }
}