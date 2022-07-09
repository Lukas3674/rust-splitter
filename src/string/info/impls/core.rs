
use super::StrInfo;
use core::{
    pin::Pin,
    marker::{PhantomData, PhantomPinned},
    mem::ManuallyDrop,
    cell::{ Cell, RefCell, UnsafeCell },
    ops::{ Range, RangeInclusive, Deref },
};

impl<'a, T> StrInfo<'a> for Pin<T>
where
    T: StrInfo<'a> + Deref,
    T::Target: Unpin,
{
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Pin::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T> StrInfo<'a> for PhantomData<T> {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, _ts: &'a str) -> Self {
        PhantomData
    }
}

impl<'a> StrInfo<'a> for PhantomPinned {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, _ts: &'a str) -> Self {
        PhantomPinned
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for ManuallyDrop<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        ManuallyDrop::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for Cell<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Cell::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for RefCell<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        RefCell::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for UnsafeCell<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        UnsafeCell::new(T::generate(&mut ctx.0, ts))
    }
}

#[derive(Default)]
pub struct RangeCtx(usize);

impl<'a> StrInfo<'a> for Range<usize> {
    type Context = RangeCtx;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        let start = ctx.0;
        ctx.0 += ts.len();
        start..ctx.0
    }
}

impl<'a> StrInfo<'a> for RangeInclusive<usize> {
    type Context = RangeCtx;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        let start = ctx.0;
        ctx.0 += ts.len();
        let end = if ctx.0 != 0 {
            ctx.0
        } else {
            0
        };
        start..=end
    }
}
