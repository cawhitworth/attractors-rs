use image;

mod geometry;
use geometry::*;

mod bitmap;
use bitmap::*;

mod functions;
use functions::*;

fn expose(width: usize, height: usize, bounds: &Rect,
          iterations: usize,
          function: impl Fn(&Coord) -> Coord) -> (DeepBitmap, u32) {

    let mut bitmap = DeepBitmap::new(width, height);

    let x_scale = (width as f64 - 1.0) / bounds.width();
    let y_scale = (height as f64 - 1.0) / bounds.height();
    let mut max_exposure = 0;

    let mut p = Coord::new(0.0, 0.0);

    for _ in 0..iterations {
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

    (bitmap, max_exposure)
}

fn main() -> Result<(), image::ImageError>{
    let w = 1920;
    let h = 1080;
    let iters = 100000000;
    let c = Coeffs::new(1.79048,1.26424,1.90425,-1.33372);
    let bounds = Rect::from_coords(-3.0, -2.5, 3.0, 2.2);

    let f = bind_function(clifford_attractor, &c);

    let (bitmap, max_exposure) = expose(w, h, &bounds,iters, f);

    println!("Max exposure: {}", max_exposure);

    let mut image = image::RgbImage::new(w as u32, h as u32);

    for y in 0..h {
        for x in 0..w {
            let c = std::cmp::max(0, std::cmp::min(bitmap.point(x,y), 255)) as u8;
            image.put_pixel(x as u32, y as u32, image::Rgb([c, c, c]));
        }
    }

    image.save("output.png")
}
