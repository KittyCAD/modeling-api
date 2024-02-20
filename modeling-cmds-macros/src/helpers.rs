/// Remove the defaults from a generic type.
/// For example, turns <T = f32> into <T>.
/// This is useful because defaults like that are valid when declaring a type, but should NOT
/// be included everywhere the type gets used.
/// E.g. you can't say `struct Foo { field: Option<T = f32> }`
pub fn remove_generics_defaults(mut g: syn::Generics) -> syn::Generics {
    for generic_param in g.params.iter_mut() {
        if let syn::GenericParam::Type(type_param) = generic_param {
            type_param.default = None;
        }
    }
    g
}
