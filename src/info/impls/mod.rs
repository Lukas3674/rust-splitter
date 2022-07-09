
use super::Info;

#[cfg(feature = "std")]
/// all automatic [`Info`] implementations for usefull [`std`] types
mod std;

/// all automatic [`Info`] implementations for usefull [`core`] types
mod core;

/// A simple [`Info::Context`] wrapper for [`std`] and [`core`] wrapper types
pub struct WrapperCtx<'a, T, I: Info<'a, T>>(I::Context);

impl<'a, T, I: Info<'a, T>> Default for WrapperCtx<'a, T, I> {
    #[inline]
    fn default() -> Self {
        WrapperCtx(I::Context::default())
    }
}
