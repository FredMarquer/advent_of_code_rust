use crate::utils::Point;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

    // TODO : improves interface start, end, sizes, min, max -> get, set -> all or D

    pub fn get_range_d(&self, d: usize) -> Range {
        Range::new(self.start[d], self.sizes[d])
    }

    pub fn start(&self) -> Point<D> {
        self.start
    }

    pub fn set_start(&mut self, start: impl Into<Point<D>>) {
        self.start = start.into();
    }

    pub fn set_start_d(&mut self, d: usize, start: i64) {
        self.start[d] = start;
    }

    pub fn end(&self) -> Point<D> {
        self.start + self.sizes
    }

    pub fn set_end(&mut self, end: impl Into<Point<D>>) {
        self.sizes = end.into() - self.start;
        debug_assert!(self.sizes.min_coord() >= 0);
    }

    pub fn set_end_d(&mut self, d: usize, end: i64) {
        self.sizes[d] = end - self.start[d];
        debug_assert!(self.sizes[d] >= 0);
    }

    pub fn sizes(&self) -> Point<D> {
        self.sizes
    }
    
    pub fn set_sizes(&mut self, sizes: impl Into<Point<D>>) {
        self.sizes = sizes.into();
        debug_assert!(self.sizes.min_coord() >= 0);
    }

    pub fn set_size_d(&mut self, d: usize, size: i64) {
        debug_assert!(size >= 0);
        self.sizes[d] = size;
    }

    pub fn min(&self) -> Point<D> {
        self.start
    }

    pub fn set_min(&mut self, min: impl Into<Point<D>>) {
        let end = self.end();
        self.set_start(min);
        self.set_end(end)
    }

    pub fn set_min_d(&mut self, d: usize, min: i64) {
        let end = self.end()[d];
        self.set_start_d(d, min);
        self.set_end_d(d, end)
    }

    pub fn max(&self) -> Point<D> {
        self.end() - 1
    }

    pub fn set_max(&mut self, max: impl Into<Point<D>>) {
        self.set_end(max.into() + 1);
    }

    pub fn set_max_d(&mut self, d: usize, max: i64) {
        self.set_end_d(d, max + 1);
    }

    pub fn volume(&self) -> i64 {
        self.sizes.volume()
    }

    pub fn contains(&self, coords: impl Into<Point<D>>) -> bool {
        let coords: Point<D> = coords.into();
        let end = self.end();
        for d in 0..D {
            if coords[d] < self.start[d] || coords[d] >= end[d] {
                return false;
            }
        }
        true
    }

    pub fn append_point(&mut self, coords: impl Into<Point<D>>) {
        let coords = coords.into();
        for d in 0..D {
            if coords[d] < self.min()[d] {
                self.set_min_d(d, coords[d]);
            } else if coords[d] > self.max()[d] {
                self.set_max_d(d, coords[d]);
            }
        }
    }

    pub fn append_bound(&mut self, other: BoundMD<D>) {
        for d in 0..D {
            if other.min()[d] < self.min()[d] {
                self.set_min_d(d, other.min()[d]);
            } else if other.max()[d] > self.max()[d] {
                self.set_max_d(d, other.max()[d]);
            }
        }
    }

    pub fn overlap(&self, other: &Self) -> OverlapResult {
        // TODO : This can probably be optimized

        if self.start == other.start && self.end() == other.end() {
            return OverlapResult::Equals;
        }

        if self.contains(other.start) && self.contains(other.end() - 1) {
            return OverlapResult::SelfContainsOther;
        }

        if other.contains(self.start) && other.contains(self.end() - 1) {
            return OverlapResult::OtherContainsSelf;
        }

        for d in 0..D {
            if self.start[d] >= other.end()[d] || other.start[d] >= self.end()[d] {
                return OverlapResult::None;
            }
        }

        OverlapResult::Intersect
    }

    pub fn and(&self, other: &Self) -> Option<Self> {
        let min = Point::max(self.start, other.start);
        let max = Point::min(self.end(), other.end()) - 1;
        for d in 0..D {
            if min[d] > max[d] {
                return None;
            }
        }
        Some(Self::from_min_max(min, max))
    }

    pub fn iter_d(&self, d: usize) -> std::ops::Range<i64> {
        self.start[d]..self.end()[d]
    }
}

#[derive(Eq, PartialEq)]
pub enum OverlapResult {
    None,
    Intersect,
    SelfContainsOther,
    OtherContainsSelf,
    Equals,
}

pub type Range = BoundMD<1>;

impl Range {
    pub fn distance(&self) -> i64 {
        self.volume()
    }

    pub fn iter(&self) -> std::ops::Range<i64> {
        self.start[0]..self.end()[0]
    }
}

pub type Bound2D = BoundMD<2>;

impl Bound2D {
    pub fn area(&self) -> i64 {
        self.volume()
    }
}

pub type Bound3D = BoundMD<3>;

impl Bound3D {
    pub fn xy(&self) -> Bound2D {
        Bound2D::new(self.start().xy(), self.sizes().xy())
    }

    pub fn xz(&self) -> Bound2D {
        Bound2D::new(self.start().xz(), self.sizes().xz())
    }

    pub fn yz(&self) -> Bound2D {
        Bound2D::new(self.start().yz(), self.sizes().yz())
    }
}
