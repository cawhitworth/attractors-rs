
#[derive(Debug, Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64
}

impl Coord {
    pub fn new(x: f64, y:  f64) -> Coord {
        Coord { x, y }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Rect {
    pub bl: Coord,
    pub tr: Coord,
}

impl Rect {
    pub fn from_coords(left: f64, bottom: f64, right: f64, top: f64) -> Rect {
        Rect { 
            bl: Coord::new(left, bottom),
            tr: Coord::new(right, top)
        }
    }

    pub fn width(&self) -> f64 {
        self.tr.x - self.bl.x
    }

    pub fn height(&self) -> f64 {
        self.tr.y - self.bl.y
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} -> {} ]", self.bl, self.tr)
    }
}