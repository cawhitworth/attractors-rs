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
