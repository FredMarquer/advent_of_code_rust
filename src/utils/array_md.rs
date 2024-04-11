use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::solvers::ParseSolverError;
use crate::utils::{Point, Point2D};

pub struct ArrayMD<const D: usize, T> {
    data: Vec<T>,
    sizes: Point<D>,
}

impl<const D: usize, T> ArrayMD<D, T> {
    pub fn sizes(&self) -> Point<D> {
        self.sizes
    }

    pub fn size(&self, d: usize) -> i32 {
        self.sizes[d]
    }

    pub fn is_in_bound(&self, coords: impl Into<Point<D>>) -> bool {
        let coords: Point<D> = coords.into();
        for d in 0..D {
            if coords[d] < 0 || coords[d] >= self.sizes[d] {
                return false;
            }
        }
        true
    }

    pub fn get(&self, coords: impl Into<Point<D>>) -> &T {
        let index = self.get_index(coords.into());
        &self.data[index]
    }

    pub fn get_mut(&mut self, coords: impl Into<Point<D>>) -> &mut T {
        let index = self.get_index(coords.into());
        &mut self.data[index]
    }

    pub fn try_get(&self, coords: impl Into<Point<D>>) -> Option<&T> {
        let index = self.try_get_index(coords.into())?;
        Some(&self.data[index])
    }

    pub fn try_get_mut(&mut self, coords: impl Into<Point<D>>) -> Option<&mut T> {
        let index = self.try_get_index(coords.into())?;
        Some(&mut self.data[index])
    }

    fn get_index(&self, coords: Point<D>) -> usize {
        let mut index = 0;
        let mut step = 1;
        for d in 0..D {
            if coords[d] < 0 || coords[d] >= self.sizes[d] {
                panic!("coords (= {:?}) out of bound (= {:?})", coords, self.sizes)
            }
            index += coords[d] * step;
            step *= self.sizes[d];
        }
        usize::try_from(index).unwrap()
    }

    fn try_get_index(&self, coords: Point<D>) -> Option<usize> {
        let mut index = 0;
        let mut step = 1;
        for d in 0..D {
            if coords[d] < 0 || coords[d] >= self.sizes[d] {
                return None;
            }
            index += coords[d] * step;
            step *= self.sizes[d];
        }
        Some(usize::try_from(index).unwrap())
    }

    pub fn iter<'a>(&'a self) -> core::slice::Iter<'a, T> {
        self.data.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, T> {
        self.data.iter_mut()
    }
}


impl<const D: usize, I: Into<Point<D>>, T> Index<I> for ArrayMD<D, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index)
    }
}

impl<const D: usize, I: Into<Point<D>>, T> IndexMut<I> for ArrayMD<D, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<const D: usize, T: Default + Clone> ArrayMD<D, T> {
    pub fn new(sizes: impl Into<Point<D>>) -> Self {
        let sizes: Point<D> = sizes.into();
        let data_length = sizes.as_slice().iter()
            .fold(1, |acc, size| acc * size);
        ArrayMD {
            data: vec![T::default(); data_length as usize],
            sizes,
        }
    }
}

impl<const D: usize, T: Clone> ArrayMD<D, T> {
    pub fn copy_to_with_offset(&self, dst: &mut ArrayMD<D, T>, offset: impl Into<Point<D>>) {
        let offset: Point<D> = offset.into();
        if !dst.is_in_bound(self.sizes + offset) {
            panic!("copy_to_with_offset out of bound (self.sizes = {:?}, dst.sizes = {:?}, offset = {:?})", self.sizes, dst.sizes, offset);
        }

        let width = usize::try_from(self.sizes[0]).unwrap();
        let mut current = Point::ZERO;
        loop {
            let src_start = self.get_index(current);
            let src_end = src_start + width;
            let dst_start = dst.get_index(current + offset);
            let dst_end = dst_start + width;
            dst.data[dst_start..dst_end].clone_from_slice(&self.data[src_start..src_end]);
            
            if incremente_coords(&mut current, self.sizes) {
                break;
            }
        }
        
        fn incremente_coords<const D: usize>(coords: &mut Point<D>, end: Point<D>) -> bool {
            for d in (1..D).rev() {
                coords[d] += 1;
                if coords[d] >= end[d] {
                    coords[d] = 0;
                } else {
                    return false;
                }
            }
            true
        }
    }
}

pub type Array2D<T> = ArrayMD<2, T>;

impl<T> Array2D<T> {
    pub fn width(&self) -> i32 {
        self.sizes[0]
    }

    pub fn height(&self) -> i32 {
        self.sizes[1]
    }

    pub fn from_str_map(s: &str, mut f: impl FnMut(Point2D, char) -> Result<T, ParseSolverError>) -> Result<Self, ParseSolverError> {
        let width = s.lines().next().ok_or(ParseSolverError::new("fail to parse array width"))?.chars().count();
        let height = s.lines().count();
        if width  == 0 || height == 0 {
            return Err(ParseSolverError::new(format!("invalid array width (= {width}) or height (= {height})")));
        }

        let size = width * height;
        let mut data = Vec::with_capacity(size);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data.push(f(Point2D::new(x as i32, y as i32), c)?);
            }
        }

        if data.len() != size {
            return Err(ParseSolverError::new(format!("array data length (= {}) don't match total size (= {} (width = {}, height = {}))", data.len(), size, width, height)));
        }
        
        Ok(ArrayMD {
            data,
            sizes: Point2D::new(width as i32, height as i32)
        })
    }
}

impl FromStr for Array2D<char> {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        ArrayMD::from_str_map(s, |_, c| Ok(c))
    }
}

pub type Array3D<T> = ArrayMD<3, T>;

impl<T> Array3D<T> {
    pub fn width(&self) -> i32 {
        self.sizes[0]
    }

    pub fn height(&self) -> i32 {
        self.sizes[1]
    }

    pub fn depth(&self) -> i32 {
        self.sizes[2]
    }
}