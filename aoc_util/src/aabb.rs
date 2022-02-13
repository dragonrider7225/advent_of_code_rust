/// An axis-aligned bounding box. Includes all points `(x, y, z)` such that
/// `(self.min_x..=self.max_x).contains(&x) && (self.min_y..=self.max_y).contains(&y) &&
/// (self.min_z..=self.max_z).contains(&z)` holds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Aabb {
    /// The minimum x-coordinate among all points in this box.
    pub min_x: i64,
    /// The maximum x-coordinate among all points in this box.
    pub max_x: i64,
    /// The minimum y-coordinate among all points in this box.
    pub min_y: i64,
    /// The maximum y-coordinate among all points in this box.
    pub max_y: i64,
    /// The minimum z-coordinate among all points in this box.
    pub min_z: i64,
    /// The maximum z-coordinate among all points in this box.
    pub max_z: i64,
}

impl Aabb {
    /// Checks whether the box contains no points. Since the range of included points in inclusive,
    /// this only occurs when the maximum along an axis is strictly less than the minimum along
    /// that axis.
    pub fn is_empty(&self) -> bool {
        self.max_x < self.min_x || self.max_y < self.min_y || self.max_z < self.min_z
    }

    /// Gets the number of points in the box.
    pub fn size(&self) -> u64 {
        if self.is_empty() {
            0
        } else {
            let x_width = (self.max_x - self.min_x + 1) as u64;
            let y_width = (self.max_y - self.min_y + 1) as u64;
            let z_width = (self.max_z - self.min_z + 1) as u64;
            x_width * y_width * z_width
        }
    }
}

impl Aabb {
    /// Checks whether there is some point which is in both `self` and `rhs`.
    pub fn intersects(&self, rhs: &Self) -> bool {
        ((rhs.min_x..=rhs.max_x).contains(&self.min_x)
            || (rhs.min_x..=rhs.max_x).contains(&self.max_x)
            || (self.min_x..=self.max_x).contains(&rhs.min_x)
            || (self.min_x..=self.max_x).contains(&rhs.max_x))
            && ((rhs.min_y..=rhs.max_y).contains(&self.min_y)
                || (rhs.min_y..=rhs.max_y).contains(&self.max_y)
                || (self.min_y..=self.max_y).contains(&rhs.min_y)
                || (self.min_y..=self.max_y).contains(&rhs.max_y))
            && ((rhs.min_z..=rhs.max_z).contains(&self.min_z)
                || (rhs.min_z..=rhs.max_z).contains(&self.max_z)
                || (self.min_z..=self.max_z).contains(&rhs.min_z)
                || (self.min_z..=self.max_z).contains(&rhs.max_z))
    }

    /// Creates an [`AabbSet`] which contains all and only those points which are in `self` but not
    /// in `rhs`. No guarantees are made about how those points are collected into boxes.
    pub fn except(&self, rhs: &Self) -> AabbSet {
        if !self.intersects(rhs) {
            #[cfg(test)]
            println!("{self:?} and {rhs:?} do not intersect");
            return AabbSet {
                inner: AabbSetInner::Singleton(*self),
            };
        }
        let modified_min_x = rhs.min_x.max(self.min_x);
        let modified_max_x = rhs.max_x.min(self.max_x);
        let modified_min_y = rhs.min_y.max(self.min_y);
        let modified_max_y = rhs.max_y.min(self.max_y);
        #[cfg(test)]
        dbg!(
            modified_min_x,
            modified_max_x,
            modified_min_y,
            modified_max_y
        );
        [
            // (-, *, *)
            Self {
                min_x: self.min_x,
                max_x: rhs.min_x - 1,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            },
            // (+, *, *)
            Self {
                min_x: rhs.max_x + 1,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            },
            // (0, -, *)
            Self {
                min_x: modified_min_x,
                max_x: modified_max_x,
                min_y: self.min_y,
                max_y: rhs.min_y - 1,
                min_z: self.min_z,
                max_z: self.max_z,
            },
            // (0, +, *)
            Self {
                min_x: modified_min_x,
                max_x: modified_max_x,
                min_y: rhs.max_y + 1,
                max_y: self.max_y,
                min_z: self.min_z,
                max_z: self.max_z,
            },
            // (0, 0, -)
            Self {
                min_x: modified_min_x,
                max_x: modified_max_x,
                min_y: modified_min_y,
                max_y: modified_max_y,
                min_z: self.min_z,
                max_z: rhs.min_z - 1,
            },
            // (0, 0, +)
            Self {
                min_x: modified_min_x,
                max_x: modified_max_x,
                min_y: modified_min_y,
                max_y: modified_max_y,
                min_z: rhs.max_z + 1,
                max_z: self.max_z,
            },
        ]
        .into_iter()
        .filter(|piece| {
            #[allow(clippy::let_and_return)]
            let ret = !piece.is_empty();
            #[cfg(test)]
            if ret {
                println!("Including piece {piece:?}");
            } else {
                println!("Removing empty piece {piece:?}");
            }
            ret
        })
        .collect()
    }
}

#[derive(Clone, Debug)]
enum AabbSetInner {
    Empty,
    Singleton(Aabb),
    Multi { pieces: Vec<Aabb> },
}

impl AabbSetInner {
    fn size(&self) -> u64 {
        match self {
            Self::Empty => 0,
            Self::Singleton(singleton) => singleton.size(),
            Self::Multi { pieces } => pieces.iter().map(Aabb::size).sum(),
        }
    }
}

impl AabbSetInner {
    fn insert(&mut self, aabb: Aabb) {
        match self {
            Self::Empty => *self = Self::Singleton(aabb),
            Self::Singleton(current) => match aabb.except(&*current).inner {
                Self::Empty => {}
                Self::Singleton(additional) => {
                    *self = Self::Multi {
                        pieces: vec![*current, additional],
                    }
                }
                Self::Multi { mut pieces } => {
                    pieces.push(*current);
                    *self = Self::Multi { pieces };
                }
            },
            Self::Multi { pieces } => {
                let mut new_pieces = vec![aabb];
                for piece in pieces.iter() {
                    let local_pieces = new_pieces
                        .drain(..)
                        .flat_map(|new_piece| {
                            // `new_piece`s are disjoint because they are all fragments of `aabb`
                            // from `Aabb::except`.
                            if !new_piece.intersects(piece) {
                                Self::Singleton(new_piece)
                            } else {
                                new_piece.except(piece).inner
                            }
                        })
                        .collect::<Vec<_>>();
                    new_pieces.extend(local_pieces);
                }
                pieces.extend(new_pieces);
            }
        }
    }

    fn remove(&mut self, aabb: Aabb) {
        match self {
            Self::Empty => {}
            Self::Singleton(singleton) => {
                *self = singleton.except(&aabb).inner;
            }
            Self::Multi { pieces } => {
                let fragments = pieces
                    .drain(..)
                    .flat_map(|piece| piece.except(&aabb))
                    .collect::<Vec<_>>();
                pieces.extend(fragments);
                match pieces.len() {
                    0 => *self = Self::Empty,
                    1 => {
                        let singleton = pieces.pop().unwrap();
                        *self = Self::Singleton(singleton);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Default for AabbSetInner {
    fn default() -> Self {
        Self::Empty
    }
}

impl IntoIterator for AabbSetInner {
    type Item = Aabb;
    type IntoIter = AabbSetIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Empty => Self::IntoIter::empty(),
            Self::Singleton(singleton) => Self::IntoIter::from_singleton(singleton),
            Self::Multi { pieces } => Self::IntoIter::from_pieces(pieces),
        }
    }
}

/// A collection of disjoint bounding boxes.
#[derive(Clone, Debug, Default)]
pub struct AabbSet {
    inner: AabbSetInner,
}

impl AabbSet {
    /// Gets the total number of points in all of the contained boxes.
    pub fn size(&self) -> u64 {
        self.inner.size()
    }
}

impl AabbSet {
    /// Adds the points contained in `aabb` to this set.
    pub fn insert(&mut self, aabb: Aabb) {
        self.inner.insert(aabb)
    }

    /// Removes the points contained in `aabb` from this set.
    pub fn remove(&mut self, aabb: Aabb) {
        self.inner.remove(aabb)
    }
}

impl FromIterator<Aabb> for AabbSet {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Aabb>,
    {
        iter.into_iter().fold(Self::default(), |mut acc, piece| {
            acc.insert(piece);
            acc
        })
    }
}

impl IntoIterator for AabbSet {
    type Item = Aabb;
    type IntoIter = AabbSetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

/// An iterator over the boxes in an [`AabbSet`].
#[derive(Clone, Debug)]
pub struct AabbSetIter {
    remaining: Vec<Aabb>,
    next: Option<Aabb>,
}

impl AabbSetIter {
    fn empty() -> Self {
        Self {
            remaining: vec![],
            next: None,
        }
    }

    fn from_singleton(singleton: Aabb) -> Self {
        Self {
            remaining: vec![],
            next: Some(singleton),
        }
    }

    fn from_pieces(mut pieces: Vec<Aabb>) -> Self {
        pieces.reverse();
        let next = pieces.pop();
        Self {
            remaining: pieces,
            next,
        }
    }
}

impl Iterator for AabbSetIter {
    type Item = Aabb;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            Some(next) => {
                self.next = self.remaining.pop();
                Some(next)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_set_insert() {
        let mut set = AabbSet {
            inner: AabbSetInner::Empty,
        };
        set.insert(Aabb {
            min_x: -20,
            max_x: 26,
            min_y: -36,
            max_y: 17,
            min_z: -47,
            max_z: 7,
        });
        assert_eq!(139_590, set.size());
        // The portion of this Aabb which does not intersect with the extant volume is the union of
        // { x: 27..=33, y: -21..=23, z: -26..=28 }, { x: -20..=26, y: 18..=23, z: -26..=28 }, and
        // { x: -20..=26, y: -21..=17, z: 8..=28 }.
        set.insert(Aabb {
            min_x: -20,
            max_x: 33,
            min_y: -21,
            max_y: 23,
            min_z: -26,
            max_z: 28,
        });
        assert_eq!(139_590 + 17_325 + 15_510 + 38_493, set.size());
    }

    #[test]
    fn test_aabb_except_inner() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(152, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_x_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 5,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_x_greater() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 6,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_x_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 0,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_x_less() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: -1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_y_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 5,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_y_greater() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 6,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_y_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 0,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_y_less() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: -1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_z_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 5,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_max_z_greater() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 6,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_z_equal() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: 0,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_min_z_less() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: -1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(136, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_x_outer() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: -1,
            max_x: 6,
            min_y: 1,
            max_y: 4,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(120, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_y_outer() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: -1,
            max_y: 6,
            min_z: 1,
            max_z: 4,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(120, difference.size());
    }

    #[test]
    fn test_aabb_except_inner_z_outer() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: 1,
            max_x: 4,
            min_y: 1,
            max_y: 4,
            min_z: -1,
            max_z: 6,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        assert_eq!(120, difference.size());
    }

    #[test]
    fn test_aabb_except_outer() {
        let box1 = Aabb {
            min_x: 0,
            max_x: 5,
            min_y: 0,
            max_y: 5,
            min_z: 0,
            max_z: 5,
        };
        let box2 = Aabb {
            min_x: -1,
            max_x: 6,
            min_y: -1,
            max_y: 6,
            min_z: -1,
            max_z: 6,
        };
        let difference = box1.except(&box2);
        dbg!(&difference);
        match difference.inner {
            AabbSetInner::Empty => {}
            AabbSetInner::Singleton(_) => unreachable!("Got singleton for empty set"),
            AabbSetInner::Multi { .. } => unreachable!("Got multi-part empty set"),
        }
        assert_eq!(0, difference.size());
    }
}
