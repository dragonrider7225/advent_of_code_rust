use std::iter::{FusedIterator, TrustedLen};

pub fn replicate<T>(count: usize, value: T) -> Replicate<T> {
    Replicate {
        value: match count {
            0 => None,
            _ => Some(value),
        },
        count,
    }
}

pub struct Replicate<T> {
    value: Option<T>,
    count: usize,
}

/// A `Replicate<T>` is its own reverse, so `next_back` just delegates to `next`.
impl<T> DoubleEndedIterator for Replicate<T>
where
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<T> ExactSizeIterator for Replicate<T> where T: Clone {}

impl<T> FusedIterator for Replicate<T> where T: Clone {}

impl<T> Iterator for Replicate<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.count {
            0 => None,
            1 => self.value.take(),
            _ => self.value.clone(),
        };
        self.count = self.count.saturating_sub(1);
        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

// SAFETY: This implementation is safe because the size of the iterator is always `self.count`.
unsafe impl<T> TrustedLen for Replicate<T> where T: Clone {}
