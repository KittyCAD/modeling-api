use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DataEnum, DeriveInput, Fields, Ident};

use crate::helpers::remove_generics_defaults;

pub(crate) fn impl_derive_value(input: DeriveInput) -> TokenStream2 {
    // Where in the input source code is this type defined?
    let span = input.span();
    // Name of type that is deriving Value
    let name = input.ident;
    // Any generics defined on the type deriving Value.
    let generics = input.generics;
    match input.data {
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
    // First build fragments of the AST, then we'll combine them into a final output below.
    // Build the arms of the `match` statements we'll use below.
    let into_parts_match_each_variant = into_parts_match_arms(&data, &name);
    let from_parts_match_each_variant = from_parts_match_arms(&data);
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

// Used in `from_parts()`
// This generates one match arm for each variant of the enum on which `trait Value` is being derived.
// Each match arm will call `from_parts()` recursively on each field of the enum variant,
// then reconstruct the enum from those parts.
fn from_parts_match_arms(data: &DataEnum) -> Vec<TokenStream2> {
    data.variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                // Variant with named fields, like
                // ```
                // enum MyEnum {
                //      Extrude{direction: Point3d, distance: f64},
                // }
                // ```
                Fields::Named(expr) => {
                    let (field_idents, instantiate_fields): (Vec<_>, Vec<_>) = expr
                        .named
                        .iter()
                        .filter_map(|named| {
                            named
                                .ident
                                .as_ref()
                                .map(|id| (id, make_instantiate_field(named.ty.clone(), id)))
                        })
                        .unzip();
                    let rhs = quote_spanned! {expr.span()=>
                        #(#instantiate_fields)*
                        Ok(Self::#variant_name{ #(#field_idents),* })
                    };
                    quote_spanned! {variant.span() =>
                        stringify!(#variant_name) => {
                            #rhs
                        }
                    }
                }
                // Variant with unnamed fields (i.e. fields referenced by position, not name), like
                // ```
                // enum MyEnum {
                //      Extrude(f64),
                // }
                // ```
                Fields::Unnamed(expr) => {
                    // The fields don't have built-in names, but we still need to choose identifiers
                    // for the variables we're going to match them into.
                    // Something like MyVariant(field0, field1) => {...}
                    let (field_idents, instantiate_fields): (Vec<_>, Vec<_>) = expr
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, field)| {
                            let id = Ident::new(&format!("field{i}"), field.span());
                            (id.clone(), make_instantiate_field(field.ty.clone(), &id))
                        })
                        .unzip();
                    let rhs = quote_spanned! {expr.span()=>
                        #(#instantiate_fields)*
                        Ok(Self::#variant_name(#(#field_idents),* ))
                    };
                    quote_spanned! {expr.span() =>
                        stringify!(#variant_name) => {
                            #rhs
                        }
                    }
                }
                // Variant with no fields (or, equivalently, where the fields are () aka the unit type), like
                // ```
                // enum MyEnum {
                //      Extrude,
                // }
                // ```
                Fields::Unit => {
                    quote_spanned! {variant.span()=>
                        stringify!(#variant_name) => {
                            Ok(Self::#variant_name)
                        }
                    }
                }
            }
        })
        .collect()
}

fn make_instantiate_field(ty: syn::Type, id: &Ident) -> TokenStream2 {
    if let Some(ub) = unbox(ty.clone()) {
        quote! {
            let #id = Box::new(#ub::from_parts(values)?);
        }
    } else {
        let field_type = remove_generics(ty);
        quote! {
            let #id = #field_type::from_parts(values)?;
        }
    }
}

/// Given `Box<T>`, returns `T`.
/// i.e. it returns the inner type of a boxed type.
fn unbox(ty: syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(p) = ty else {
        return None;
    };
    let Some(first) = p.path.segments.into_iter().next() else {
        return None;
    };
    if first.ident != "Box" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(type_of_box) = first.arguments else {
        return None;
    };
    let Some(syn::GenericArgument::Type(type_of_box)) = type_of_box.args.into_iter().next() else {
        return None;
    };
    Some(type_of_box)
}

// Used in `into_parts()`
// This generates one match arm for each variant of the enum on which `trait Value` is being derived.
// Each match arm will call `into_parts()` recursively on each field of the enum variant.
fn into_parts_match_arms(data: &DataEnum, name: &proc_macro2::Ident) -> Vec<TokenStream2> {
    data.variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let fields = &variant.fields;
            let (lhs, rhs) = match fields {
                // Variant with named fields, like
                // ```
                // enum MyEnum {
                //      Extrude{direction: Point3d, distance: f64},
                // }
                // ```
                Fields::Named(expr) => {
                    let field_idents: Vec<_> = expr.named.iter().filter_map(|name| name.ident.as_ref()).collect();
                    (
                        quote_spanned! {expr.span()=>
                            #name::#variant_name{#(#field_idents),*}
                        },
                        quote_spanned! {expr.span()=>
                            let mut parts = Vec::new();
                            let tag = stringify!(#variant_name).to_owned();
                            parts.push(kittycad_execution_plan_traits::Primitive::from(tag));
                            #(parts.extend(#field_idents.into_parts());)*
                            parts
                        },
                    )
                }
                // Variant with unnamed fields (i.e. fields referenced by position, not name), like
                // ```
                // enum MyEnum {
                //      Extrude(f64),
                // }
                // ```
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
                            let tag = stringify!(#variant_name).to_owned();
                            parts.push(kittycad_execution_plan_traits::Primitive::from(tag));
                            #(parts.extend(#placeholder_field_idents.into_parts());)*
                            parts
                        },
                    )
                }
                // Variant with no fields (or, equivalently, where the fields are () aka the unit type), like
                // ```
                // enum MyEnum {
                //      Extrude,
                // }
                // ```
                Fields::Unit => (
                    quote_spanned! {variant.span() =>
                        #name::#variant_name
                    },
                    quote_spanned! {variant.span()=>
                        let tag = stringify!(#variant_name).to_owned();
                        let part = kittycad_execution_plan_traits::Primitive::from(tag);
                        vec![part]
                    },
                ),
            };
            quote_spanned! {variant.span() =>
                #lhs => {
                    #rhs
                }
            }
        })
        .collect()
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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

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

    #[test]
    fn test_unbox() {
        let tests = [
            // Positive case
            (quote! {Box<usize>}, Some(quote! {usize})),
            // Negative case
            (quote! {usize}, None),
        ];
        for (input, expected) in tests {
            let input_type: syn::Type = syn::parse2(input).unwrap();
            let actual = unbox(input_type);
            match expected {
                None => assert_eq!(None, actual, "expected unbox to return None but it returned Some"),
                Some(expected) => {
                    let expected: syn::Type = syn::parse2(expected).unwrap();
                    assert_eq!(actual.unwrap(), expected);
                }
            };
        }
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
