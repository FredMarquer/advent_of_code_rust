pub struct Array2D<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Array2D<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        let index = x + y * self.width;
        &self.data[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = x + y * self.width;
        &mut self.data[index]
    }
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new(width: usize, height: usize,) -> Self {
        Array2D {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }
}

impl Array2D<char> {
    pub fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let mut data = Vec::with_capacity(width * height);

        for line in input.lines() {
            for c in line.chars() {
                data.push(c);
            }
        }

        Array2D {
            data,
            width,
            height,
        }
    }
}