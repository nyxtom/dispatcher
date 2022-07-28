# eventroute

[![latest version](https://img.shields.io/crates/v/eventroute.svg)](https://crates.io/crates/eventroute)
[![documentation](https://docs.rs/eventroute/badge.svg)](https://docs.rs/eventroute)
![license](https://img.shields.io/crates/l/eventroute.svg)

*eventroute* provides a type safe middleware routing using generics rather than a specific event name or identifier. Middleware can be directly passed into `.on` while `.emit` provides a simple mechanism for dispatching to callback middleware.

## Examples

```rust
pub struct Test {
    foo: i32,
}

pub struct Foo {
    bar: i32,
}

#[test]
fn test_router() {
    let mut router = eventroute::new();
    router.on(|i: i32| {
        println!("{}", i * 10);
    });
    router.on(|(a, b): (i32, i32)| {
        println!("{}", a * b);
    });
    router.on(|test: Test| {
        println!("test {}", test.foo);
    });
    router.on(|foo: Foo| {
        println!("bar {}", foo.bar);
    });

    router.emit(3);
    router.emit((2, 3));
    router.emit(Foo { bar: 232 });
    router.emit(Test { foo: 232 });
}
```
