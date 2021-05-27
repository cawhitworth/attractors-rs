use std::vec::Vec;


fn lerp_u8(a: u8, b: u8, amount: f64) -> u8 {
    ((a as f64) + (b as f64 - a as f64) * amount) as u8
}

pub fn lerp(c1: image::Rgb<u8>, c2: image::Rgb<u8>, amount: f64) -> image::Rgb<u8> {
    image::Rgb([
        lerp_u8(c1[0], c2[0], amount),
        lerp_u8(c1[1], c2[1], amount),
        lerp_u8(c1[2], c2[2], amount)
    ])
}

pub struct GradientPoint {
    point: f64,
    colour: image::Rgb<u8>
}

impl GradientPoint {
    pub fn new(point: f64, colour: image::Rgb<u8>) -> GradientPoint {
        GradientPoint { point, colour }
    }
}

pub struct Gradient {
   name: String,
   points: Vec<GradientPoint> 
}

impl Gradient {
    pub fn new(name: &str, points: Vec<GradientPoint>) -> Gradient {

        let mut g = Gradient {
            name: String::from(name),
            points
        };

        g.points.sort_by(|a,b| a.point.partial_cmp(&b.point).unwrap());

        g
    }

    pub fn colour_at(&self, point: f64) -> image::Rgb<u8> {

        for index in 0..self.points.len()-1 {
            let p1 = &self.points[index];
            let p2 = &self.points[index + 1];
            if p1.point <= point && point <= p2.point {
                let d = (point - p1.point) / (p2.point - p1.point);
                return lerp(p1.colour, p2.colour, d);
            }
        }

        image::Rgb([0,0,0])
    }
}