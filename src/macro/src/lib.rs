//! Sub-crate for macros I will be using in this repo.

/// Implement a default version of [`std::fmt::Display`] for a type if that
/// type already implements [`std::fmt::Debug`].
/// 
/// I tried to create a separate arm to handle generics but for some reason
/// the compiler wouldn't treat the type parameters as valid. So for types
/// with use generics, you have to manually implement [`std::fmt::Display`]
/// yourself or derive it if your type allows it.
/// 
/// # Example
/// 
/// ```
/// use kaleidoscope_macro::impl_display;
/// 
/// #[derive(Debug)]
/// pub struct NumberWrapper {
///     pub num: i32
/// }
/// 
/// impl_display!(NumberWrapper);
/// 
/// println!("{}", NumberWrapper {num: 5});
/// ```
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
