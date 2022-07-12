
use std::ops::Range;
use splitter::{StrInfo, Info, StrSplitter, Splitter};
#[derive(Debug, PartialEq, Info)]
#[splitter(u8)]
struct Custom<'a> {
    content: &'a [u8],
}

#[derive(Debug, PartialEq, StrInfo)]
struct StrCustom<'a> {
    content: &'a str,
    span: Range<usize>,
}

#[derive(Debug, PartialEq, StrInfo)]
struct TupleCustom<'a>(&'a str);

fn main() {
    let sp = Splitter::new(b"bytes example", b" ").with_info::<Custom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            Custom { content: b"bytes" },
            Custom { content: b" " },
            Custom { content: b"example" },
        ],
    );

    let sp = StrSplitter::new("bytes example", " ").with_info::<StrCustom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            StrCustom { content: "bytes", span: 0..5 },
            StrCustom { content: " ", span: 5..6 },
            StrCustom { content: "example", span: 6..13 },
        ],
    );

    let sp = StrSplitter::new("bytes example", " ").with_info::<TupleCustom>();
    assert_eq!(
        sp.collect::<Vec<_>>(),
        vec![
            TupleCustom("bytes"),
            TupleCustom(" "),
            TupleCustom("example"),
        ],
    );
}
