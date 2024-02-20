use proc_macro2::{TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, DeriveInput};


pub(crate) fn impl_empty(input: DeriveInput) -> TokenStream {
    // Where in the input source code is this type defined?
    let span = input.span();
    // Name of type that is deriving Value
    let name = input.ident;
    match input.data {
        syn::Data::Struct(_) => impl_empty_on_struct(name),
        syn::Data::Enum(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on an enum type")
        },
        syn::Data::Union(_) => quote_spanned! {span =>
            compile_error!("ModelingCmdVariant cannot be implemented on a union type")
        },
    }
}

fn impl_empty_on_struct(
    name: proc_macro2::Ident,
) -> TokenStream {
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
