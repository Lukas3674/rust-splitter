# Splitter

[![API](https://docs.rs/splitter/badge.svg)](https://docs.rs/splitter) [![Crate](https://img.shields.io/crates/v/splitter.svg)](https://crates.io/crates/splitter)

### A string and slice splitter library

## String Example
```rust
use splitter::StrSplitter;

let sp = StrSplitter::new("bytes example", " ");
assert_eq!(
    sp.collect::<Vec<_>>(),
    vec!["bytes", " ", "example"],
);
```

## Slice Example
```rust
use splitter::Splitter;

let sp = Splitter::new(&[1, 2, 3, 3, 4], [[2], [4]]);
let re: Vec<&[usize]> = vec![&[1], &[2], &[3, 3], &[4]];
assert_eq!(sp.collect::<Vec<_>>(), re);
```

## Derive Example (`derive` and `impls` - feature)
```rust
use std::ops::Range;
use splitter::{StrInfo, StrSplitter};

#[derive(Debug, PartialEq, StrInfo)]
struct Custom<'a> {
    content: &'a str,
    span: Range<usize>,
}

let sp = StrSplitter::new("bytes example", " ").with_info::<Custom>();
assert_eq!(
    sp.collect::<Vec<_>>(),
    vec![
        Custom { content: "bytes", span: 0..5 },
        Custom { content: " ", span: 5..6 },
        Custom { content: "example", span: 6..13 },
    ],
);
```

## Derive Info
When using the derive macro, and lifetimes are provided, the first lifetime
has to be the lifetime of the slice or of the string.

Same with generic type parameters, the first generic type parameter
has to be the the type of the slice elements.

## Features
- `std` - enables the standard library (currently only used with `impls` - feature)
- `impls` - automatically implements `Info` and `StrInfo` for usefull types from `core` or `std`
- `infos` - adds pre-defined usefull `Info` and `StrInfo` types
- `derive` - enables the `Info` and `StrInfo` derive macro
- `full` - enables all features

## Automatic Implementations

### `impls` - feature
- `core::pin::Pin`
- `core::marker::PhantomData`
- `core::marker::PhantomPinned`
- `core::mem::ManuallyDrop`
- `core::cell::Cell`
- `core::cell::RefCell`
- `core::cell::UnsafeCell`
- `core::ops::Range`
- `core::ops::RangeInclusive`

### `impls` and `std` - feature
- `std::boxed::Box`
- `std::rc::Rc`
- `std::sync::Arc`
- `std::sync::Mutex`
- `std::sync::RwLock`
- `std::vec::Vec` (only for slices)
- `std::path::PathBuf` (only for strings)
- `std::string::String` (only for strings)
