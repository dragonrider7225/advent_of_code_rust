use nom::{bytes::complete as bytes, combinator as comb, sequence, IResult};

use std::{cmp::Ordering, io, iter::FromIterator};

use crate::parse::NomParse;

trait Semigroup {
    /// An associative operation.
    fn op(self, other: Self) -> Self;
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn op(self, other: Option<T>) -> Option<T> {
        match (self, other) {
            (Some(x), Some(y)) => Some(x.op(y)),
            (Some(x), _) => Some(x),
            (_, Some(y)) => Some(y),
            _ => None,
        }
    }
}

impl Semigroup for Ordering {
    fn op(self, other: Ordering) -> Ordering {
        self.then(other)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point(x, y)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((base_x, base_y): (T, T)) -> Point<T> {
        Point::new(base_x, base_y)
    }
}

impl<T: PartialOrd> PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Point<T>) -> Option<Ordering> {
        self.1
            .partial_cmp(&other.1)
            .op(self.0.partial_cmp(&other.0))
    }
}

impl<T: Ord> Ord for Point<T> {
    fn cmp(&self, other: &Point<T>) -> Ordering {
        self.1.cmp(&other.1).op(self.0.cmp(&other.0))
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Rect {
    id: u32,
    left: u32,
    bottom: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn with_id(id: u32, left: u32, bottom: u32, width: u32, height: u32) -> Rect {
        Rect {
            id,
            left,
            bottom,
            width,
            height,
        }
    }

    fn new(left: u32, bottom: u32, width: u32, height: u32) -> Rect {
        Rect::with_id(0, left, bottom, width, height)
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn lower_left(&self) -> Point<u32> {
        Point::new(self.left(), self.bottom())
    }

    fn left(&self) -> u32 {
        self.left
    }

    fn bottom(&self) -> u32 {
        self.bottom
    }

    fn right(&self) -> u32 {
        self.left() + self.width()
    }

    fn top(&self) -> u32 {
        self.bottom() + self.height()
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn area(&self) -> u32 {
        self.width() * self.height()
    }

    /// Compute the intersection of `self` and `other`. If `self` and `other` do
    /// not intersect, return `None`.
    fn intersect(&self, other: &Rect) -> Option<Rect> {
        if self.top() <= other.bottom()
            || other.top() <= self.bottom()
            || self.right() <= other.left()
            || other.right() <= self.left()
        {
            None
        } else {
            let new_left = u32::max(self.left(), other.left());
            let new_bottom = u32::max(self.bottom(), other.bottom());
            let new_width = u32::min(self.right(), other.right()) - new_left;
            let new_height = u32::min(self.top(), other.top()) - new_bottom;
            Some(Rect::new(new_left, new_bottom, new_width, new_height))
        }
    }

    fn intersect_set(&self, others: &RectSet) -> RectSet {
        others
            .clone()
            .into_iter()
            .filter_map(|r| r.intersect(self))
            .collect()
    }

    /**
     * Produce a `RectSet` that represents all portions of the rectangle
     * represented by `self` except those contained in the rectangle represented
     * by `other`.
     */
    fn except(&self, other: &Rect) -> RectSet {
        if self.top() <= other.bottom()
            || other.top() <= self.bottom()
            || self.right() <= other.left()
            || other.right() <= self.left()
        {
            RectSet::just(self.clone())
        } else {
            // General shape of ret:
            // +----------+
            // |   s      |
            // +-+---+----+
            // | | o |    |
            // +-+---+----+
            // +----------+
            // If `other` extends above the top of `self`, then `pillar_top` will
            // be equal to `self.top()` and `top_bar` will represent a degenerate
            // rectangle with zero height. Similarly, if `other` extends beyond
            // either side of `self`, then the pillar for the side in question
            // will be degenerate with zero width.
            let mut ret = RectSet::new();
            // If `other` extends above `self`, the top bar is nonexistent.
            let pillar_top = u32::min(self.top(), other.top());
            let top_bar = Rect::new(
                self.left(),
                other.top(),
                self.width(),
                self.top() - pillar_top,
            );
            let pillar_bottom = u32::max(self.bottom(), other.bottom());
            let pillar_height = pillar_top - pillar_bottom;
            let right_pillar = Rect::new(
                other.right(),
                pillar_bottom,
                self.right() - u32::min(self.right(), other.right()),
                pillar_height,
            );
            let left_pillar = Rect::new(
                self.left(),
                pillar_bottom,
                u32::max(self.left(), other.left()) - self.left(),
                pillar_height,
            );
            let bottom_bar = Rect::new(
                self.left(),
                self.bottom(),
                self.width(),
                pillar_bottom - self.bottom(),
            );
            ret.add(top_bar);
            ret.add(right_pillar);
            ret.add(left_pillar);
            ret.add(bottom_bar);
            ret
        }
    }

    /**
     * Produce a `RectSet` that represents all portions of the rectangle
     * represented by `self` except those contained in one or more of the
     * rectangles represented by the elements of `other`. The result is *not*
     * guaranteed to be the smallest possible set of `Rect`s that cover the area
     * of `self` not covered by `other`.
     */
    fn except_set(&self, other: &RectSet) -> RectSet {
        // Guaranteed to always be "flat", since `Rect::except` always produces a
        // flat `RectSet`.
        let mut ret = RectSet::just(self.clone());
        for rect in other.clone() {
            ret = ret.into_iter().flat_map(|r| r.except(&rect)).collect();
        }
        ret
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Rect) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rect {
    fn cmp(&self, other: &Rect) -> Ordering {
        self.lower_left()
            .cmp(&other.lower_left())
            .op(self.height().cmp(&other.height()))
            .op(self.width().cmp(&other.width()))
    }
}

impl<'s> NomParse<'s> for Rect {
    fn nom_parse(s: &str) -> IResult<&str, Rect> {
        comb::map(
            // Parse ((id, (left, bottom)), (width, height)) ("#{} @ {},{}: {}x{}")
            sequence::pair(
                // Parse (id, (left, bottom)) ("#{} @ {},{}")
                sequence::pair(
                    // Parse id ("#{} @ ")
                    sequence::terminated(
                        // Parse id ("#{}")
                        sequence::preceded(bytes::tag("#"), u32::nom_parse),
                        bytes::tag(" @ "),
                    ),
                    // Parse (left, bottom) ("{},{}")
                    sequence::separated_pair(u32::nom_parse, bytes::tag(","), u32::nom_parse),
                ),
                // Parse (width, height) (": {}x{}")
                sequence::preceded(
                    bytes::tag(": "),
                    // Parse (width, height) ("{}x{}")
                    sequence::separated_pair(u32::nom_parse, bytes::tag("x"), u32::nom_parse),
                ),
            ),
            |((id, (left, bottom)), (width, height))| {
                Rect::with_id(id, left, bottom, width, height)
            },
        )(s)
    }
}

impl_from_str_for_nom_parse!(Rect);

// Invariants: contents of wrapped `Vec` monotonically increase.
#[derive(Clone)]
struct RectSet(Vec<Rect>);

impl RectSet {
    /// Create a new empty RectSet.
    fn new() -> RectSet {
        // Invariant upheld because wrapped `Vec` is empty.
        RectSet(Vec::new())
    }

    /// Create a new RectSet from a single Rect.
    fn just(r: Rect) -> RectSet {
        let ret = vec![r];
        // Invariant upheld because any sequence of a single element is
        // guaranteed to be monotonically increasing.
        RectSet(ret)
    }

    /// Add a Rect to an existing RectSet.
    fn add(&mut self, r: Rect) {
        if r.width() != 0 && r.height() != 0 {
            for i in 0..self.0.len() {
                if r <= self.0[i] {
                    self.0.insert(i, r);
                    // Invariant upheld because the new `Rect` is inserted
                    // immediately before the first `Rect` that is not less than
                    // it.
                    return;
                }
            }
            self.0.push(r);
        }
        // Invariant upheld because the for-loop only completes normally if all
        // `Rect`s already present are less than the new `Rect`.
    }

    fn intersect(&self, other: &RectSet) -> RectSet {
        self.clone()
            .into_iter()
            .flat_map(|r| r.intersect_set(other))
            .collect()
    }

    /// Compute the union of `self` and `other` in place.
    fn union_mut(&mut self, other: &RectSet) {
        if other.0.is_empty() || self.0.is_empty() {
            self.0.extend(other.0.clone().into_iter());
            // Invariant upheld because `self` now contains exactly the same
            // `Rect`s in exactly the same order as `other`.
            return;
        }
        let mut other_iter = other.0.clone().into_iter();
        let mut other_next = other_iter.next();
        let mut i = 0;
        while i < self.0.len() {
            if let Some(r) = other_next {
                if r <= self.0[i] {
                    self.0.insert(i, r);
                    other_next = other_iter.next();
                } else {
                    i += 1;
                    other_next = Some(r);
                }
            } else {
                // Invariant upheld because all `Rect`s from `other` have
                // been inserted immediately before the `Rect` that was in
                // `self` and not less than the inserted `Rect`.
                return;
            }
        }
        // All remaining Rects are greater than or equal to the last Rect in
        // self. If any Rects remain, push them to the end of the wrapped Vec.
        if let Some(other_next) = other_next {
            self.0.push(other_next);
            self.0.extend(other_iter);
        }
        // Invariant upheld because all `Rect`s from `other` are inserted
        // either immediately before the first `Rect` that was already in `self`
        // but not less than the inserted `Rect` or at the end of `self` in
        // ascending order if no such subsequent `Rect` exists.
    }

    /// Compute the RectSet which represents all area covered by at least two
    /// Rects in this RectSet.
    fn overlap(self) -> RectSet {
        let mut seen = RectSet::new();
        let mut ret = RectSet::new();
        for rect in self {
            let just_rect = RectSet::just(rect);
            let intersection = seen.intersect(&just_rect);
            if intersection.area() > 0 {
                ret.union_mut(&intersection);
            }
            seen.union_mut(&just_rect);
        }
        // Invariant upheld because all additions to ret are made using
        // `RectSet::union_mut`, which upholds the invariant.
        ret
    }

    fn non_overlap_ids(self) -> Vec<u32> {
        let overlap_set = self.clone().overlap();
        let mut ret = Vec::new();
        for rect in self {
            if rect.intersect_set(&overlap_set).len() == 0 {
                ret.push(rect.id());
            }
        }
        ret
    }

    /**
     * Compute the total area covered by this `RectSet`. If two or more
     * rectangles cover a particular area simultaneously, count that area only
     * once.
     */
    fn area(&self) -> u32 {
        self.flatten().into_iter().map(|r| r.area()).sum()
    }

    /// Compute a RectSet with no overlap which covers the same area as this
    /// RectSet.
    fn flatten(&self) -> RectSet {
        let mut ret = RectSet::new();
        for rect in self.clone() {
            ret.union_mut(&rect.except_set(&ret));
        }
        // Invariant upheld because all additions to `ret` are made by
        // `Rect::union_mut`, which upholds the invariant
        ret
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromIterator<Rect> for RectSet {
    fn from_iter<T>(iter: T) -> RectSet
    where
        T: IntoIterator<Item = Rect>,
    {
        let mut ret = RectSet::new();
        for item in iter {
            ret.add(item);
        }
        ret
    }
}

impl IntoIterator for RectSet {
    type Item = Rect;
    type IntoIter = <Vec<Rect> as IntoIterator>::IntoIter;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.0.into_iter()
    }
}

pub fn run() -> io::Result<()> {
    fn get_claims() -> io::Result<RectSet> {
        Ok(super::super::parse_lines("3.txt")?.collect())
    }
    // Part 1
    println!("Overlap area: {}", get_claims()?.overlap().area());
    // Part 2
    println!(
        "Non-overlapping claim: {:?}",
        get_claims()?.non_overlap_ids()
    );
    Ok(())
}
