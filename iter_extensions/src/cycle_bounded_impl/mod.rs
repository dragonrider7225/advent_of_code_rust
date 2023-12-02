use std::{
    iter::{FusedIterator, TrustedLen},
    ops::Try,
};

/// Like [`cycle()`].[`take()`] except that the number of elements is guaranteed to be exactly
/// `num_cycles` times the length of `base`, even when the length of `base` is not already known.
///
/// [`cycle()`]: std::iter::Iterator.html#method.cycle
/// [`take()`]: std::iter::Iterator.html#method.take
pub fn cycle_bounded<I>(num_cycles: usize, base: I) -> CycleBounded<I>
where
    I: Clone,
{
    let inner = if num_cycles != 0 {
        Some(base.clone())
    } else {
        None
    };
    let back = if num_cycles > 1 {
        Some(base.clone())
    } else {
        None
    };
    CycleBounded {
        num_cycles,
        inner,
        back,
        base,
    }
}

pub struct CycleBounded<I> {
    num_cycles: usize,
    inner: Option<I>,
    back: Option<I>,
    base: I,
}

impl<I> CycleBounded<I> {
    /// A shared reference to the clone of `base` that should be operated on by `Iterator::next`.
    fn front_iter(&self) -> Option<&I> {
        self.inner.as_ref()
    }

    /// A mutable reference to the clone of `base` that should be operated on by `Iterator::next`.
    fn front_iter_mut(&mut self) -> Option<&mut I> {
        self.inner.as_mut()
    }

    /// A shared reference to the clone of `base` that should be operated on by
    /// `DoubleEndedIterator::next_back`.
    fn back_iter(&self) -> Option<&I> {
        match self.num_cycles {
            1 => self.inner.as_ref(),
            _ => self.back.as_ref(),
        }
    }

    /// A mutable reference to the clone of `base` that should be operated on by
    /// `DoubleEndedIterator::next_back`.
    fn back_iter_mut(&mut self) -> Option<&mut I> {
        match self.num_cycles {
            1 => self.inner.as_mut(),
            _ => self.back.as_mut(),
        }
    }
}

impl<I> CycleBounded<I>
where
    I: Clone + Iterator,
{
    /// Discards the current clone of `base` used by `Iterator::next` and replaces it. If the
    /// iterator is already on the last cycle, the replacement will be `None`. If calling this
    /// function will cause the iterator to be on its last cycle, the replacement will be the clone
    /// that is already being used by `DoubleEndedIterator::next_back` (if applicable). Otherwise,
    /// the replacement will be a new clone.
    fn next_cycle(&mut self) {
        match self.num_cycles {
            0 => {}
            1 => {
                self.num_cycles = 0;
                self.inner.take();
            }
            2 => {
                self.num_cycles = 1;
                self.inner = self.back.take();
            }
            _ => {
                self.num_cycles -= 1;
                self.inner = Some(self.base.clone());
            }
        }
    }
}

impl<I> CycleBounded<I>
where
    I: Clone + DoubleEndedIterator,
{
    /// Discards the current clone of `base` used by `DoubleEndedIterator::next_back` and replaces
    /// it. If the iterator is already on the last cycle, the replacement will be `None`. If calling
    /// this function will cause the iterator to be on its last cycle, the replacement will be the
    /// clone that is already being used by `Iterator::next`. Otherwise, the replacement will be a
    /// new clone.
    fn next_cycle_back(&mut self) {
        match self.num_cycles {
            0 => {}
            1 => {
                self.num_cycles = 0;
                self.inner.take();
            }
            2 => {
                self.num_cycles = 1;
                self.inner = self.back.take();
            }
            _ => {
                self.num_cycles -= 1;
                self.back = Some(self.base.clone());
            }
        }
    }
}

impl<I> Clone for CycleBounded<I>
where
    I: Clone,
{
    fn clone(&self) -> Self {
        Self {
            num_cycles: self.num_cycles.clone(),
            inner: self.inner.clone(),
            back: self.back.clone(),
            base: self.base.clone(),
        }
    }
}

impl<I> DoubleEndedIterator for CycleBounded<I>
where
    I: Clone + DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(back_iter) = self.back_iter_mut() {
            back_iter.next_back().or_else(|| {
                self.next_cycle_back();
                self.next_back()
            })
        } else {
            None
        }
    }

    fn advance_back_by(&mut self, n: usize) -> Result<(), usize> {
        let mut res = 0;
        while let Some(back_iter) = self.back_iter_mut() {
            if let Err(skipped) = back_iter.advance_back_by(n - res) {
                self.next_cycle_back();
                res += skipped
            } else {
                res = n;
                break;
            }
        }
        if res == n {
            Ok(())
        } else {
            debug_assert!(res < n);
            debug_assert!(self.back_iter().is_none());
            Err(res)
        }
    }

    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let mut res = init;
        while let Some(back_iter) = self.back_iter_mut() {
            res = back_iter.try_rfold(res, &mut f)?;
            self.next_cycle_back();
        }
        Try::from_output(res)
    }

    fn rfold<B, F>(mut self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.try_rfold(init, |acc, item| Some(f(acc, item)))
            .unwrap()
    }

    fn rfind<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        while let Some(back_iter) = self.back_iter_mut() {
            if let Some(value) = back_iter.rfind(|item| predicate(item)) {
                return Some(value);
            } else {
                self.next_cycle_back();
            }
        }
        None
    }
}

impl<I> ExactSizeIterator for CycleBounded<I> where I: Clone + ExactSizeIterator {}

impl<I> FusedIterator for CycleBounded<I> where I: Clone + Iterator {}

impl<I> Iterator for CycleBounded<I>
where
    I: Clone,
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(front_iter) = self.front_iter_mut() {
            front_iter.next().or_else(|| {
                self.next_cycle();
                self.next()
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.num_cycles {
            0 => (0, Some(0)),
            1 => self.front_iter().unwrap().size_hint(),
            n => {
                let (front_lower, front_upper) = self.front_iter().unwrap().size_hint();
                let (base_lower, base_upper) = self.base.size_hint();
                let (back_lower, back_upper) = self.back_iter().unwrap().size_hint();

                let remaining_lower = base_lower
                    // Skip the front and back cycles, since those may be partially consumed.
                    .checked_mul(n - 2)
                    // Add in the front cycle.
                    .and_then(|lower| lower.checked_add(front_lower))
                    // Add in the back cycle.
                    .and_then(|lower| lower.checked_add(back_lower))
                    // If there are too many items left in the remaining cycles, max out the lower
                    // bound.
                    .unwrap_or(usize::MAX);
                let remaining_upper = base_upper
                    // Skip the front and back cycles, since those may be partially consumed.
                    .and_then(|upper| upper.checked_mul(n - 2))
                    // Add in the front cycle.
                    .and_then(|upper| upper.checked_add(front_upper?))
                    // Add in the back cycle.
                    .and_then(|upper| upper.checked_add(back_upper?));
                (remaining_lower, remaining_upper)
            }
        }
    }

    fn advance_by(&mut self, n: usize) -> Result<(), usize> {
        let mut res = 0;
        while let Some(front_iter) = self.front_iter_mut() {
            if let Err(skipped) = front_iter.advance_by(n - res) {
                res += skipped;
                self.next_cycle();
            } else {
                res = n;
                break;
            }
        }
        if res == n {
            Ok(())
        } else {
            debug_assert!(res < n);
            debug_assert!(self.num_cycles == 0);
            debug_assert!(self.back.is_none());
            Err(res)
        }
    }

    fn try_fold<B, F, R>(&mut self, mut init: B, mut f: F) -> R
    where
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        while let Some(front_iter) = self.front_iter_mut() {
            init = front_iter.try_fold(init, &mut f)?;
            self.next_cycle();
        }
        Try::from_output(init)
    }

    fn fold<B, F>(mut self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        while let Some(front_iter) = self.front_iter_mut() {
            init = front_iter.fold(init, &mut f);
            self.next_cycle();
        }
        init
    }
}

// SAFETY: A `CycleBounded<I>` has an accurate `size_hint` whenever `I` is `TrustedLen`, since
//         `CycleBounded<I>::size_hint` calculates its result exactly from `I::size_hint`.
unsafe impl<I> TrustedLen for CycleBounded<I> where I: Clone + TrustedLen {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_cycles_is_empty() {
        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(0, IntoIterator::into_iter(vals));
        assert_eq!(it.next(), None);

        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(0, IntoIterator::into_iter(vals));
        assert_eq!(it.next_back(), None);
    }

    #[test]
    fn one_cycle_is_transparent() {
        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(1, IntoIterator::into_iter(vals));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);

        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(1, IntoIterator::into_iter(vals));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(it.next_back(), Some(3));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.next_back(), None);
    }

    #[test]
    fn multiple_cycles_work() {
        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(2, IntoIterator::into_iter(vals));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);

        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(2, IntoIterator::into_iter(vals));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(it.next_back(), Some(3));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(it.next_back(), Some(3));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.next_back(), None);
    }

    #[test]
    fn advance_by_works_between_cycles() {
        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(2, IntoIterator::into_iter(vals));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.advance_by(2), Ok(()));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);

        let vals = [1, 2, 3, 4];
        let mut it = cycle_bounded(2, IntoIterator::into_iter(vals));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(it.next_back(), Some(3));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.advance_back_by(2), Ok(()));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.next_back(), None);
    }
}
