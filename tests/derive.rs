
use core::ops::Range;
use std::marker::PhantomData;
use splitter::{StrSplitter, Splitter, StrInfo, Info};

#[test]
fn impls() {
    #[derive(Debug, PartialEq, Info)]
    #[splitter(u8)]
    struct Custom<'a> {
        content: &'a [u8],
    }

    let sp = Splitter::new(b"bytes example", b" ").with_info::<Custom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Custom { content: b"bytes" },
            Custom { content: b" " },
            Custom { content: b"example" },
        ],
    );

    #[derive(Debug, PartialEq, StrInfo)]
    struct StrCustom<'a> {
        content: &'a str,
        span: Range<usize>,
    }

    let sp = StrSplitter::new("bytes example", " ").with_info::<StrCustom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            StrCustom { content: "bytes", span: 0..5 },
            StrCustom { content: " ", span: 5..6 },
            StrCustom { content: "example", span: 6..13 },
        ],
    );
}

#[test]
fn empty() {
    #[derive(Info)]
    struct Empty {}

    #[derive(StrInfo)]
    struct StrEmpty {}

    #[derive(Info)]
    struct SemiEmpty {
        _m: PhantomData<()>,
    }

    #[derive(StrInfo)]
    struct SemiStrEmpty {
        _m: PhantomData<()>,
    }
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

#[test]
fn structs() {
    #[derive(Info)]
    #[splitter(u8)]
    struct _1<'a> {
        _1: &'a [u8],
    }

    #[derive(Info)]
    #[splitter(T)]
    struct _2<'a, T> {
        _1: &'a [T],
    }

    #[derive(Info)]
    #[splitter(T)]
    struct _3<'a, T, E>
    where
        &'a [E]: Info<'a, T>
    {
        _1: &'a [T],
        _2: &'a [E],
    }

    #[derive(Info)]
    #[splitter(E)]
    struct _4<'a, T, E>
    where
        &'a [T]: Info<'a, E>
        {
        _1: &'a [T],
        _2: &'a [E],
    }

    #[derive(Info)]
    #[splitter(E)]
    struct _5<'a, T, E> {
        _1: &'a [E],
        _2: PhantomData<T>,
    }
    #[derive(StrInfo)]
    struct _1Str<'a> {
        _1: &'a str,
    }

    #[derive(StrInfo)]
    struct _2Str<'a, T>
    where
        &'a [T]: StrInfo<'a>
    {
        _1: &'a [T],
    }

    #[derive(StrInfo)]
    struct _5Str<'a, T, E>
    where
        &'a [E]: StrInfo<'a>
    {
        _1: &'a [E],
        _2: PhantomData<T>,
    }
}

#[test]
fn tuple() {
    #[derive(Debug, PartialEq, Info)]
    #[splitter(u8)]
    struct Custom<'a>(&'a [u8]);

    let sp = Splitter::new(b"bytes example", b" ").with_info::<Custom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Custom(b"bytes"),
            Custom(b" "),
            Custom(b"example"),
        ],
    );

    #[derive(Debug, PartialEq, StrInfo)]
    struct StrCustom<'a>(&'a str);

    let sp = StrSplitter::new("bytes example", " ").with_info::<StrCustom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            StrCustom("bytes"),
            StrCustom(" "),
            StrCustom("example"),
        ],
    );
}
