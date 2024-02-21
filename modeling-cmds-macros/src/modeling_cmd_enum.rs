use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, ItemMod};

pub(crate) fn generate(input: ItemMod) -> TokenStream {
    let span = input.span();

    // Parse all items from the module, to discover which enum variants should exist.
    // Also, find the doc for each enum variant.
    let (variants, docs): (Vec<_>, Vec<_>) = input
        .clone()
        .content
        .into_iter()
        .next()
        .unwrap()
        .1
        .into_iter()
        .filter_map(|item| {
            // All modeling commands are public structs.
            let syn::Item::Struct(item) = item else {
                return None;
            };
            let syn::Visibility::Public(_) = item.vis else {
                return None;
            };

            // Copy the struct's docstring. That'll become the docstring for the enum variant.
            let doc: Vec<String> = item
                .attrs
                .into_iter()
                .filter_map(|attr| match attr.meta {
                    syn::Meta::NameValue(syn::MetaNameValue { path, value, .. }) => {
                        if !path.is_ident("doc") {
                            return None;
                        }
                        let syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(doc), ..}) = value else {
                            return None;
                        };
                        let doc = doc.value().trim().to_owned();
                        Some(doc)
                    }
                    _ => None,
                })
                .collect();
            let doc: String = doc.join("\n");
            Some((item.ident, doc))
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
        // TODO: rename to ModelingCmd
        pub enum ModelingCmd {
            #(#[doc = #docs] #variants(kittycad_modeling_cmds::each_cmd::#variants),)*
        }
    }
}
