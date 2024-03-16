//! A little macro for easy implementation of cxx::ExternType

/// This tells the c++ interop what the native c++ names are
/// from <https://docs.rs/cxx/latest/src/cxx/extern_type.rs.html>
#[macro_export]
macro_rules! impl_extern_type {
    ($([$kind:ident] $($(#[$($attr:tt)*])* $ty:path = $cxxpath:literal)*)*) => {
        $($(
            $(#[$($attr)*])*
            unsafe impl cxx::ExternType for $ty {
                #[allow(unused_attributes)] // incorrect lint; this doc(hidden) attr *is* respected by rustdoc
                #[doc(hidden)]
                type Id = cxx::type_id!($cxxpath);
                type Kind = cxx::kind::$kind;
            }
        )*)*
    };
}
