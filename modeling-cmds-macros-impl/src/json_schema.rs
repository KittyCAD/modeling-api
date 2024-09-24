use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

pub fn derive(input: DeriveInput) -> TokenStream {
    let _span = input.span();
    quote! {}
}
