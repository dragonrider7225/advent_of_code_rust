/// A priority queue is a list of values which can provide the value with the highest priority in
/// constant time.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PriorityQueue<P, T> {
    values: Vec<(P, T)>,
}

impl<P, T> PriorityQueue<P, T> {
    /// Creates an empty PriorityQueue.
    pub const fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Returns true if and only if [`self.peek()`]`.is_none()`.
    ///
    /// [`self.peek()`]: #method.peek
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the number of values in the queue.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns a reference to the element that would be returned by [`self.pop()`].
    ///
    /// [`self.pop()`]: #method.pop
    pub fn peek(&self) -> Option<&T> {
        self.values.first().map(|(_, value)| value)
    }
}

impl<P, T> PriorityQueue<P, T>
where
    P: Ord,
{
    /// Returns the element in the queue with the greatest priority.
    pub fn pop(&mut self) -> Option<T> {
        self.remove(0).map(|(_, value)| value)
    }

    /// Inserts `value` into the queue with priority `priority`.
    pub fn insert(&mut self, value: T, priority: P) {
        let mut idx = self.len();
        self.values.push((priority, value));
        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            if self.values[parent_idx].0 < self.values[idx].0 {
                self.values.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    /// Like [`insert()`] except that the priority is `priority_fn(&value)` instead of being passed
    /// in directly.
    ///
    /// [`insert()`]: #method.insert
    pub fn insert_with_fn<F>(&mut self, value: T, priority_fn: F)
    where
        F: FnOnce(&T) -> P,
    {
        let priority = priority_fn(&value);
        self.insert(value, priority)
    }

    fn remove(&mut self, mut idx: usize) -> Option<(P, T)> {
        if self.is_empty() {
            return None;
        }
        while idx > 0 {
            let parent_idx = (idx - 1) / 2;
            self.values.swap(idx, parent_idx);
            idx = parent_idx;
        }
        let ret = Some(self.values.swap_remove(0));
        loop {
            let left_idx = 2 * idx + 1;
            let right_idx = left_idx + 1;
            let left = self.values.get(left_idx);
            let right = self.values.get(right_idx);
            match (left, right) {
                (Some((left_priority, _)), Some((right_priority, _)))
                    if left_priority > right_priority =>
                {
                    self.values.swap(idx, left_idx);
                    idx = left_idx;
                    continue;
                }
                (Some(_), None) => {
                    self.values.swap(idx, left_idx);
                    idx = left_idx;
                    continue;
                }
                (Some(_), Some(_)) => {
                    self.values.swap(idx, right_idx);
                    idx = right_idx;
                    continue;
                }
                (None, Some(_)) => unreachable!(),
                (None, None) => break,
            }
        }
        ret
    }
}

impl<P, T> PriorityQueue<P, T>
where
    P: Ord,
    T: Eq,
{
    /// Selects and removes an arbitrary value in the queue equal to `value` if such a value exists
    /// then inserts `value` with priority `priority`. If a value was removed, it and its priority
    /// are returned.
    pub fn replace(&mut self, value: T, priority: P) -> Option<(P, T)> {
        self.replace_by(value, priority, PartialEq::eq)
    }

    /// Like [`replace()`] except that the priority is `priority_fn(&value)` instead of being
    /// passed in directly.
    ///
    /// [`replace()`]: #method.replace
    pub fn replace_with_fn<F>(&mut self, value: T, priority_fn: F) -> Option<(P, T)>
    where
        F: FnOnce(&T) -> P,
    {
        let priority = priority_fn(&value);
        self.replace(value, priority)
    }

    /// Like [`replace()`] except that comparison between values is done by `eq` instead of
    /// [`PartialEq::eq`].
    ///
    /// [`replace()`]: #method.replace
    pub fn replace_by<E>(&mut self, value: T, priority: P, mut eq: E) -> Option<(P, T)>
    where
        E: FnMut(&T, &T) -> bool,
    {
        for idx in 0..self.len() {
            if eq(&self.values[0].1, &value) {
                let ret = self.remove(idx);
                self.insert(value, priority);
                return ret;
            }
        }
        self.insert(value, priority);
        None
    }

    /// Like [`replace()`] except that the priority is `priority_fn(&value)` instead of being
    /// passed in directly and comparison between values is done by `eq` instead of
    /// [`PartialEq::eq`].
    ///
    /// [`replace()`]: #method.replace
    pub fn replace_with_fn_by<F, E>(&mut self, value: T, priority_fn: F, eq: E) -> Option<(P, T)>
    where
        F: FnOnce(&T) -> P,
        E: FnMut(&T, &T) -> bool,
    {
        let priority = priority_fn(&value);
        self.replace_by(value, priority, eq)
    }
}

impl<T, P> Default for PriorityQueue<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut queue = PriorityQueue::new();
        for (i, j) in (0..5).rev().zip(5..10) {
            queue.insert(i, i);
            queue.insert(j, j);
        }
        let expected = PriorityQueue {
            values: Vec::from_iter([9, 8, 6, 5, 7, 3, 1, 4, 0, 2].into_iter().map(|x| (x, x))),
        };
        assert_eq!(queue, expected);
    }

    #[test]
    fn test_pop() {
        let mut queue = PriorityQueue {
            values: Vec::from_iter([9, 8, 6, 5, 7, 3, 1, 4, 0, 2].into_iter().map(|x| (x, x))),
        };
        for i in (0..10).rev() {
            assert_eq!(queue.pop(), Some(i));
        }
        assert_eq!(queue.pop(), None);
    }
}
