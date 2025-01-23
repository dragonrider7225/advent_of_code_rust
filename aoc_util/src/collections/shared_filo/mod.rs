use std::{
    fmt::{self, Debug, Formatter},
    rc::Rc,
};

/// A FILO queue that shares as much of the bottom of its stack as possible with other queues.
#[derive(Eq, PartialEq)]
pub struct SharedFilo<T> {
    inner: Tail<T>,
    len: usize,
}

impl<T> SharedFilo<T> {
    /// Creates an empty queue.
    pub const fn new() -> Self {
        Self {
            inner: None,
            len: 0,
        }
    }

    /// Gets the number of elements in the queue.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Checks whether the queue contains no elements.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Puts an element at the head of the queue.
    #[must_use = "SharedFilo is immutable"]
    pub fn push(&self, value: T) -> Self {
        Self {
            inner: Some(Rc::new(Node {
                head: value,
                tail: self.inner.clone(),
            })),
            len: self.len + 1,
        }
    }

    /// Removes the element from the head of the queue.
    #[must_use = "SharedFilo is immutable"]
    pub fn pop(self) -> Option<(T, Self)>
    where
        T: Clone,
    {
        let Self {
            inner: Some(inner),
            len,
        } = self
        else {
            return None;
        };
        let head = inner.head.clone();
        let tail = inner.tail.clone();
        Some((
            head,
            Self {
                inner: tail,
                len: len - 1,
            },
        ))
    }
}

impl<T> Clone for SharedFilo<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            len: self.len,
        }
    }
}

impl<T> Debug for SharedFilo<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();
        let mut tail = &self.inner;
        while let Some(remaining) = tail {
            debug_list.entry(&remaining.head);
            tail = &remaining.tail;
        }
        debug_list.finish()
    }
}

impl<T> Default for SharedFilo<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for SharedFilo<T>
where
    T: Clone,
{
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { back: self }
    }
}

/// An iterator over the
#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    back: SharedFilo<T>,
}

impl<T> Iterator for IntoIter<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (ret, tail) = self.back.clone().pop()?;
        self.back = tail;
        Some(ret)
    }
}

type Tail<T> = Option<Rc<Node<T>>>;

#[derive(Clone, Eq, PartialEq)]
struct Node<T> {
    head: T,
    tail: Tail<T>,
}
