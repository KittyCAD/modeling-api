//! Proc-macros for implementing execution-plan traits.

mod derive_value;
mod helpers;

use self::derive_value::impl_derive_value;
use proc_macro::TokenStream;
use syn::DeriveInput;

/// This will derive the trait `Value` from the `kittycad-execution-plan-traits` crate.
#[proc_macro_derive(ExecutionPlanValue)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    // Parse the input into a stream of Rust syntax tokens.
    let input: DeriveInput = syn::parse2(input.into()).unwrap();
    // Generate a new stream of Rust syntax tokens from the input stream.
    // Then hand them back to the compiler.
    // It's idiomatic to make your proc macros a thin wrapper around an "impl" function, because it
    // simplifies unit testing. This is recommended in The Rust Book.
    TokenStream::from(impl_derive_value(input))
}
