#![doc = include_str!("../README.md")]
#![no_std]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

/// A macro for using `?` operator in functions that don't return `Option`.
///
/// See [crate level documentation](crate) for more information.
#[macro_export]
macro_rules! opt {
    ($e:expr) => {
        {
            || -> ::core::option::Option<_> { ::core::option::Option::Some($e) }()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_named() {
        struct Test1 {
            a: Option<Test2>,
        }
        struct Test2 {
            b: Option<Test3>,
        }
        struct Test3 {
            c: i32,
        }

        let v1 = Test1 {
            a: None,
        };
        let v1 = opt!(v1.a?.b?.c);
        assert!(v1.is_none());

        let v2 = Test1 {
            a: Some(Test2 { b: None }),
        };
        let v2 = opt!(v2.a?.b?.c);
        assert!(v2.is_none());

        let v3 = Test1 {
            a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
        };
        let v3 = opt!(v3.a?.b?.c);
        assert_eq!(v3.unwrap(), 42);
    }

    #[test]
    fn test_macro_unnamed() {
        struct Test1(Option<Test2>);
        struct Test2(Option<Test3>);
        struct Test3(i32);

        let v1 = Test1(None);
        let v1 = opt!(v1.0?.0?.0);
        assert!(v1.is_none());

        let v2 = Test1(Some(Test2(None)));
        let v2 = opt!(v2.0?.0?.0);
        assert!(v2.is_none());

        let v3 = Test1(Some(Test2(Some(Test3(42)))));
        let v3 = opt!(v3.0?.0?.0);
        assert_eq!(v3.unwrap(), 42);
    }

    #[test]
    fn test_macro_mixed() {
        struct Test1 {
            a: Option<Test2>,
        }
        struct Test2(Option<Test3>);
        struct Test3(i32);

        let v1 = Test1 {
            a: None,
        };
        let v1 = opt!(v1.a?.0?.0);
        assert!(v1.is_none());

        let v2 = Test1 {
            a: Some(Test2(None)),
        };
        let v2 = opt!(v2.a?.0?.0);
        assert!(v2.is_none());

        let v3 = Test1 {
            a: Some(Test2(Some(Test3(42)))),
        };
        let v3 = opt!(v3.a?.0?.0);
        assert_eq!(v3.unwrap(), 42);
    }
}
