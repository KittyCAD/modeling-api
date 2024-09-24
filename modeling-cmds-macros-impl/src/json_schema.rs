use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

/// Doesn't actually do anything. No-op to avoid deriving the JsonSchema trait
/// and therefore bringing schemars to do a shit ton of codegen.
pub fn derive(input: DeriveInput) -> TokenStream {
    let _span = input.span();
    quote! {}
}
