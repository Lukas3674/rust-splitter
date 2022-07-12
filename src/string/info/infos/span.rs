
use super::StrInfo;

mod ctx {
    #[derive(Default)]
    pub struct SpanCtx(pub usize); 
}

/// Like a [`core::ops::Range<usize>`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Inclusive start
    pub start: usize,
    /// Exclusive end
    pub end: usize,
}

impl<'a> StrInfo<'a> for Span {
    type Context = ctx::SpanCtx;

    fn generate(ctx: &mut Self::Context, s: &'a str) -> Self {
        let start = ctx.0;
        ctx.0 += s.len();
        Self { start, end: ctx.0 }
    }
}
