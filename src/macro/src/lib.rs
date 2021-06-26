//! Sub-crate for macros I will be using in this repo.

/// Implement a default version of [`std::fmt::Display`] for a type if that
/// type already implements [`std::fmt::Debug`].
#[macro_export]
macro_rules! impl_display {
    ($r#type: ident) => {
        impl std::fmt::Display for $r#type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
    /*
    ($r#type: ident, < $( $tparams: ty ),* >) => {
        impl< $( $tparams, )* > $r#type< $( $tparams, )* > {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
    */
}

/// Create a hash map
/// 
/// Stolen from `<https://stackoverflow.com/a/27582993>`
#[macro_export]
macro_rules! hash_map {
    { $($key:expr => $value:expr),* } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
     };
}
