/// Derive helpers for implementing unit conversions for enum types.

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

/// Implement unit conversions based on an enum.
#[proc_macro_derive(UnitConversion)]
pub fn derive_unit_conversions(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive(parse_macro_input!(input)).into()
}

fn derive(item: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = &item.ident;

    let measurement_struct_name = format_ident!("{}", struct_name.to_string().replace("Unit", ""));

    // Make sure this is an enum.
    match &item.data {
        syn::Data::Enum(_) => {}
        // Return early if this is not an enum.
        _ => return quote!(),
    }

    // Get all the variants of the enum.
    let variants: Vec<&proc_macro2::Ident> = match &item.data {
        syn::Data::Enum(data) => data.variants.iter().map(|v| &v.ident).collect(),
        _ => unreachable!(),
    };

    let mut items = quote!();
    for variant in variants.clone() {
        // Iterate over the variants again.
        for to_variant in variants.clone() {
            if variant == to_variant {
                // If these two are equal our enum part is easier.
                items =
                    quote! {
                        #items
                        (#struct_name::#variant, #struct_name::#to_variant) => {
                            input
                        }
                    };
            } else {
                let from_fn = format_ident!("from_{}", clean_fn_name(&variant.to_string()));
                let to_fn = format_ident!("as_{}", clean_fn_name(&to_variant.to_string()));
                // Generate the conversions part of the function.
                items = quote! {
                    #items
                    (#struct_name::#variant, #struct_name::#to_variant) => {
                        let value = measurements::#measurement_struct_name::#from_fn(input);
                        value.#to_fn()
                    }
                };
            }
        }
    }

    quote! {
        impl #struct_name {
            /// Do a unit conversion for this type.
            pub fn convert_to(&self, to: #struct_name, input: f64) -> f64 {
                match (self, to) {
                    #items
                }
            }
        }
    }
}
// Rewrite some names to match the measurements lib.
fn clean_fn_name(name: &str) -> String {
    inflections::case::to_snake_case(
        &name
            .replace("Electronvolts", "e_v")
            .replace("Kilocalories", "Kcalories"),
    )
}
