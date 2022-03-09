use std::io::{Write, stdout};

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
            function: &F,
            log: bool) -> (DeepBitmap, u32)
    where F: Fn(&Coord) -> Coord {

    let mut bitmap = DeepBitmap::new(width, height);

    let x_scale = (width as f64 - 1.0) / bounds.width();
    let y_scale = (height as f64 - 1.0) / bounds.height();
    let mut max_exposure = 0;

    let mut p = Coord::new(0.0, 0.0);

    let reset = iterations / 10;

    if log {
        print!("Exposing");
        stdout().flush().ok();
    }

    for iter in 0..iterations {
        if log && (iter % reset == 0) {
            print!(".");
            stdout().flush().ok();
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
    if log {
        println!();
    }
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

fn find_interesting_coeffs<F>(function: &F ) -> (Coeffs, Rect)
    where F: Fn(&Coord, &Coeffs) -> Coord {

    let mut coeffs;
    let mut bounds;

    let mut rng = rand::thread_rng();
    let mut rand_coeff = || (rng.gen::<f64>() * 4.0) - 2.0;

    loop {
        coeffs =
            Coeffs::new( rand_coeff(), rand_coeff(), rand_coeff(), rand_coeff());

        let fn_with_coeffs = bind_1(function, &coeffs);

        bounds = find_bounds(&fn_with_coeffs, 10000);

        let (_, max_exposure) =
            expose(640, 512, &bounds, 10000, &fn_with_coeffs, false);

        if max_exposure < 10 {
            break;
        }
    } 

    (coeffs, bounds)
}

fn pick<T>(collection: &Vec<T>) -> &T {
    let size = collection.len();
    let index = rand::thread_rng().gen::<u32>() as usize % size;
    &collection[index]
}

fn main() -> Result<(), image::ImageError> {

    let w = 1920;
    let h = 1080;
<<<<<<< Updated upstream
    let iters = 100 * 1000 * 1000;
=======
    let iters = 1000000000;
>>>>>>> Stashed changes
  
    let functions = get_functions();
    let (function_name, function) = pick(&functions);
   
    println!("Using {}", function_name);
    println!("Finding interesting coefficients...");
    let (c, bounds) = find_interesting_coeffs(&function);

    println!("Using coefficients: {}", c);
    println!("Bounds: {}", bounds);

    let f = bind_1(&function, &c);

    let (bitmap, max_exposure) =
        expose(w, h, &bounds,iters, &f, true);

    println!("Max exposure: {}", max_exposure);

    let palettes = palettes::get_palettes();
    let chosen_palette = pick(&palettes);
    println!("Developing with palette {}", &chosen_palette.name);

    let image =
        develop(&bitmap, max_exposure, 2.0, chosen_palette);
    image.save("output.png")
}
