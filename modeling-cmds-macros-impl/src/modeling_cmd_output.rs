use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, DeriveInput};

pub fn derive(input: DeriveInput) -> TokenStream {
    // Where in the input source code is this type defined?
    let span = input.span();
    // Name of type that is deriving the trait.
    let name = input.ident;
    quote_spanned! {span=>
        impl kittycad_modeling_cmds::traits::ModelingCmdOutput for #name {}
    }
}
