use syn::{ItemMod, spanned::Spanned};
use proc_macro2::TokenStream;

pub(crate) fn generate(input: ItemMod) -> TokenStream {
    let _span = input.span();
    todo!()
}