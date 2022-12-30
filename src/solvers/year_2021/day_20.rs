use std::mem;
use crate::solvers::{Solver, SolverResult};

const OFFSETS: [(usize, usize); 9] = [
    (0, 0),
    (1, 0),
    (2, 0),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 2),
    (1, 2),
    (2, 2),
];

pub fn create() -> Day20 {
    let input = include_str!("inputs/20.txt");
    let mut splits = input.split("\r\n\r\n");

    let mut image_enhancement_algorithm = Box::new([false; 512]);
    for (index, c) in splits.next().unwrap().chars().enumerate() {
        image_enhancement_algorithm[index] = char_to_pixel(c);
    }
    
    let input_image = Image::from_input(splits.next().unwrap());

    Day20 { image_enhancement_algorithm, input_image }
}

pub struct Day20 {
    image_enhancement_algorithm: Box<[bool; 512]>,
    input_image: Image,
}

impl Solver for Day20 {
    fn run_part1(&self) -> SolverResult {
        let ouput_image = self.input_image.apply_image_enhancement_algorithm(&self.image_enhancement_algorithm, 2);
        ouput_image.count_lit_pixels().into()
    }

    fn run_part2(&self) -> SolverResult {
        let ouput_image = self.input_image.apply_image_enhancement_algorithm(&self.image_enhancement_algorithm, 50);
        ouput_image.count_lit_pixels().into()
    }
}

fn char_to_pixel(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _=> panic!("invalid character: {c}"),
    }
}


#[derive(Clone)]
struct Image {
    pixels: Box<[bool]>,
    width: usize,
    heigth: usize,
}

impl Image {
    fn new(width: usize, heigth: usize) -> Image {
        Image {
            pixels: vec![false; width * heigth].into_boxed_slice(),
            width,
            heigth,
        }
    }

    fn from_input(input: &str) -> Image {
        let width = input.lines().next().unwrap().chars().count();
        let heigth = input.lines().count();
        let mut image = Image::new(width, heigth);

        for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let index = image.get_index(x, y).unwrap();
            image.pixels[index] = char_to_pixel(c);
        }}

        image
    }

    fn copy_to(&self, other: &mut Image) {
        other.set_size(self.width, self.heigth);

        for y in 0..self.heigth {
        for x in 0..self.width {
            let index = self.get_index(x, y).unwrap();
            other.pixels[index] = self.pixels[index];
        }}
    }

    fn set_size(&mut self, width: usize, heigth: usize) {
        assert!(width * heigth <= self.pixels.len());
        self.width = width;
        self.heigth = heigth;
    }

    fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.heigth {
            return None;
        }

        Some(x + y * self.width)
    }

    fn apply_image_enhancement_algorithm(&self, image_enhancement_algorithm: &[bool; 512], iteration_count: usize) -> Image {
        let mut current_width = self.width;
        let mut current_heigth = self.width;
        let max_width = current_width + iteration_count * 2;
        let max_heigth = current_heigth + iteration_count * 2;

        let mut input_image = Image::new(max_width, max_heigth);
        let mut output_image = Image::new(max_width, max_heigth);
        self.copy_to(&mut input_image);

        for current_iteration in 0..iteration_count {
            current_width += 2;
            current_heigth += 2;
            output_image.set_size(current_width, current_heigth);
            input_image.apply_image_enhancement_algorithm_internal(&mut output_image, image_enhancement_algorithm, current_iteration);
            mem::swap(&mut input_image, &mut output_image);
        }

        input_image
    }

    fn apply_image_enhancement_algorithm_internal(&self, output_image: &mut Image, image_enhancement_algorithm: &[bool; 512], current_iteration: usize) {
        assert!(output_image.width == self.width + 2);
        assert!(output_image.heigth == self.heigth + 2);
        assert!(!(image_enhancement_algorithm[0] && image_enhancement_algorithm[image_enhancement_algorithm.len() - 1]));

        let infinite_pixel = if current_iteration % 2 == 1 { image_enhancement_algorithm[0] } else { image_enhancement_algorithm[image_enhancement_algorithm.len() - 1] };
        
        for new_y in 0..output_image.heigth {
        for new_x in 0..output_image.width {
            let mut enhancement_index = 0;
            for (bit_index, offset) in OFFSETS.iter().enumerate() {
                let x = new_x.wrapping_sub(offset.0);
                let y = new_y.wrapping_sub(offset.1);
                if let Some(pixel_index) = self.get_index(x, y) {
                    if self.pixels[pixel_index] {
                        enhancement_index |= 1 << bit_index;
                    }
                } else if infinite_pixel {
                    enhancement_index |= 1 << bit_index;
                }
            }

            let new_index = output_image.get_index(new_x, new_y).unwrap();
            output_image.pixels[new_index] = image_enhancement_algorithm[enhancement_index];
        }}
    }

    fn count_lit_pixels(&self) -> i64 {
        let capacity = self.width * self.heigth;
        let mut count = 0;
        for index in 0..capacity {
            if self.pixels[index] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 5301.into(), "Part1");
        assert_eq!(day.run_part2(), 19492.into(), "Part2");
    }
}