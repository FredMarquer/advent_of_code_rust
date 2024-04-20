use crate::utils::Point;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BoundMD<const D: usize> {
    start: Point<D>,
    sizes: Point<D>,
}

impl<const D: usize> BoundMD<D> {
    pub fn new(start: impl Into<Point<D>>, sizes: impl Into<Point<D>>) -> Self {
        let start = start.into();
        let sizes = sizes.into();
        debug_assert!(sizes.min_coord() >= 0);

        BoundMD {
            start,
            sizes,
        }
    }

    pub fn from_min_max(min: impl Into<Point<D>>, max: impl Into<Point<D>>) -> Self {
        let start = min.into();
        let max = max.into();
        let sizes = (max - start) + 1;
        debug_assert!(sizes.min_coord() >= 0);

        BoundMD {
            start,
            sizes,
        }
    }

    pub fn start(&self) -> Point<D> {
        self.start
    }

    pub fn start_mut(&mut self) -> &mut Point<D> {
        &mut self.start
    }

    pub fn sizes(&self) -> Point<D> {
        self.sizes
    }

    pub fn set_sizes(&mut self, sizes: impl Into<Point<D>>) {
        self.sizes = sizes.into();
        debug_assert!(self.sizes.min_coord() >= 0);
    }

    pub fn size(&self, d: usize) -> i64 {
        self.sizes[d]
    }

    pub fn end(&self) -> Point<D> {
        self.start + self.sizes
    }

    pub fn set_end(&mut self, end: impl Into<Point<D>>) {
        self.sizes = end.into() - self.start;
        debug_assert!(self.sizes.min_coord() >= 0);
    }

    pub fn volume(&self) -> i64 {
        self.sizes.volume()
    }

    pub fn is_in_bound(&self, coords: impl Into<Point<D>>) -> bool {
        let coords: Point<D> = coords.into();
        let end = self.end();
        for d in 0..D {
            if coords[d] < self.start[d] || coords[d] >= end[d] {
                return false;
            }
        }
        true
    }

    pub fn overlap(&self, other: &Self) -> OverlapResult {
        // TODO : This can probably be optimized

        if self.is_in_bound(other.start) && self.is_in_bound(other.end() - 1) {
            return OverlapResult::SelfContainsOther;
        }

        if other.is_in_bound(self.start) && other.is_in_bound(self.end() - 1) {
            return OverlapResult::OtherContainsSelf;
        }

        for d in 0..D {
            if self.start[d] >= other.end()[d] || other.start[d] >= self.end()[d] {
                return OverlapResult::None;
            }
        }

        OverlapResult::Intersect
    }
}

#[derive(Eq, PartialEq)]
pub enum OverlapResult {
    None,
    Intersect,
    SelfContainsOther,
    OtherContainsSelf,
}

pub type Range = BoundMD<1>;

impl Range {
    pub fn distance(&self) -> i64 {
        self.volume()
    }
}

pub type Bound2D = BoundMD<2>;

impl Bound2D {
    pub fn area(&self) -> i64 {
        self.volume()
    }
}

pub type Bound3D = BoundMD<3>;
