//! Proc-macros for implementing kittycad-modeling-cmds traits.

mod modeling_cmd_enum;
mod modeling_cmd_output;
mod modeling_cmd_variant;

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemMod};

/// This will derive the trait `ModelingCmdVariant` from the `kittycad-modeling-cmds` crate.
/// Its associated type `output` will be ().
#[proc_macro_derive(ModelingCmdVariantEmpty)]
pub fn derive_modeling_cmd_variant_empty(input: TokenStream) -> TokenStream {
    // Parse the input into a stream of Rust syntax tokens.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    // Generate a new stream of Rust syntax tokens from the input stream.
    // Then hand them back to the compiler.
    // It's idiomatic to make your proc macros a thin wrapper around an "impl" function, because it
    // simplifies unit testing. This is recommended in The Rust Book.
    TokenStream::from(modeling_cmd_variant::derive_empty(input))
}

/// This will derive the trait `ModelingCmdVariant` from the `kittycad-modeling-cmds` crate.
/// Its associated type `output` will be the corresponding modeling command output type.
#[proc_macro_derive(ModelingCmdVariant)]
pub fn derive_modeling_cmd_variant_nonempty(input: TokenStream) -> TokenStream {
    // For comments, see `derive_modeling_cmd_variant_empty`.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
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
    // For comments, see `derive_modeling_cmd_output_empty`.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    TokenStream::from(modeling_cmd_output::derive(input))
}
