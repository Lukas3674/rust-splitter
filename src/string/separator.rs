
use core::str::from_utf8;
use crate::separator::{SepInfo, Separator};

/// The trait every string separator has to implement
pub trait StrSeparator {
    /// A sting separator will have to say, where the next separator in the string
    /// is located (and what size it has), by providing a [`SepInfo`]
    fn str_separate(&self, ts: &str) -> Option<SepInfo>;
}

impl<S: StrSeparator> Separator<u8> for S {
    #[inline]
    fn separate(&self, ts: &[u8]) -> Option<SepInfo> {
        self.str_separate(from_utf8(ts).expect("valid utf8"))
    }
}

impl StrSeparator for char {
    fn str_separate(&self, ts: &str) -> Option<SepInfo> {
        ts.find(*self).map(|pos| SepInfo { pos, size: self.len_utf8() })
    }
}

impl StrSeparator for &str {
    fn str_separate(&self, ts: &str) -> Option<SepInfo> {
        ts.find(self).map(|pos| SepInfo { pos, size: self.len() })
    }
}

impl StrSeparator for &[&str] {
    fn str_separate(&self, ts: &str) -> Option<SepInfo> {
        self.iter().filter_map(|s| s.str_separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for &[&str; N] {
    fn str_separate(&self, ts: &str) -> Option<SepInfo> {
        self.iter().filter_map(|s| s.str_separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for [&str; N] {
    fn str_separate(&self, ts: &str) -> Option<SepInfo> {
        self.iter().filter_map(|s| s.str_separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}
