use std::fmt;
use std::vec;

pub struct DeepBitmap {
    pixels: vec::Vec<u32>,
    width: usize,
    height: usize
}

impl DeepBitmap {
    pub fn new(width: usize, height: usize) -> DeepBitmap {
        DeepBitmap {
            pixels: vec![0; width * height],
            width: width,
            height: height
        }
    }

    pub fn plot(&mut self, x: usize, y: usize, intensity: u32) {
        self.pixels[x + y * self.width] = intensity;
    }

    pub fn point(&self, x: usize, y: usize) -> u32 {
        self.pixels[x + y * self.width]
    }
}

impl fmt::Display for DeepBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeepBitmap({},{})", self.width, self.height)
    }
}