
use std::ops::Range;
use splitter::{StrInfo, StrSplitter};

#[derive(Debug, PartialEq, StrInfo)]
struct Custom<'a> {
    content: &'a str,
    span: Range<usize>,
}

fn main() {
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
