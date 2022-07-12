
#[cfg(any(feature = "impls"))]
mod impls;

#[cfg(feature = "infos")]
mod infos;
#[cfg(feature = "infos")]
pub use infos::*;

/// The trait for the elements yielded by the [`crate::StrSplitter`]
/// 
/// ### Example
/// ```rust
/// use splitter::StrInfo;
/// 
/// #[derive(Default)]
/// struct SpanCtx(usize);
/// 
/// struct Span {
///     start: usize,
///     end: usize,
/// }
/// 
/// impl<'a> StrInfo<'a> for Span {
///     type Context = SpanCtx;
/// 
///     fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
///         let start = ctx.0;
///         ctx.0 += ts.len();
///         Self { start, end: ctx.0 }
///     }
/// }
/// ```
pub trait StrInfo<'a>: Sized {
    /// The needed `Context` for the [`StrInfo`], to generate the correct values
    type Context: Default;
    
    /// Generates the [`StrInfo`], based on the provided context an slice
    fn generate(_: &mut Self::Context, _: &'a str) -> Self;
}

impl<'a> StrInfo<'a> for &'a str {
    type Context = ();
    #[inline]
    fn generate(_: &mut Self::Context, ts: &'a str) -> Self {
        ts
    }
}
