
use core::ops::Range;
use std::marker::PhantomData;
use splitter::{StrSplitter, StrInfo, Info};

#[test]
fn impls() {
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
}

#[test]
fn empty() {
    #[derive(Info)]
    struct Empty {}

    #[derive(StrInfo)]
    struct StrEmpty {}
}

#[test]
fn generic() {
    #[derive(Info)]
    struct Generic<'a, T, I: Info<'a, T>> {
        _i: I,
        _m: PhantomData<&'a T>,
    }

    #[derive(StrInfo)]
    struct StrGeneric<'a, I: StrInfo<'a>> {
        _i: I,
        _m: PhantomData<&'a ()>,
    }
}

#[test]
fn multi_generics() {
    #[derive(Info)]
    struct Generic<'a, 'b, 'c, T, U, V, I: Info<'a, T>> {
        _i: I,
        _m1: PhantomData<&'a T>,
        _m2: PhantomData<&'b U>,
        _m3: PhantomData<&'c V>,
    }

    #[derive(StrInfo)]
    struct StrGeneric<'a, 'b, 'c, T, U, V, I: StrInfo<'a>> {
        _i: I,
        _m1: PhantomData<&'a T>,
        _m2: PhantomData<&'b U>,
        _m3: PhantomData<&'c V>,
    }
}
