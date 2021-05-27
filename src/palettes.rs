use std::vec;

use crate::colour::{GradientPoint, Gradient};

pub fn get_palettes() -> Vec<Gradient> {
    vec![
        Gradient::new("Hot", vec![
            GradientPoint::new( 0.0, image::Rgb([50, 0, 0])),
            GradientPoint::new( 0.1, image::Rgb([255, 0, 0])),
            GradientPoint::new( 0.4, image::Rgb([255, 255, 0])),
            GradientPoint::new( 1.0, image::Rgb([255, 255, 255]))
        ]),
        Gradient::new("Smoke", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([150, 150, 150])),
            GradientPoint::new( 0.2, image::Rgb([100, 100, 100])),
            GradientPoint::new( 0.4, image::Rgb([25, 25, 25])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("BlueSmoke", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([150, 150, 170])),
            GradientPoint::new( 0.2, image::Rgb([100, 100, 120])),
            GradientPoint::new( 0.4, image::Rgb([25, 25, 45])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("WhiteOrange", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([255, 200, 100])),
            GradientPoint::new( 0.2, image::Rgb([255, 127, 0])),
            GradientPoint::new( 0.4, image::Rgb([127, 64, 0])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("WhiteCyan", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([100, 200, 255])),
            GradientPoint::new( 0.2, image::Rgb([0, 127, 255])),
            GradientPoint::new( 0.4, image::Rgb([0, 64, 127])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("WhitePurple", vec![
            GradientPoint::new( 0.0, image::Rgb([255, 255, 255])),
            GradientPoint::new( 0.1, image::Rgb([200, 100, 255])),
            GradientPoint::new( 0.2, image::Rgb([127, 0, 255])),
            GradientPoint::new( 0.4, image::Rgb([64, 0, 127])),
            GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("PurpleBlue", vec![
            GradientPoint::new( 0.0, image::Rgb([0, 0, 25])),
            GradientPoint::new( 0.1, image::Rgb([0, 0, 125])),
            GradientPoint::new( 0.2, image::Rgb([50, 0, 175])),
            GradientPoint::new( 0.4, image::Rgb([100, 0, 255])),
            GradientPoint::new( 1.0, image::Rgb([255, 0, 255])),
        ]),
        Gradient::new("PetrolOlive", vec![
                GradientPoint::new( 0.0, image::Rgb([3, 14, 14])),
                GradientPoint::new( 0.1, image::Rgb([34, 145, 140])),
                GradientPoint::new( 0.2, image::Rgb([85, 145, 48])),
                GradientPoint::new( 0.5, image::Rgb([38, 65, 22])),
                GradientPoint::new( 1.0, image::Rgb([0,0,0]))
        ]),
        Gradient::new("PastelPink", vec![
            GradientPoint::new( 0.0, image::Rgb([200, 180, 180])),
            GradientPoint::new( 0.01, image::Rgb([255, 230, 230])),
            GradientPoint::new( 0.1, image::Rgb([230, 200, 200])),
            GradientPoint::new( 1.0, image::Rgb([180, 150, 150]))
        ])]
}