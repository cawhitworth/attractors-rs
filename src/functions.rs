use std::fmt::Display;

use crate::geometry::*;

pub struct Coeffs {
    a: f64,
    b: f64,
    c: f64,
    d: f64
}

impl Coeffs {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Coeffs {
        Coeffs { a,b,c,d }
    }
}

impl Display for Coeffs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ a: {}, b: {}, c: {}, d: {} ]", self.a, self.b, self.c, self.d)
    }
}

pub fn clifford_attractor(p: &Coord, coeffs: &Coeffs) -> Coord {
    Coord {
        x: f64::sin(coeffs.a * p.y) + coeffs.c * f64::cos(coeffs.a * p.x),
        y: f64::sin(coeffs.b * p.x) + coeffs.d * f64::cos(coeffs.b * p.y)
    }
}

pub fn bind_function<'a, T, S>(function: impl Fn(&T, &S) -> T + 'a, c: &'a S)
        -> impl Fn(&T) -> T + 'a {
    move |p| function(p,c)
}

pub fn find_bounds(f: &impl Fn(&Coord) -> Coord, iterations: usize) -> Rect {
    let mut bounds = Rect::from_coords(0.0, 0.0, 0.0, 0.0);
    let mut p = Coord::new(0.0, 0.0);

    for _ in 0..iterations {
        p = f(&p);

        bounds.bl.x = if p.x < bounds.bl.x { p.x } else { bounds.bl.x };
        bounds.bl.y = if p.y < bounds.bl.y { p.y } else { bounds.bl.y };
        bounds.tr.x = if p.x > bounds.tr.x { p.x } else { bounds.tr.x };
        bounds.tr.y = if p.y > bounds.tr.y { p.y } else { bounds.tr.y };
    }

    bounds.bl.x *= 1.05;
    bounds.bl.y *= 1.05;
    bounds.tr.x *= 1.05;
    bounds.tr.y *= 1.05;

    bounds

}