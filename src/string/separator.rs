
use core::str::from_utf8;
use crate::separator::{SepInfo, Separator};

/// The trait every string separator has to implement
pub trait StrSeparator {
    /// A sting separator will have to say, where the next separator in the string
    /// is located (and what size it has), by providing a [`SepInfo`]
    fn str_separate(&self, _: &str) -> Option<SepInfo>;
}

impl<S: StrSeparator> Separator<u8> for S {
    #[inline]
    fn separate(&self, bs: &[u8]) -> Option<SepInfo> {
        self.str_separate(from_utf8(bs).expect("valid utf8"))
    }
}

impl StrSeparator for char {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        s.find(*self).map(|pos| SepInfo { pos, size: self.len_utf8() })
    }
}

impl StrSeparator for &str {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        s.find(self).map(|pos| SepInfo { pos, size: self.len() })
    }
}

impl StrSeparator for &[char] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for &[char; N] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for [char; N] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl StrSeparator for &[&str] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for &[&str; N] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<const N: usize> StrSeparator for [&str; N] {
    fn str_separate(&self, s: &str) -> Option<SepInfo> {
        self.iter().filter_map(|ss| ss.str_separate(s))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}
