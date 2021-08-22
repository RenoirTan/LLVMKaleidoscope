//! Sub-crate for macros I will be using in this repo.


#[cfg(test)]
mod tests;


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
/// Stolen from <https://stackoverflow.com/a/27582993>
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

/// See [`function_path`].
#[macro_export]
macro_rules! untrimmed_function_path {
    () => {{
        // Fully qualified name = "path::to::current_function::f"
        fn f() {}
        // f's identifier to string
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
    }};
}

/// Get the full qualified name of the current function/method.
/// 
/// Stolen from <https://stackoverflow.com/a/40234666>
#[macro_export]
macro_rules! function_path {
    () => {{
        use $crate::untrimmed_function_path;
        let name = untrimmed_function_path!();
        // Shave off "::f" in "path::to::current_function::f"
        &name[..name.len() - 3]
    }};
}

/// Get the name of the function (no paths) of the current function/method.
/// 
/// Stolen from <https://stackoverflow.com/a/63904992>
#[macro_export]
macro_rules! function_name {
    () => {{
        use $crate::untrimmed_function_path;
        let name = untrimmed_function_path!();
        // Shave off trailing "::f" and preserve only the characters after
        // the first "::" from the right.
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

/// Unwrap an [`Option`] but return `Ok(None)` if the [`Option`] is [`None`].
#[macro_export]
macro_rules! ok_none {
    ($option: expr) => {
        match $option {
            Some(t) => t,
            None => return Ok(None)
        }
    };
}

/// If `$option` is [`Some`], return `Ok($option)`.
#[macro_export]
macro_rules! return_ok_some {
    ($option: ident) => {{
        if $option.is_some() {
            return Ok($option);
        }
    }};
    ($option: expr => $retval: expr) => {{
        if $option.is_some() {
            return Ok($retval);
        }
    }};
}

/// Convert an iterator into a string with each element being a separated by
/// a string.
/// 
/// # Example
/// 
/// ```
/// use kaleidoscope_macro::iterator_to_str;
/// 
/// let sequence = vec![1, 2, 3, 4, 5];
/// let string = iterator_to_str!(sequence.iter(), " ??? ");
/// assert_eq!(string, "1 ??? 2 ??? 3 ??? 4 ??? 5");
/// ```
#[macro_export]
macro_rules! iterator_to_str {
    ($iterator: expr, $separator: expr) => {
        $iterator
            .map(|e| format!("{}", e))
            .collect::<Vec<String>>()
            .join($separator)
    };
    ($iterator: expr) => {
        iterator_to_str!($iterator, "")
    }
}
