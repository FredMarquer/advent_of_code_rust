use std::ops::{Index, IndexMut};

use crate::utils::{ArrayMD, BoundMD, OverlapResult, Point};

pub struct SliceMD<'a, const D: usize, T> {
    array: &'a ArrayMD<D, T>,
    bound: BoundMD<D>,
}

impl<'a, const D: usize, T> SliceMD<'a, D, T> {
    pub fn new(array: &'a ArrayMD<D, T>, bound: impl Into<BoundMD<D>>) -> Self {
        let bound = bound.into();
        debug_assert!(bound.overlap(&BoundMD::new(Point::ZERO, array.sizes())) == OverlapResult::OtherContainsSelf);
        SliceMD {
            array,
            bound,
        }
    }

    pub fn sizes(&self) -> Point<D> {
        self.bound.sizes()
    }

    pub fn size(&self, d: usize) -> i64 {
        self.bound.sizes()[d]
    }

    pub fn is_in_bound(&self, coords: impl Into<Point<D>>) -> bool {
        let coords: Point<D> = coords.into();
        for d in 0..D {
            if coords[d] < 0 || coords[d] >= self.bound.sizes()[d] {
                return false;
            }
        }
        true
    }

    pub fn get(&self, coords: impl Into<Point<D>>) -> &T {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            panic!("coords (= {:?}) out of bound (= {:?})", coords, self.bound.sizes());
        }

        self.array.get(coords + self.bound.start())
    }

    pub fn try_get(&self, coords: impl Into<Point<D>>) -> Option<&T> {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            return None;
        }

        self.array.try_get(coords + self.bound.start())
    }

    pub fn get_slice(&'a self, bound: impl Into<BoundMD<D>>) -> SliceMD<'a, D, T> {
        let mut bound = bound.into();
        debug_assert!(self.bound.overlap(&bound) == OverlapResult::SelfContainsOther);
        *bound.start_mut() += self.bound.start();
        SliceMD {
            array: self.array,
            bound,
        }
    }
}

impl<'a, const D: usize, I: Into<Point<D>>, T> Index<I> for SliceMD<'a, D, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index)
    }
}

pub type Slice2D<'a, T> = SliceMD<'a, 2, T>;

impl<'a, T> Slice2D<'a, T> {
    pub fn width(&self) -> i64 {
        self.sizes()[0]
    }

    pub fn height(&self) -> i64 {
        self.sizes()[1]
    }
}

impl<'a, T> Slice2D<'a, T> {
    pub fn print(&self, f: impl Fn(&T) -> char) {
        print!("\n");
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", f(&self[[x, y]]));
            }
            print!("\n");
        }
        print!("\n");
    }
}

pub struct SliceMutMD<'a, const D: usize, T> {
    array: &'a mut ArrayMD<D, T>,
    bound: BoundMD<D>,
}

impl<'a, const D: usize, T> SliceMutMD<'a, D, T> {
    pub fn new(array: &'a mut ArrayMD<D, T>, bound: impl Into<BoundMD<D>>) -> Self {
        let bound = bound.into();
        debug_assert!(bound.overlap(&BoundMD::new(Point::ZERO, array.sizes())) == OverlapResult::OtherContainsSelf, "array sizes {}, bound {:?}", array.sizes(), bound);
        SliceMutMD {
            array,
            bound,
        }
    }

    pub fn sizes(&self) -> Point<D> {
        self.bound.sizes()
    }

    pub fn size(&self, d: usize) -> i64 {
        self.bound.sizes()[d]
    }

    pub fn is_in_bound(&self, coords: impl Into<Point<D>>) -> bool {
        let coords: Point<D> = coords.into();
        for d in 0..D {
            if coords[d] < 0 || coords[d] >= self.bound.sizes()[d] {
                return false;
            }
        }
        true
    }

    pub fn get(&self, coords: impl Into<Point<D>>) -> &T {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            panic!("coords (= {:?}) out of bound (= {:?})", coords, self.bound.sizes());
        }

        self.array.get(coords + self.bound.start())
    }

    pub fn get_mut(&mut self, coords: impl Into<Point<D>>) -> &mut T {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            panic!("coords (= {:?}) out of bound (= {:?})", coords, self.bound.sizes());
        }

        self.array.get_mut(coords + self.bound.start())
    }

    pub fn try_get(&self, coords: impl Into<Point<D>>) -> Option<&T> {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            return None;
        }

        self.array.try_get(coords + self.bound.start())
    }

    pub fn try_get_mut(&mut self, coords: impl Into<Point<D>>) -> Option<&mut T> {
        let coords = coords.into();
        if !self.is_in_bound(coords) {
            return None;
        }

        self.array.try_get_mut(coords + self.bound.start())
    }

    pub fn get_slice(&'a self, bound: impl Into<BoundMD<D>>) -> SliceMD<'a, D, T> {
        let mut bound = bound.into();
        debug_assert!(self.bound.overlap(&bound) == OverlapResult::SelfContainsOther);
        *bound.start_mut() += self.bound.start();
        SliceMD {
            array: self.array,
            bound,
        }
    }

    pub fn get_slice_mut(&'a mut self, bound: impl Into<BoundMD<D>>) -> SliceMutMD<'a, D, T> {
        let mut bound = bound.into();
        debug_assert!(self.bound.overlap(&bound) == OverlapResult::SelfContainsOther);
        *bound.start_mut() += self.bound.start();
        SliceMutMD {
            array: self.array,
            bound,
        }
    }
}

impl<'a, const D: usize, I: Into<Point<D>>, T> Index<I> for SliceMutMD<'a, D, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index)
    }
}

impl<'a, const D: usize, I: Into<Point<D>>, T> IndexMut<I> for SliceMutMD<'a, D, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index)
    }
}

pub type SliceMut2D<'a, T> = SliceMutMD<'a, 2, T>;

impl<'a, T> SliceMut2D<'a, T> {
    pub fn width(&self) -> i64 {
        self.sizes()[0]
    }

    pub fn height(&self) -> i64 {
        self.sizes()[1]
    }
}

impl<'a, T> SliceMut2D<'a, T> {
    pub fn print(&self, f: impl Fn(&T) -> char) {
        print!("\n");
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", f(&self[[x, y]]));
            }
            print!("\n");
        }
        print!("\n");
    }
}
