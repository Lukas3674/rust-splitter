
use super::StrInfo;
use std::{
    boxed::Box,
    path::PathBuf,
    rc::Rc,
    string::String,
    sync::{Arc, Mutex, RwLock},
};

impl<'a, T: StrInfo<'a>> StrInfo<'a> for Box<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Box::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a> StrInfo<'a> for PathBuf {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, ts: &'a str) -> Self {
        PathBuf::from(ts)
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for Rc<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Rc::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a> StrInfo<'a> for String {
    type Context = ();
    fn generate(_ctx: &mut Self::Context, ts: &'a str) -> Self {
        ts.to_owned()
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for Arc<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Arc::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for Mutex<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        Mutex::new(T::generate(&mut ctx.0, ts))
    }
}

impl<'a, T: StrInfo<'a>> StrInfo<'a> for RwLock<T> {
    type Context = super::WrapperCtx<'a, T>;
    fn generate(ctx: &mut Self::Context, ts: &'a str) -> Self {
        RwLock::new(T::generate(&mut ctx.0, ts))
    }
}
