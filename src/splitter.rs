
use crate::{
    info::Info,
    separator::{Separator, SepInfo},
};
use core::num::NonZeroUsize;

/// The base splitter over any type of slice
/// 
/// ### Usage
/// ```rust
/// use splitter::Splitter;
/// 
/// let sp = Splitter::new(&[1, 2, 3, 3, 4], [[2], [4]]);
/// let re: Vec<&[usize]> = vec![&[1], &[2], &[3, 3], &[4]];
/// assert_eq!(sp.collect::<Vec<_>>(), re);
/// ```
pub struct Splitter<'a, T, I, S, F>
where
    I: Info<'a, T>,
    S: Separator<T>,
    F: Fn(&'a [T]) -> NonZeroUsize,
{
    cursor: usize,
    content: &'a [T],
    sep: S,
    ctx: I::Context,
    next: Option<NonZeroUsize>,
    increment: F,
}

impl<'a, T, S> Splitter<'a, T, &'a [T], S, fn(&'a [T]) -> NonZeroUsize>
where
    S: Separator<T>,
{
    /// Create a new [`Splitter`]
    /// 
    /// `content`: the input slice
    /// 
    /// `sep`: any type of separator, that implements [`Separator<T>`]
    pub fn new(content: &'a [T], sep: S) -> Self {

        /// the default increment of the cursor, when the separator size is zero
        const fn increment<T>(_: &[T]) -> NonZeroUsize {
            // SAFETY: constant 1
            unsafe { NonZeroUsize::new_unchecked(1) }
        }

        Self { cursor: 0, content, sep, ctx: (), next: None, increment }
    }
}

impl<'a, T, H, S, E> Splitter<'a, T, H, S, E>
where
    H: Info<'a, T>,
    S: Separator<T>,
    E: Fn(&'a [T]) -> NonZeroUsize,
{
    /// Change the type of [`Info`], that is yielded by the [`Splitter`]
    pub fn with_info<I: Info<'a, T>>(self) -> Splitter<'a, T, I, S, E> {
        Splitter {
            cursor: self.cursor,
            content: self.content,
            sep: self.sep,
            ctx: I::Context::default(),
            next: self.next,
            increment: self.increment,
        }
    }

    /// Change the increment of the cursor, when the separator size is zero
    pub(crate) fn with_increment<F>(self, increment: F) -> Splitter<'a, T, H, S, F>
    where
        F: Fn(&'a [T]) -> NonZeroUsize,
    {
        Splitter {
            cursor: self.cursor,
            content: self.content,
            sep: self.sep,
            ctx: self.ctx,
            next: self.next,
            increment,
        }
    }
}

impl<'a, T, I, S, F> Iterator for Splitter<'a, T, I, S, F>
where
    I: Info<'a, T>,
    S: Separator<T>,
    F: Fn(&'a [T]) -> NonZeroUsize,
{
    type Item = I;

    /// Get the next [`Info`] from the [`Splitter`]
    fn next(&mut self) -> Option<Self::Item> {
        let con = &self.content[self.cursor..];

        // if we have a next, just take and return it
        self.next.take().map(|len| {
            let len = len.get();
            self.cursor += len;
            I::generate(&mut self.ctx, &con[..len])
        }).or_else(|| {
            let con_len = con.len();
            (con_len != 0).then(|| {
                // if the content is not empty, get the next [`SepInfo`]
                match self.sep.separate(con) {
                    // if the separator is at the begining,
                    // we can't set self.next, so we can only return the separator itself
                    Some(SepInfo { pos: 0, size }) => {
                        let size = NonZeroUsize::new(size)
                            .unwrap_or_else(|| (self.increment)(con))
                            .get();
                        self.cursor += size;
                        I::generate(&mut self.ctx, &con[..size])
                    }
                    // if the separator is not at the begining,
                    // can return the part before it, and save the separator in self.next
                    Some(SepInfo { pos, size }) => {
                        self.cursor += pos;
                        self.next = Some(NonZeroUsize::new(size)
                            .unwrap_or_else(|| (self.increment)(con)));
                        I::generate(&mut self.ctx, &con[..pos])
                    },
                    // If there was no separator in the slice, return the rest
                    None => {
                        self.cursor += con_len;
                        I::generate(&mut self.ctx, con)
                    }
                }
            })
        })
    }
}
