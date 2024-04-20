use std::convert::From;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::ops::{Index, IndexMut};
use std::cmp::Ord;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point<const D: usize> {
    coords: [i64; D]
}

impl<const D: usize> Point<D> {
    pub const ZERO: Point<D> = Point { coords: [0; D] };
    pub const ONE:  Point<D> = Point { coords: [1; D] };
    pub const MIN:  Point<D> = Point { coords: [i64::MIN; D] };
    pub const MAX:  Point<D> = Point { coords: [i64::MAX; D] };

    pub fn is_zero(&self) -> bool {
        *self == Point::ZERO
    }

    pub fn is_unit(&self) -> bool {
        self.manhattan_magnitude() == 1
    }

    pub fn manhattan_magnitude(&self) -> i64 {
        self.as_slice()
            .iter()
            .map(|coord| coord.abs())
            .sum()
    }

    pub fn manhattan_distance(self, other: Self) -> i64 {
        self.sub(other)
            .manhattan_magnitude()
    }

    pub fn volume(&self) -> i64 {
        self.as_slice()
            .iter()
            .product()
    }

    pub fn max(self, other: Self) -> Self {
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = Ord::max(self.coords[d], other.coords[d]);
        }
        Point::from(coords)
    }

    pub fn min(self, other: Self) -> Self {
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = Ord::min(self.coords[d], other.coords[d]);
        }
        Point::from(coords)
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        let mut coords = [0; D];
        for d in 0..D {
            coords[d] = Ord::clamp(self.coords[d], min.coords[d], max.coords[d]);
        }
        Point::from(coords)
    }

    pub fn opposite(self) -> Self {
        self.mul(-1)
    }

    pub fn dot(self, other: Self) -> i64 {
        self.mul(other)
            .as_slice()
            .iter()
            .sum()
    }

    pub fn max_coord(&self) -> i64 {
        *self.as_slice()
            .iter()
            .max()
            .unwrap()
    }

    pub fn min_coord(&self) -> i64 {
        *self.as_slice()
            .iter()
            .min()
            .unwrap()
    }

    pub const fn as_slice(&self) -> &[i64; D] {
        &self.coords
    }

    pub fn as_mut_slice(&mut self) -> &mut [i64; D] {
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

impl<const D: usize, T: Into<Point<D>>> AddAssign<T> for Point<D> {
    fn add_assign(&mut self, other: T) {
        let other: Point<D> = other.into();
        for d in 0..D {
            self.coords[d] += other.coords[d];
        }
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

impl<const D: usize, T: Into<Point<D>>> SubAssign<T> for Point<D> {
    fn sub_assign(&mut self, other: T) {
        let other: Point<D> = other.into();
        for d in 0..D {
            self.coords[d] -= other.coords[d];
        }
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

impl<const D: usize, T: Into<Point<D>>> MulAssign<T> for Point<D> {
    fn mul_assign(&mut self, other: T) {
        let other: Point<D> = other.into();
        for d in 0..D {
            self.coords[d] *= other.coords[d];
        }
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

impl<const D: usize, T: Into<Point<D>>> DivAssign<T> for Point<D> {
    fn div_assign(&mut self, other: T) {
        let other: Point<D> = other.into();
        for d in 0..D {
            self.coords[d] /= other.coords[d];
        }
    }
}

impl<const D: usize> Index<usize> for Point<D> {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const D: usize> Index<i32> for Point<D> {
    type Output = i64;

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

impl<const D: usize> From<i64> for Point<D> {
    fn from(value: i64) -> Self {
        Self { coords: [value; D] }
    }
}

impl<const D: usize> From<[i32; D]> for Point<D> {
    fn from(coords: [i32; D]) -> Self {
        let mut new_coors = [0; D];
        for d in 0..D {
            new_coors[d] = i64::from(coords[d]);
        }
        Self { coords: new_coors }
    }
}

impl<const D: usize> From<[i64; D]> for Point<D> {
    fn from(coords: [i64; D]) -> Self {
        Self { coords }
    }
}

impl<const D: usize> From<[usize; D]> for Point<D> {
    fn from(coords: [usize; D]) -> Self {
        let mut new_coors = [0; D];
        for d in 0..D {
            new_coors[d] = i64::try_from(coords[d]).unwrap();
        }
        Self { coords: new_coors }
    }
}

use std::fmt;
impl<const D: usize> fmt::Display for Point<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.coords)
    }
}

pub type Point2D = Point<2>;

impl Point2D {
    pub const RIGHT: Point2D = Point2D::new( 1,  0);
    pub const LEFT:  Point2D = Point2D::new(-1,  0);
    pub const UP:    Point2D = Point2D::new( 0,  1);
    pub const DOWN:  Point2D = Point2D::new( 0, -1);

    pub const fn new(x: i64, y: i64) -> Self {
        Point2D { coords: [x, y]}
    }

    pub const fn is_horizontal(&self) -> bool {
        self.x() != 0 && self.y() == 0
    }

    pub const fn is_vertical(&self) -> bool {
        self.x() == 0 && self.y() != 0
    }

    pub const fn x(&self) -> i64 {
        self.coords[0]
    }

    pub fn x_mut(&mut self) -> &mut i64 {
        &mut self.coords[0]
    }

    pub const fn y(&self) -> i64 {
        self.coords[1]
    }

    pub fn y_mut(&mut self) -> &mut i64 {
        &mut self.coords[1]
    }

    pub fn area(&self) -> i64 {
        self.volume()
    }
}

impl From<(i64, i64)> for Point2D {
    fn from(coords: (i64, i64)) -> Self {
        Self { coords: [coords.0, coords.1] }
    }
}

pub type Point3D = Point<3>;

impl Point3D {
    pub const RIGHT:    Point3D = Point3D::new( 1,  0,  0);
    pub const LEFT:     Point3D = Point3D::new(-1,  0,  0);
    pub const UP:       Point3D = Point3D::new( 0,  1,  0);
    pub const DOWN:     Point3D = Point3D::new( 0, -1,  0);
    pub const FORWARD:  Point3D = Point3D::new( 0,  0,  1);
    pub const BACKWARD: Point3D = Point3D::new( 0,  0, -1);

    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D { coords: [x, y, z]}
    }

    pub const fn x(&self) -> i64 {
        self.coords[0]
    }

    pub fn x_mut(&mut self) -> &mut i64 {
        &mut self.coords[0]
    }

    pub const fn y(&self) -> i64 {
        self.coords[1]
    }

    pub fn y_mut(&mut self) -> &mut i64 {
        &mut self.coords[1]
    }

    pub const fn z(&self) -> i64 {
        self.coords[2]
    }

    pub fn z_mut(&mut self) -> &mut i64 {
        &mut self.coords[2]
    }

    pub const fn xy(&self) -> Point2D {
        Point2D::new(self.coords[0],self.coords[1])
    }

    pub const fn xz(&self) -> Point2D {
        Point2D::new(self.coords[0],self.coords[2])
    }

    pub const fn yz(&self) -> Point2D {
        Point2D::new(self.coords[1],self.coords[2])
    }

    pub fn cross(self, other: Self) -> Self {
        Point3D::new(
            (self.y() * other.z()) - (self.z() * other.y()),
            (self.z() * other.x()) - (self.x() * other.z()),
            (self.x() * other.y()) - (self.y() * other.x()),
        )
    }
}

impl From<(i64, i64, i64)> for Point3D {
    fn from(coords: (i64, i64, i64)) -> Self {
        Self { coords: [coords.0, coords.1, coords.2] }
    }
}