
use std::ops::Range;
use splitter::{StrInfo, StrSplitter};

#[derive(Default)]
struct CustomCtx {
    cursor: usize,
}

#[derive(Debug, PartialEq)]
struct Custom<'a> {
    content: &'a str,
    span: Range<usize>,
}

impl<'a> StrInfo<'a> for Custom<'a> {
    type Context = CustomCtx;
    fn generate(ctx: &mut Self::Context, s: &'a str) -> Self {
        let start = ctx.cursor;
        ctx.cursor += s.len();
        Custom { content: s, span: start..ctx.cursor }
    }
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
