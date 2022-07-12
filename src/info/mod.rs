
#[cfg(feature = "impls")]
mod impls;

#[cfg(feature = "infos")]
mod infos;
#[cfg(feature = "infos")]
pub use infos::*;

/// The trait for the elements yielded by the [`crate::Splitter`]
/// 
/// ### Example
/// ```rust
/// use splitter::Info;
/// 
/// #[derive(Default)]
/// struct SpanCtx(usize);
/// 
/// struct Span {
///     start: usize,
///     end: usize,
/// }
/// 
/// impl<'a, T> Info<'a, T> for Span {
///     type Context = SpanCtx;
/// 
///     fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
///         let start = ctx.0;
///         ctx.0 += ts.len();
///         Self { start, end: ctx.0 }
///     }
/// }
/// ```
pub trait Info<'a, T>: Sized {
    /// The needed `Context` for the [`Info`], to generate the correct values
    type Context: Default;

    /// Generates the [`Info`], based on the provided context an slice
    fn generate(_: &mut Self::Context, _: &'a [T]) -> Self;
}

impl<'a, T> Info<'a, T> for &'a [T] {
    type Context = ();
    #[inline]
    fn generate(_: &mut Self::Context, ts: &'a [T]) -> Self {
        ts
    }
}
