use syn::{ItemMod, spanned::Spanned};
use proc_macro2::TokenStream;

pub(crate) fn generate(input: ItemMod) -> TokenStream {
    // Where in the input source code is this type defined?
    let span = input.span();
    todo!()
}