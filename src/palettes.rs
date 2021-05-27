use std::vec;

use crate::colour::{GradientPoint, Gradient};

pub fn get_palettes() -> Vec<Gradient> {
    vec![
        Gradient::new("WhitePurple", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([200, 100, 255])),
            GradientPoint::new( 0.2, image::Rgb([127, 0, 255])),
            GradientPoint::new( 0.4, image::Rgb([64, 0, 127])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
    ])]
}