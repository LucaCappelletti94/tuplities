//! Submodule providing the derive macro for the `NestTuple` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, type_params};

/// Generate `NestTuple` trait implementations for all tuple sizes.
pub fn impl_nest_tuple() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);

        // Generate the flat input type
        let flat_input_type = quote! { (#(#type_params,)*) };

        // Generate the nested output type
        let nested_output_type = generate_nested_output_type(&type_params);
        let nested_ref_output_type = generate_nested_ref_output_type(&type_params);
        let nested_mut_output_type = generate_nested_mut_output_type(&type_params);

        // Generate the nest implementation
        let nest_impl = generate_nest_impl(size);
        let nest_ref_impl = generate_nest_ref_impl(size);
        let nest_mut_impl = generate_nest_mut_impl(size);

        quote! {
            impl<#(#type_params,)*> NestTuple for #flat_input_type {
                type Nested = #nested_output_type;

                #[inline]
                fn nest(self) -> Self::Nested {
                    #nest_impl
                }
            }

            impl<#(#type_params,)*> NestTupleRef for #flat_input_type {
                type NestedRef<'a> = #nested_ref_output_type where Self: 'a;

                #[inline]
                fn nest_ref(&self) -> Self::NestedRef<'_> {
                    #nest_ref_impl
                }
            }

            impl<#(#type_params,)*> NestTupleMut for #flat_input_type {
                type NestedMut<'a> = #nested_mut_output_type where Self: 'a;

                #[inline]
                fn nest_mut(&mut self) -> Self::NestedMut<'_> {
                    #nest_mut_impl
                }
            }
        }
    })
}

/// Generate the nested output type for NestTuple.
/// For size N with types A, B, C, ..., N, generates (A, (B, (C, (...(N,)))))
fn generate_nested_output_type(type_params: &[proc_macro2::Ident]) -> proc_macro2::TokenStream {
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

fn generate_nested_ref_output_type(type_params: &[proc_macro2::Ident]) -> proc_macro2::TokenStream {
    match type_params {
        [] => quote! { () },
        [single] => quote! { (&'a #single,) },
        _ => {
            let last_ty = &type_params[type_params.len() - 1];
            let mut result = quote! { (&'a #last_ty,) };
            for ty in type_params.iter().rev().skip(1) {
                result = quote! { (&'a #ty, #result) };
            }
            result
        }
    }
}

fn generate_nested_mut_output_type(type_params: &[proc_macro2::Ident]) -> proc_macro2::TokenStream {
    match type_params {
        [] => quote! { () },
        [single] => quote! { (&'a mut #single,) },
        _ => {
            let last_ty = &type_params[type_params.len() - 1];
            let mut result = quote! { (&'a mut #last_ty,) };
            for ty in type_params.iter().rev().skip(1) {
                result = quote! { (&'a mut #ty, #result) };
            }
            result
        }
    }
}

/// Generate the nest implementation that builds nested structure from flat tuple
fn generate_nest_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { self },
        _ => {
            // Build nested structure: (self.0, (self.1, (self.2, (...(self.N-1,)))))
            // Start with innermost tuple (self.{N-1},) and build outwards
            let last_index = syn::Index::from(size - 1);
            (0..size)
                .rev()
                .skip(1)
                .fold(quote! { (self.#last_index,) }, |acc, i| {
                    let index = syn::Index::from(i);
                    quote! { (self.#index, #acc) }
                })
        }
    }
}

fn generate_nest_ref_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { (&self.0,) },
        _ => {
            let last_index = syn::Index::from(size - 1);
            (0..size)
                .rev()
                .skip(1)
                .fold(quote! { (&self.#last_index,) }, |acc, i| {
                    let index = syn::Index::from(i);
                    quote! { (&self.#index, #acc) }
                })
        }
    }
}

fn generate_nest_mut_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { (&mut self.0,) },
        _ => {
            let last_index = syn::Index::from(size - 1);
            (0..size)
                .rev()
                .skip(1)
                .fold(quote! { (&mut self.#last_index,) }, |acc, i| {
                    let index = syn::Index::from(i);
                    quote! { (&mut self.#index, #acc) }
                })
        }
    }
}
