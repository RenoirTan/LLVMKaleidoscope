#[test]
fn test_impl_display() {
    use crate::impl_display;

    #[derive(Debug)]
    struct Thing {
        pub field1: i64,
        pub field2: String
    }
    impl_display!(Thing);
    
    let thing = Thing {
        field1: 16,
        field2: String::from("hmm")
    };
    assert_eq!(format!("{}", thing), format!("{:?}", thing));
}

#[test]
fn test_hash_map() {
    use std::collections::HashMap;
    use crate::hash_map;

    let dict: HashMap<&'static str, i32> = hash_map!{
        "item" => 1,
        "thing" => 2
    };
    assert!(matches!(dict.get("item"), Some(1)));
    assert!(matches!(dict.get("thing"), Some(2)));
    assert!(matches!(dict.get("else"), None));
}

#[test]
fn test_function_path() {
    use crate::function_path;

    assert_eq!(
        function_path!(),
        "kaleidoscope_macro::tests::test_function_path"
    );
}

#[test]
fn test_function_name() {
    use crate::function_name;

    assert_eq!(function_name!(), "test_function_name");
}

#[test]
fn test_ok_none() {
    use std::{
        convert::Infallible,
        option::Option,
        result::Result
    };
    use crate::ok_none;

    fn converter<T>(thing: Option<T>) -> Result<Option<T>, Infallible>  {
        Ok(Some(ok_none!(thing)))
    }

    assert_eq!(converter(Some(1)), Ok(Some(1)));
    assert_eq!(converter::<i32>(None), Ok(None));
}

#[test]
fn test_iterator_to_str() {
    use crate::iterator_to_str;

    let iterable = 0..5;
    assert_eq!(
        iterator_to_str!(iterable, " < "),
        "0 < 1 < 2 < 3 < 4"
    );
}
