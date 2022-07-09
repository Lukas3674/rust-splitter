
use super::StrInfo;

#[cfg(feature = "std")]
/// all automatic [`StrInfo`] implementations for usefull [`std`] types
mod std;

/// all automatic [`StrInfo`] implementations for usefull [`core`] types
mod core;

/// A simple [`StrInfo::Context`] wrapper for [`std`] and [`core`] wrapper types
pub struct WrapperCtx<'a, T: StrInfo<'a>>(T::Context);

impl<'a, T: StrInfo<'a>> Default for WrapperCtx<'a, T> {
    #[inline]
    fn default() -> Self {
        WrapperCtx(T::Context::default())
    }
}
