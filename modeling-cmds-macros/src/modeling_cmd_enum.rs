use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, ItemMod};

pub(crate) fn generate(input: ItemMod) -> TokenStream {
    let span = input.span();

    // Parse all items from the module, to discover which enum variants should exist.
    // Also, find the doc for each enum variant.
    let (variants, docs): (Vec<_>, Vec<_>) = input
        .content
        .iter()
        .next()
        .unwrap()
        .1
        .iter()
        .filter_map(|item| {
            // All modeling commands are public structs.
            let syn::Item::Struct(item) = item else {
                return None;
            };
            let syn::Visibility::Public(_) = item.vis else {
                return None;
            };

            // Copy the struct's docstring. That'll become the docstring for the enum variant.
            let doc = item
                .attrs
                .iter()
                .filter_map(|attr| match &attr.meta {
                    syn::Meta::NameValue(syn::MetaNameValue { path, value, .. }) => {
                        // The attribute should look like #[doc = "..."].
                        // The attribute's key must be "doc".
                        if !path.is_ident("doc") {
                            return None;
                        }
                        // Extract the attribute's value (the docstring's contents).
                        let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(value),
                            ..
                        }) = value
                        else {
                            return None;
                        };
                        let doc = value.value().trim().to_owned();
                        Some(doc)
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            Some((&item.ident, doc))
        })
        .unzip();

    // Output the generated enum.
    quote_spanned! {span=>
        // Emit the module again
        /// Definition of each modeling command.
        #input
        /// Commands that the KittyCAD engine can execute.
        #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
        #[serde(rename_all = "snake_case", tag = "type")]
        #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
        pub enum ModelingCmd {#(
            #[doc = #docs]
            #variants(kittycad_modeling_cmds::each_cmd::#variants),
        )*}
        /// Each modeling command (no parameters or fields).
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, ::parse_display::Display)]
        #[serde(rename_all = "snake_case")]
        #[cfg_attr(not(feature = "unstable_exhaustive"), non_exhaustive)]
        pub enum ModelingCmdEndpoint{#(
            #[doc = #docs]
            #variants,
        )*}
        /// You can easily convert each modeling command with its fields,
        /// into a modeling command without fields.
        impl From<ModelingCmd> for ModelingCmdEndpoint {
            fn from(v: ModelingCmd) -> Self {
                match v {#(
                    ModelingCmd::#variants(_) => Self::#variants,
                )*}
            }
        }
    }
}
