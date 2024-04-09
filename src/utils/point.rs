use std::convert::From;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point<const D: usize> {
    coords: [i32; D]
}

impl<const D: usize> Point<D> {
    pub const fn new(coords: [i32; D]) -> Self {
        Self { coords }
    }

    pub const fn zero() -> Self {
        Self { coords: [0; D] }
    }

    pub const fn as_slice(&self) -> &[i32; D] {
        &self.coords
    }

    pub fn as_slice_mut(&mut self) -> &mut [i32; D] {
        &mut self.coords
    }
}

impl<const D: usize> Default for Point<D> {
    fn default() -> Self {
        Self { coords: [0; D] }
    }
}

impl<const D: usize, T: Into<Point<D>>> Add<T> for Point<D> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        let other: Point<D> = other.into();
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = self.coords[d] + other.coords[d];
        }
        Self { coords }
    }
}

impl<const D: usize, T: Into<Point<D>>> Sub<T> for Point<D> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        let other: Point<D> = other.into();
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = self.coords[d] - other.coords[d];
        }
        Self { coords }
    }
}

impl<const D: usize, T: Into<Point<D>>> Mul<T> for Point<D> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let other: Point<D> = other.into();
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = self.coords[d] * other.coords[d];
        }
        Self { coords }
    }
}

impl<const D: usize, T: Into<Point<D>>> Div<T> for Point<D> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        let other: Point<D> = other.into();
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = self.coords[d] / other.coords[d];
        }
        Self { coords }
    }
}

impl<const D: usize> Index<usize> for Point<D> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const D: usize> Index<i32> for Point<D> {
    type Output = i32;

    fn index(&self, index: i32) -> &Self::Output {
        &self.coords[usize::try_from(index).unwrap()]
    }
}

impl<const D: usize> IndexMut<usize> for Point<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

impl<const D: usize> IndexMut<i32> for Point<D> {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.coords[usize::try_from(index).unwrap()]
    }
}

impl<const D: usize> From<i32> for Point<D> {
    fn from(value: i32) -> Self {
        Self { coords: [value; D] }
    }
}

impl<const D: usize> From<[i32; D]> for Point<D> {
    fn from(coords: [i32; D]) -> Self {
        Self { coords }
    }
}

pub type Point2D = Point<2>;

impl Point2D {
    pub const fn x(&self) -> i32 {
        self.coords[0]
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.coords[0]
    }

    pub const fn y(&self) -> i32 {
        self.coords[1]
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.coords[1]
    }
}

pub type Point3D = Point<3>;

impl Point3D {
    pub const fn x(&self) -> i32 {
        self.coords[0]
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.coords[0]
    }

    pub const fn y(&self) -> i32 {
        self.coords[1]
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.coords[1]
    }

    pub const fn z(&self) -> i32 {
        self.coords[2]
    }

    pub fn z_mut(&mut self) -> &mut i32 {
        &mut self.coords[2]
    }
}