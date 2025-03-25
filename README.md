<img alt="Logo" src="./logo.svg" align="right" style="width: 4em; height: 4em;"></img>

# option-chain

A macro for using `?` operator in functions that don't return `Option`.

## Examples

```rust
use option_chain::opt;

struct Test1 {
    a: Option<Test2>,
}
struct Test2 {
    b: Option<Test3>,
}
struct Test3 {
    c: i32,
}

let v = Test1 {
    a: Some(Test2 { b: Some(Test3 { c: 42 }) }),
};
let v = opt!(v.a?.b?.c);
assert_eq!(v.unwrap(), 42);
```

## Implementation

It just wraps the expression in a closure that returns `Option`:

```rust
macro_rules! opt {
    ($e:expr) => {
        {
            || -> Option<_> { Some($e) }()
        }
    };
}
```
