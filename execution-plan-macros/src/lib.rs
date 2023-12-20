//! Proc-macros for implementing execution-plan traits.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Fields};

#[proc_macro_derive(MyMacro)]
pub fn impl_value(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let span = input.span();
    // Name of type that is deriving Value
    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = match input.data {
        syn::Data::Struct(data) => impl_value_on_struct(span, name, data),
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => quote_spanned! {span =>
            compile_error!("Value cannot be implemented on a union type")
        },
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn impl_value_on_struct(span: Span, name: proc_macro2::Ident, data: syn::DataStruct) -> proc_macro2::TokenStream {
    let Fields::Named(ref fields) = data.fields else {
        return quote_spanned! {span =>
            compile_error!("Value cannot be implemented on a struct with unnamed fields")
        };
    };
    let field_names: Vec<_> = fields.named.iter().filter_map(|field| field.ident.as_ref()).collect();
    let mut extend_per_field = quote!();
    let mut instantiate_each_field = quote!();
    for field in field_names {
        extend_per_field = quote! {
            parts.extend(self.#field.into_parts());
            #extend_per_field
        };
        instantiate_each_field = quote! {
            #field: kittycad_execution_plan_traits::Value::from_parts(values)?,
            #instantiate_each_field
        }
    }
    quote! {
        impl kittycad_execution_plan_traits::Value for #name {
            fn into_parts(self) -> Vec<kittycad_execution_plan_traits::Primitive> {
                let mut parts = Vec::new();
                #extend_per_field
                parts
            }

            fn from_parts<I>(values: &mut I) -> Result<Self, kittycad_execution_plan_traits::MemoryError>
            where
                I: Iterator<Item = Option<kittycad_execution_plan_traits::Primitive>>,
            {
                Ok(Self {
                    #instantiate_each_field
                })
            }
        }
    }
}
