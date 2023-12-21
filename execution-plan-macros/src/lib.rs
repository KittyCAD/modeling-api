//! Proc-macros for implementing execution-plan traits.

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::Ident;
use syn::{spanned::Spanned, DeriveInput, Fields, GenericParam};

/// This will derive the trait `Value` from the `kittycad-execution-plan-traits` crate.
#[proc_macro_derive(ExecutionPlanValue)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    TokenStream::from(impl_derive_value(input))
}

pub(crate) fn impl_derive_value(input: DeriveInput) -> TokenStream2 {
    // Parse the input tokens into a syntax tree

    let span = input.span();
    // Name of type that is deriving Value
    let name = input.ident;
    let data = input.data;
    let generics = input.generics;
    // Hand the output tokens back to the compiler
    match data {
        syn::Data::Struct(data) => impl_value_on_struct(span, name, data, generics),
        syn::Data::Enum(data) => impl_value_on_enum(name, data, generics),
        syn::Data::Union(_) => quote_spanned! {span =>
            compile_error!("Value cannot be implemented on a union type")
        },
    }
}

fn impl_value_on_enum(
    name: proc_macro2::Ident,
    data: syn::DataEnum,
    generics: syn::Generics,
) -> proc_macro2::TokenStream {
    // Used in `into_parts()`
    // This generates one match arm for each variant of the enum on which `trait Value` is being derived.
    // Each match arm will call `into_parts()` recursively on each field of the enum variant.
    let into_parts_match_each_variant = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let fields = &variant.fields;
        let (lhs, rhs) = match fields {
            // Variant with named fields, like `Extrude{direction: Point3d, distance: f64}`
            Fields::Named(expr) => {
                let field_idents: Vec<_> = expr.named.iter().filter_map(|name| name.ident.as_ref()).collect();
                (
                    quote_spanned! {expr.span()=>
                        #name::#variant_name{#(#field_idents),*}
                    },
                    quote_spanned! {expr.span()=>
                        let mut parts = Vec::new();
                        parts.push(kittycad_execution_plan_traits::Primitive::from(stringify!(#variant_name).to_owned()));
                        #(parts.extend(#field_idents.into_parts());)*
                        parts
                    },
                )
            }
            // Variant with unnamed (positional) fields,
            // like `Towards(Point3d)`
            Fields::Unnamed(expr) => {
                // The fields don't have built-in names, but we still need to choose identifiers
                // for the variables we're going to match them into.
                // Something like MyVariant(field0, field1) => {...}
                let placeholder_field_idents: Vec<_> = expr
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| Ident::new(&format!("field{i}"), field.span()))
                    .collect();
                (
                    quote_spanned! {expr.span() =>
                        #name::#variant_name(#(#placeholder_field_idents),*)
                    },
                    quote_spanned! {expr.span() =>
                        let mut parts = Vec::new();
                        parts.push(kittycad_execution_plan_traits::Primitive::from(stringify!(#variant_name).to_owned()));
                        #(parts.extend(#placeholder_field_idents.into_parts());)*
                        parts
                    },
                )
            }
            // Enum variant with no fields.
            Fields::Unit => (
                quote_spanned! {variant.span() =>
                    #name::#variant_name
                },
                quote_spanned! {variant.span()=> {Vec::new()}},
            ),
        };
        quote_spanned! {variant.span() =>
            #lhs => {
                #rhs
            }
        }
    });

    // Used in `from_parts()`
    // This generates one match arm for each variant of the enum on which `trait Value` is being derived.
    // Each match arm will call `from_parts()` recursively on each field of the enum variant,
    // then reconstruct the enum from those parts.
    let from_parts_match_each_variant: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                // Variant with named fields, like `Extrude{direction: Point3d, distance: f64}`
                Fields::Named(expr) => {
                    let (field_idents, field_types): (Vec<_>, Vec<_>) = expr
                        .named
                        .iter()
                        .filter_map(|named| named.ident.as_ref().map(|id| (id, remove_generics(named.ty.clone()))))
                        .unzip();
                    let rhs = quote_spanned! {expr.span()=>
                        #(let #field_idents = #field_types::from_parts(values)?;)*
                        Ok(Self::#variant_name{ #(#field_idents),* })
                    };
                    quote_spanned! {variant.span() =>
                        stringify!(#variant_name) => {
                            #rhs
                        }
                    }
                }
                // Variant with unnamed (positional) fields,
                // like `Towards(Point3d)`
                Fields::Unnamed(expr) => {
                    // The fields don't have built-in names, but we still need to choose identifiers
                    // for the variables we're going to match them into.
                    // Something like MyVariant(field0, field1) => {...}
                    let (field_idents, field_types): (Vec<_>, Vec<_>) = expr
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, field)| (Ident::new(&format!("field{i}"), field.span()), &field.ty))
                        .unzip();
                    let rhs = quote_spanned! {expr.span()=>
                        #(let #field_idents = #field_types::from_parts(values)?;)*
                        Ok(Self::#variant_name(#(#field_idents),* ))
                    };
                    quote_spanned! {expr.span() =>
                        stringify!(#variant_name) => {
                            #rhs
                        }
                    }
                }
                // Enum variant with no fields.
                Fields::Unit => {
                    quote_spanned! {variant.span()=>
                        stringify!(#variant_name) => {
                            Ok(Self::#variant_name)
                        }
                    }
                }
            }
        })
        .collect();

    // Handle generics in the original struct.
    // Firstly, if the original struct has defaults on its generics, e.g. Point2d<T = f32>,
    // don't include those defaults in this macro's output, because the compiler
    // complains it's unnecessary and will soon be a compile error.
    let generics_without_defaults = remove_generics_defaults(generics.clone());
    let where_clause = generics.where_clause;

    // Final return value: the generated Rust code to implement the trait.
    // This uses the fragments above, interpolating them into the final outputted code.
    quote! {
        impl #generics_without_defaults kittycad_execution_plan_traits::Value for #name #generics_without_defaults
        #where_clause
        {
            fn into_parts(self) -> Vec<kittycad_execution_plan_traits::Primitive> {
                match self {
                    #(#into_parts_match_each_variant)*
                }
            }

            fn from_parts<I>(values: &mut I) -> Result<Self, kittycad_execution_plan_traits::MemoryError>
            where
                I: Iterator<Item = Option<kittycad_execution_plan_traits::Primitive>>,
            {
                let variant_name = String::from_parts(values)?;
                match variant_name.as_str() {
                    #(#from_parts_match_each_variant)*
                    other => Err(kittycad_execution_plan_traits::MemoryError::InvalidEnumVariant{
                        expected_type: stringify!(#name).to_owned(),
                        actual: other.to_owned(),
                    })
                }
            }
        }
    }
}

fn remove_generics(mut ty: syn::Type) -> syn::Type {
    if let syn::Type::Path(ref mut p) = ty {
        for segment in p.path.segments.iter_mut() {
            if let syn::PathArguments::AngleBracketed(ref mut _a) = segment.arguments {
                segment.arguments = syn::PathArguments::None;
            }
        }
    }
    ty
}

fn impl_value_on_struct(
    span: Span,
    name: proc_macro2::Ident,
    data: syn::DataStruct,
    generics: syn::Generics,
) -> proc_macro2::TokenStream {
    let Fields::Named(ref fields) = data.fields else {
        return quote_spanned! {span =>
            compile_error!("Value cannot be implemented on a struct with unnamed fields")
        };
    };

    // We're going to construct some fragments of Rust source code, which will get used in the
    // final generated code this function returns.

    // For every field in the struct, this macro will:
    // - In the `into_parts`, extend the Vec of parts with that field, turned into parts.
    // - In the `from_parts`, instantiate a Self with a field from that part.
    // Step one is to get a list of all named fields in the struct (and their spans):
    let field_names: Vec<_> = fields
        .named
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|ident| (ident, field.span())))
        .collect();
    // Now we can construct those `into_parts` and `from_parts` fragments.
    // We take some care to use the span of each `syn::Field` as
    // the span of the corresponding `into_parts()` and `from_parts()`
    // calls. This way if one of the field types does not
    // implement `Value` then the compiler's error message
    // underlines which field it is.
    let extend_per_field = field_names.iter().map(|(ident, span)| {
        quote_spanned! {*span=>
            parts.extend(self.#ident.into_parts());
        }
    });
    let instantiate_each_field = field_names.iter().map(|(ident, span)| {
        quote_spanned! {*span=>
            #ident: kittycad_execution_plan_traits::Value::from_parts(values)?,
        }
    });

    // Handle generics in the original struct.
    // Firstly, if the original struct has defaults on its generics, e.g. Point2d<T = f32>,
    // don't include those defaults in this macro's output, because the compiler
    // complains it's unnecessary and will soon be a compile error.
    let generics_without_defaults = remove_generics_defaults(generics.clone());
    let where_clause = generics.where_clause;

    // Final return value: the generated Rust code to implement the trait.
    // This uses the fragments above, interpolating them into the final outputted code.
    quote! {
        impl #generics_without_defaults kittycad_execution_plan_traits::Value for #name #generics_without_defaults
        #where_clause
        {
            fn into_parts(self) -> Vec<kittycad_execution_plan_traits::Primitive> {
                let mut parts = Vec::new();
                #(#extend_per_field)*
                parts
            }

            fn from_parts<I>(values: &mut I) -> Result<Self, kittycad_execution_plan_traits::MemoryError>
            where
                I: Iterator<Item = Option<kittycad_execution_plan_traits::Primitive>>,
            {
                Ok(Self {
                #(#instantiate_each_field)*
                })
            }
        }
    }
}

/// Remove the defaults from a generic type.
/// For example, turns <T = f32> into <T>.
fn remove_generics_defaults(mut g: syn::Generics) -> syn::Generics {
    for generic_param in g.params.iter_mut() {
        if let GenericParam::Type(type_param) = generic_param {
            type_param.default = None;
        }
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_enum() {
        let input = quote! {
            enum FooEnum {
                A{x: usize},
                B{y: usize},
                C(usize, String),
                D,
            }
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let out = impl_derive_value(input);
        let formatted = get_text_fmt(&out).unwrap();
        insta::assert_snapshot!(formatted);
    }

    #[test]
    fn test_enum_with_generics() {
        let input = quote! {
            enum Segment {
                Line { point: Point3d<f64> }
            }
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let out = impl_derive_value(input);
        let formatted = get_text_fmt(&out).unwrap();
        insta::assert_snapshot!(formatted);
    }

    #[test]
    fn test_struct() {
        let input = quote! {
            struct Line {
                point: Point3d<f64>,
                tag: Option<String>,
            }
        };
        let input: DeriveInput = syn::parse2(input).unwrap();
        let out = impl_derive_value(input);
        let formatted = get_text_fmt(&out).unwrap();
        insta::assert_snapshot!(formatted);
    }

    fn clean_text(s: &str) -> String {
        // Add newlines after end-braces at <= two levels of indentation.
        if cfg!(not(windows)) {
            let regex = regex::Regex::new(r"(})(\n\s{0,8}[^} ])").unwrap();
            regex.replace_all(s, "$1\n$2").to_string()
        } else {
            let regex = regex::Regex::new(r"(})(\r\n\s{0,8}[^} ])").unwrap();
            regex.replace_all(s, "$1\r\n$2").to_string()
        }
    }

    /// Format a TokenStream as a string and run `rustfmt` on the result.
    pub fn get_text_fmt(output: &proc_macro2::TokenStream) -> Result<String> {
        let content = rustfmt_wrapper::rustfmt(output).unwrap();
        Ok(clean_text(&content))
    }
}
