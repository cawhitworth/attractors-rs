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

pub fn peter_de_jong_attractor(p: &Coord, coeffs: &Coeffs) -> Coord {
    Coord {
        x: f64::sin(coeffs.a * p.y) - f64::cos(coeffs.b * p.x),
        y: f64::sin(coeffs.c * p.y) - f64::cos(coeffs.d * p.x),
    }
}

pub fn experimental(p: &Coord, coeffs: &Coeffs) -> Coord {
    Coord {
        x: coeffs.c * f64::sin(coeffs.a * (p.x + p.y)) - coeffs.d * f64::cos(coeffs.b * (p.y - p.x)),
        y: coeffs.d * f64::sin(coeffs.a * (p.x - p.y)) - coeffs.c * f64::cos(coeffs.b * (p.y + p.x)),
    }
}

pub fn get_functions() -> Vec<(&'static str, fn(&Coord, &Coeffs) -> Coord)> {
    vec![
        ("Clifford", clifford_attractor),
        ("Peter de Jong", peter_de_jong_attractor),
        ("Experiemental", experimental),
    ]
}

pub fn bind_1<'a, T, U, V, F>(function: &'a F, u: &'a U) -> impl Fn(&T) -> V + 'a 
    where F: Fn(&T, &U) -> V {
    move |t| function(t,u)
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