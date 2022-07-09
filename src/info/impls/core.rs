
use super::Info;
use core::{
    pin::Pin,
    marker::{PhantomData, PhantomPinned},
    mem::ManuallyDrop,
    cell::{ Cell, RefCell, UnsafeCell },
    ops::{ Range, RangeInclusive, Deref },
};

impl<'a, T, I> Info<'a, T> for Pin<I>
where
    I: Info<'a, T> + Deref,
    I::Target: Unpin,
{
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Pin::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I> Info<'a, T> for PhantomData<I> {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, _ts: &'a [T]) -> Self {
        PhantomData
    }
}

impl<'a, T> Info<'a, T> for PhantomPinned {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, _ts: &'a [T]) -> Self {
        PhantomPinned
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for ManuallyDrop<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        ManuallyDrop::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for Cell<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Cell::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for RefCell<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        RefCell::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for UnsafeCell<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        UnsafeCell::new(I::generate(&mut ctx.0, ts))
    }
}

#[derive(Default)]
pub struct RangeCtx(usize);

impl<'a, T> Info<'a, T> for Range<usize> {
    type Context = RangeCtx;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        let start = ctx.0;
        ctx.0 += ts.len();
        start..ctx.0
    }
}

impl<'a, T> Info<'a, T> for RangeInclusive<usize> {
    type Context = RangeCtx;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
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
