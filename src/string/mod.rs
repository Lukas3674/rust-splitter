
pub mod info;
pub mod separator;

pub use info::StrInfo;

use separator::StrSeparator;
use crate::splitter::Splitter;
use core::{num::NonZeroUsize, str::from_utf8};

/// The string splitter
/// 
/// ### Usage
/// ```rust
/// use splitter::StrSplitter;
/// 
/// let sp = StrSplitter::new("bytes example", " ");
/// assert_eq!(
///     sp.collect::<Vec<_>>(),
///     vec!["bytes", " ", "example"],
/// );
/// ```
pub struct StrSplitter<'a, I, S>
where
    I: StrInfo<'a>,
    S: StrSeparator,
{
    /// internally uses a [`Splitter`] over a byte slice
    #[allow(clippy::type_complexity)]
    splitter: Splitter<'a, u8, &'a [u8], S, fn(&'a [u8]) -> NonZeroUsize>,
    ctx: I::Context,
}

impl<'a, S> StrSplitter<'a, &'a str, S>
where
    S: StrSeparator,
{
    /// Create a new [`StrSplitter`]
    /// 
    /// `content`: the input string
    /// 
    /// `sep`: any type of separator, that implements [`StrSeparator`]
    pub fn new(content: &'a str, sep: S) -> Self {

        /// Override for the increment of the cursor, to the size of the next [`char`]
        fn increment(s: &[u8]) -> NonZeroUsize {
            let next = from_utf8(s).expect("valid utf8").chars().next();
            // SAFETY: This function will only be called when the content is
            // not empty, meaning there is at least one char.
            let len = unsafe { next.unwrap_unchecked().len_utf8() };
            // SAFETY: The utf8-len of a char is never zero
            unsafe { NonZeroUsize::new_unchecked(len) }
        }

        Self {
            splitter: Splitter::new(content.as_bytes(), sep).with_increment(increment),
            ctx: (),
        }
    }
}

impl<'a, H, S> StrSplitter<'a, H, S>
where
    H: StrInfo<'a>,
    S: StrSeparator,
{
    /// Change the type of [`StrInfo`], that is yielded by the [`StrSplitter`]
    pub fn with_info<I: StrInfo<'a>>(self) -> StrSplitter<'a, I, S> {
        StrSplitter { splitter: self.splitter, ctx: I::Context::default() }
    }
}

impl<'a, I, S> Iterator for StrSplitter<'a, I, S>
where
    I: StrInfo<'a>,
    S: StrSeparator,
{
    type Item = I;

    /// Get the next [`StrInfo`] from the [`StrSplitter`]
    fn next(&mut self) -> Option<Self::Item> {
        self.splitter.next().map(|b|
            I::generate(&mut self.ctx, from_utf8(b).expect("valid utf8"))
        )
    }
}
