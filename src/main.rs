use std::{cmp::{max, min}, io::Write};

use image;
use rand::prelude::*;

mod geometry;
use geometry::*;

mod bitmap;
use bitmap::*;

mod functions;
use functions::*;

mod colour;
use colour::Gradient;

mod palettes;

fn expose<F>(width: usize, height: usize, bounds: &Rect,
            iterations: usize,
            function: &F) -> (DeepBitmap, u32)
    where F: Fn(&Coord) -> Coord {

    let mut bitmap = DeepBitmap::new(width, height);

    let x_scale = (width as f64 - 1.0) / bounds.width();
    let y_scale = (height as f64 - 1.0) / bounds.height();
    let mut max_exposure = 0;

    let mut p = Coord::new(0.0, 0.0);

    let reset = iterations / 10;

    print!("Exposing");
    std::io::stdout().flush().ok();

    for iter in 0..iterations {
        if iter % reset == 0 {
            print!(".");
            std::io::stdout().flush().ok();
        }

        p = function(&p);
        let plot_x = ((p.x - bounds.bl.x) * x_scale) as usize;
        let plot_y = ((p.y - bounds.bl.y) * y_scale) as usize;

        if plot_x > 0 && plot_x < width && plot_y > 0 && plot_y < height {

            let sampled = bitmap.point(plot_x, plot_y) + 1;
            if sampled > max_exposure {
                max_exposure = sampled;
            }

            bitmap.plot(plot_x, plot_y, sampled);
        }
    }
    println!();
    (bitmap, max_exposure)
}

fn develop(bitmap: &DeepBitmap, max_exposure: u32, gamma: f64, gradient: &Gradient) -> image::RgbImage {
    let mut output = image::RgbImage::new(bitmap.width as u32, bitmap.height as u32);

    let max_exposure_f = max_exposure as f64;
    for y in 0..bitmap.height {
        for x in 0..bitmap.width {
            let sample = bitmap.point(x, y);
            let exposure = sample as f64 / max_exposure_f;
            let gamma_corrected = exposure.powf(1.0 / gamma);
            let clamped = if gamma_corrected < 0.0 { 0.0 } else if gamma_corrected > 1.0 { 1.0 } else { gamma_corrected }; 
            let colour = gradient.colour_at(clamped);
            output.put_pixel(x as u32, y as u32, colour);
        }
    }

    output
}

fn find_interesting_coeffs<F>(function: &F ) -> Coeffs
    where F: Fn(&Coord, &Coeffs) -> Coord {

    let mut coeffs;

    let mut rng = rand::thread_rng();
    let mut rand_coeff = || (rng.gen::<f64>() * 4.0) - 2.0;

    loop {
        coeffs =
            Coeffs::new( rand_coeff(), rand_coeff(), rand_coeff(), rand_coeff());

        let fn_with_coeffs = bind_1(function, &coeffs);

        let bounds = find_bounds(&fn_with_coeffs, 10000);

        let (_, max_exposure) =
            expose(640, 512, &bounds, 10000, &fn_with_coeffs);

        if max_exposure < 10 {
            break;
        }
    } 

    coeffs
}

fn main() -> Result<(), image::ImageError> {

    let w = 1920*2;
    let h = 1080*2;
    let iters = 100000000;
    
    println!("Finding interesting coefficients...");
    let c = find_interesting_coeffs(&clifford_attractor);

    println!("Using coefficients: {}", c);
    let f = bind_1(&clifford_attractor, &c);

    let bounds = find_bounds(&f, 10000);

    println!("Bounds: {}", bounds);

    let (bitmap, max_exposure) =
        expose(w, h, &bounds,iters, &f);

    println!("Max exposure: {}", max_exposure);

    let image =
        develop(&bitmap, max_exposure, 1.5, &palettes::get_palettes()[0]);
    image.save("output.png")
}
