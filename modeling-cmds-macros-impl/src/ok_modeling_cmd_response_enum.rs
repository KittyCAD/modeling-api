use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, ItemMod};

pub fn generate(input: ItemMod) -> TokenStream {
    let span = input.span();

    // Parse all items from the module, to discover which enum variants should exist.
    // Also, create the doc for each enum variant.
    let items = &input.content.as_ref().unwrap().1;
    let variants = items
        .iter()
        .filter_map(|item| {
            // All modeling commands are public structs.
            match item {
                syn::Item::Struct(item) if matches!(item.vis, syn::Visibility::Public(_)) => Some(&item.ident),
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    // Output the generated enum.
    quote_spanned! {span=>
        // Emit the module again
        #input
        /// A successful response from a modeling command.
        /// This can be one of several types of responses, depending on the command.
        #[derive(Debug, Serialize, Deserialize, Clone)]
        #[serde(rename_all = "snake_case", tag = "type", content = "data")]
        #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
        pub enum OkModelingCmdResponse {
            /// An empty response, used for any command that does not explicitly have a response
            /// defined here.
            Empty,
            #(#[doc = concat!("The response to the '", stringify!(#variants), "' endpoint.")] #variants(output::#variants),)*
        }

        // Loop over `variants`, generate N different `From` impls on the enum,
        // each of which corresponds to a variant. This way each individual output can be converted
        // into the enum.
        #(
        impl From<output::#variants> for OkModelingCmdResponse {
            fn from(x: output::#variants) -> Self {
                Self::#variants(x)
            }
        }
        )*

        // The `Empty` enum variant is a bit different, doesn't conform to the same pattern.
        // So define it manually.
        impl From<()> for OkModelingCmdResponse {
            fn from(_: ()) -> Self {
                Self::Empty
            }
        }
    }
}
