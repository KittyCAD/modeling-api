//! Proc-macros for implementing kittycad-modeling-cmds traits.

use kittycad_modeling_cmds_macros_impl::{
    modeling_cmd_enum, modeling_cmd_output, modeling_cmd_variant, ok_modeling_cmd_response_enum,
};
use proc_macro::TokenStream;
use syn::{DeriveInput, ItemMod};

/// This will derive the trait `ModelingCmdVariant` from the `kittycad-modeling-cmds` crate.
/// Its associated type `output` will be the corresponding modeling command output type.
#[proc_macro_derive(ModelingCmdVariant)]
pub fn derive_modeling_cmd_variant_nonempty(input: TokenStream) -> TokenStream {
    // Parse the input into a stream of Rust syntax tokens.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    // Generate a new stream of Rust syntax tokens from the input stream.
    // Then hand them back to the compiler.
    // It's idiomatic to make your proc macros a thin wrapper around an "impl" function, because it
    // simplifies unit testing. This is recommended in The Rust Book.
    TokenStream::from(modeling_cmd_variant::derive_nonempty(input))
}

/// Generates the ModelingCmd enum from all its variants.
#[proc_macro]
pub fn define_modeling_cmd_enum(item: TokenStream) -> TokenStream {
    let input: ItemMod = syn::parse2(item.into()).unwrap();
    TokenStream::from(modeling_cmd_enum::generate(input))
}

/// Derives `ModelingCmdOutput`.
#[proc_macro_derive(ModelingCmdOutput)]
pub fn derive_modeling_cmd_output(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    TokenStream::from(modeling_cmd_output::derive(input))
}

/// Generates the OkModelingCmdResponse enum from all its variants.
#[proc_macro]
pub fn define_ok_modeling_cmd_response_enum(item: TokenStream) -> TokenStream {
    let input: ItemMod = syn::parse2(item.into()).unwrap();
    TokenStream::from(ok_modeling_cmd_response_enum::generate(input))
}
