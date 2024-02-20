//! Proc-macros for implementing execution-plan traits.

mod derive_modeling_cmd_output;

use proc_macro::TokenStream;
use syn::DeriveInput;

/// This will derive the trait `ModelingCmdVariant` from the `kittycad-modeling-cmds` crate.
/// Its associated type will be ().
#[proc_macro_derive(ModelingCmdVariantEmpty)]
pub fn derive_modeling_cmd_output_empty(input: TokenStream) -> TokenStream {
    // Parse the input into a stream of Rust syntax tokens.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    // Generate a new stream of Rust syntax tokens from the input stream.
    // Then hand them back to the compiler.
    // It's idiomatic to make your proc macros a thin wrapper around an "impl" function, because it
    // simplifies unit testing. This is recommended in The Rust Book.
    TokenStream::from(derive_modeling_cmd_output::impl_empty(input))
}
