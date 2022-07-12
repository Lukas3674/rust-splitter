
fn main() {
    slice::run();
    string::run();
}

mod slice {
    use std::ops::Range;
    use splitter::{Info, Splitter};

    pub fn run() {
        #[derive(Default)]
        struct CustomCtx {
            cursor: usize,
        }
        
        #[derive(Debug, PartialEq)]
        struct Custom<'a, T> {
            content: &'a [T],
            span: Range<usize>,
        }
        
        impl<'a, T> Info<'a, T> for Custom<'a, T> {
            type Context = CustomCtx;
            fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
                let start = ctx.cursor;
                ctx.cursor += ts.len();
                Custom { content: ts, span: start..ctx.cursor }
            }
        }
        
        let sp = Splitter::new(b"bytes example", b" ").with_info::<Custom<u8>>();
        assert_eq!(
            sp.collect::<Vec<_>>(),
            vec![
                Custom { content: b"bytes", span: 0..5 },
                Custom { content: b" ", span: 5..6 },
                Custom { content: b"example", span: 6..13 },
            ],
        );
    }
}

mod string {
    use std::ops::Range;
    use splitter::{StrInfo, StrSplitter};

    pub fn run() {
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
}
