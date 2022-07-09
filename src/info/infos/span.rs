
use super::Info;

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

impl<'a, T> Info<'a, T> for Span {
    type Context = ctx::SpanCtx;

    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        let start = ctx.0;
        ctx.0 += ts.len();
        Self { start, end: ctx.0 }
    }
}
