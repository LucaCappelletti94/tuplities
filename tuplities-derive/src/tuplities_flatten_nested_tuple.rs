//! Submodule providing the derive macro for the `FlattenNestedTuple` trait.

use quote::quote;

use crate::tuple_size::{generate_all_sizes, type_params};

/// Generate `FlattenNestedTuple` trait implementations for all tuple sizes.
pub fn impl_flatten_nested_tuple() -> proc_macro2::TokenStream {
    generate_all_sizes(|size| {
        let type_params = type_params(size);

        // Generate the nested input type
        let nested_input_type = generate_nested_input_type(&type_params);

        // Generate the flattened output type
        let flattened_output_type = quote! { (#(#type_params,)*) };

        // Generate the flatten implementation
        let flatten_impl = generate_flatten_impl(size);

        // Generate the flatten_ref implementation
        let flatten_ref_impl = generate_flatten_ref_impl(size);

        // Generate the flatten_mut implementation
        let flatten_mut_impl = generate_flatten_mut_impl(size);

        quote! {
            impl<#(#type_params,)*> FlattenNestedTuple for #nested_input_type {
                type Flattened = #flattened_output_type;

                #[inline]
                fn flatten(self) -> Self::Flattened {
                    #flatten_impl
                }

                #[inline]
                fn flatten_ref(&self) -> <Self::Flattened as TupleRef>::Ref<'_> {
                    #flatten_ref_impl
                }

                #[inline]
                fn flatten_mut(&mut self) -> <Self::Flattened as TupleMut>::Mut<'_> {
                    #flatten_mut_impl
                }
            }
        }
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

/// Generate the flatten implementation that extracts elements from nested structure
fn generate_flatten_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { self },
        _ => quote! { self.1.flatten().push_front(self.0) },
    }
}

/// Generate the flatten_ref implementation that extracts references from nested structure
fn generate_flatten_ref_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { (&self.0,) },
        _ => {
            let accesses = (0..size).map(|i| {
                if i == 0 {
                    quote! { &self.0 }
                } else {
                    // Generate nested access: self.1.1.1...1.0 with (i-1) ones
                    let nested_access = (1..i).fold(quote! { self.1 }, |acc, _| {
                        quote! { #acc .1 }
                    });
                    quote! { &(#nested_access .0) }
                }
            });

            quote! { (#(#accesses,)*) }
        }
    }
}

/// Generate the flatten_mut implementation that extracts mutable references from nested structure
fn generate_flatten_mut_impl(size: usize) -> proc_macro2::TokenStream {
    match size {
        0 => quote! { () },
        1 => quote! { (&mut self.0,) },
        _ => {
            let accesses = (0..size).map(|i| {
                if i == 0 {
                    quote! { &mut self.0 }
                } else {
                    // Generate nested access: self.1.1.1...1.0 with (i-1) ones
                    let nested_access = (1..i).fold(quote! { self.1 }, |acc, _| {
                        quote! { #acc .1 }
                    });
                    quote! { &mut (#nested_access .0) }
                }
            });

            quote! { (#(#accesses,)*) }
        }
    }
}
