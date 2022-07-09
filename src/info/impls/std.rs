
use super::Info;
use std::{
    boxed::Box,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
    vec::Vec,
};

impl<'a, T: Clone> Info<'a, T> for Vec<T> {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        ts.to_vec()
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for Box<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Box::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for Rc<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Rc::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for Arc<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Arc::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for Mutex<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        Mutex::new(I::generate(&mut ctx.0, ts))
    }
}

impl<'a, T, I: Info<'a, T>> Info<'a, T> for RwLock<I> {
    type Context = super::WrapperCtx<'a, T, I>;
    fn generate(ctx: &mut Self::Context, ts: &'a [T]) -> Self {
        RwLock::new(I::generate(&mut ctx.0, ts))
    }
}
