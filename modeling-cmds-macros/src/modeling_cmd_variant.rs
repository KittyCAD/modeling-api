use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput};

pub(crate) fn derive_empty(input: DeriveInput) -> TokenStream {
    // Where in the input source code is this type defined?
    let span = input.span();
    // Name of type that is deriving the trait.
    let name = input.ident;
    // Delegate to a macro that can generate code for this specific type.
    match input.data {
        syn::Data::Struct(_) => derive_empty_on_struct(name),
        syn::Data::Enum(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on an enum type")
        },
        syn::Data::Union(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on a union type")
        },
    }
}

fn derive_empty_on_struct(name: proc_macro2::Ident) -> TokenStream {
    quote! {
        impl kittycad_modeling_cmds::ModelingCmdVariant for #name {
            type Output = ();

            fn into_enum(self) -> kittycad_modeling_cmds::ModelingCmd {
                kittycad_modeling_cmds::ModelingCmd::#name(self)
            }
            fn name() -> &'static str {
                stringify!(#name)
            }
        }
    }
}

// For comments, see `fn derive_empty`.
pub(crate) fn derive_nonempty(input: DeriveInput) -> TokenStream {
    let span = input.span();
    let name = input.ident;
    match input.data {
        syn::Data::Struct(_) => derive_nonempty_on_struct(name),
        syn::Data::Enum(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on an enum type")
        },
        syn::Data::Union(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on a union type")
        },
    }
}

fn derive_nonempty_on_struct(name: proc_macro2::Ident) -> TokenStream {
    quote! {
        impl kittycad_modeling_cmds::ModelingCmdVariant for #name {
            type Output = kittycad_modeling_cmds::output::#name;
            fn into_enum(self) -> kittycad_modeling_cmds::ModelingCmd {
                kittycad_modeling_cmds::ModelingCmd::#name(self)
            }
            fn name() -> &'static str {
                stringify!(#name)
            }
        }
    }
}
