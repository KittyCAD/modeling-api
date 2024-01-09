use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote_spanned;
use syn::{spanned::Spanned, DeriveInput, Fields, FieldsNamed};

use crate::helpers::remove_generics_defaults;

pub(crate) fn impl_derive_from_memory(input: DeriveInput) -> TokenStream2 {
    // Where in the input source code is this type defined?
    let span = input.span();
    // Name of type that is deriving Value
    let name = input.ident;
    // Any generics defined on the type deriving Value.
    let generics = input.generics;
    match input.data {
        syn::Data::Struct(data) => match data.fields {
            Fields::Named(expr) => impl_on_struct_named_fields(span, name, expr, generics),
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => impl_on_struct_no_fields(span, name, generics),
        },
        _ => quote_spanned! {span =>
            compile_error!("Value cannot be implemented on an enum or union type")
        },
    }
}

fn impl_on_struct_no_fields(span: Span, name: proc_macro2::Ident, generics: syn::Generics) -> TokenStream2 {
    // Handle generics in the original struct.
    // Firstly, if the original struct has defaults on its generics, e.g. Point2d<T = f32>,
    // don't include those defaults in this macro's output, because the compiler
    // complains it's unnecessary and will soon be a compile error.
    let generics_without_defaults = remove_generics_defaults(generics.clone());
    let where_clause = generics.where_clause;

    // Final return value: the generated Rust code to implement the trait.
    // This uses the fragments above, interpolating them into the final outputted code.
    quote_spanned! {span=>
        impl #generics_without_defaults ::kittycad_execution_plan_traits::FromMemory for #name #generics_without_defaults
        #where_clause
        {
            fn from_memory<I, M>(_fields: &mut I, _mem: &M) -> Result<Self, ::kittycad_execution_plan_traits::MemoryError>
            where
                M: ::kittycad_execution_plan_traits::ReadMemory,
                I: Iterator<Item = M::Address>
            {

                Ok(Self {})
            }
        }
    }
}

fn impl_on_struct_named_fields(
    span: Span,
    name: proc_macro2::Ident,
    fields: FieldsNamed,
    generics: syn::Generics,
) -> TokenStream2 {
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
    let read_each_field = field_names.iter().map(|(ident, span)| {
        quote_spanned! {*span=>
            let #ident = fields.next()
                .ok_or(::kittycad_execution_plan_traits::MemoryError::MemoryWrongSize)
                .and_then(|a| mem.get_composite(a))?;
        }
    });
    let instantiate_each_field = field_names.iter().map(|(ident, span)| {
        quote_spanned! {*span=>
            #ident,
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
    quote_spanned! {span=>
        impl #generics_without_defaults ::kittycad_execution_plan_traits::FromMemory for #name #generics_without_defaults
        #where_clause
        {
            fn from_memory<I, M>(fields: &mut I, mem: &M) -> Result<Self, ::kittycad_execution_plan_traits::MemoryError>
            where
                M: ::kittycad_execution_plan_traits::ReadMemory,
                I: Iterator<Item = M::Address>
            {
                #(#read_each_field)*
                Ok(Self {
                #(#instantiate_each_field)*
                })
            }
        }
    }
}
