
/// The info returned by a separator
/// 
/// `pos`: the position of where the next separator starts
/// 
/// `size`: the size of the next separator
#[derive(Debug)]
pub struct SepInfo {
    pub pos: usize,
    pub size: usize,
}

/// The trait every separator has to implement
pub trait Separator<T> {
    /// A separator will have to say, where the next separator in the slice
    /// is located (and what size it has), by providing a [`SepInfo`]
    fn separate(&self, _: &[T]) -> Option<SepInfo>;
}

impl<T: PartialEq> Separator<T> for T {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        ts.iter().enumerate().find(|(_, t)| *t == self).map(|(pos, _)| SepInfo {
            pos, size: 1,
        })
    }
}

impl<T: PartialEq> Separator<T> for &[T] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        let mut pos = 0;
        let len = ts.len();
        let size = self.len();
        while (pos + size) <= len {
            if ts[pos..pos + size].starts_with(self) {
                return Some(SepInfo { pos, size });
            }
            pos += 1;
        }
        None
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for &[T; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        let mut pos = 0;
        let len = ts.len();
        while (pos + N) <= len {
            if ts[pos..pos + N].starts_with(*self) {
                return Some(SepInfo { pos, size: N });
            }
            pos += 1;
        }
        None
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for [T; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        let mut pos = 0;
        let len = ts.len();
        while (pos + N) <= len {
            if ts[pos..pos + N].starts_with(self) {
                return Some(SepInfo { pos, size: N });
            }
            pos += 1;
        }
        None
    }
}

impl<T: PartialEq> Separator<T> for &[&[T]] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for &[&[T; N]] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for &[[T; N]] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for &[&[T]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize, const M: usize> Separator<T> for &[&[T; M]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize, const M: usize> Separator<T> for &[[T; M]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize> Separator<T> for [&[T]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize, const M: usize> Separator<T> for [&[T; M]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}

impl<T: PartialEq, const N: usize, const M: usize> Separator<T> for [[T; M]; N] {
    fn separate(&self, ts: &[T]) -> Option<SepInfo> {
        self.iter().filter_map(|e| e.separate(ts))
            .min_by_key(|SepInfo { pos, size: _ }| *pos)
    }
}
