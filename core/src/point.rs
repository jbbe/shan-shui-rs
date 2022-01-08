
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

const EPSILON: f64 = 0.001;
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x - other.x < EPSILON && self.y - other.y < EPSILON
    }

    fn ne(&self, other: &Self) -> bool {
        self.x - other.x > EPSILON || self.y - other.y > EPSILON
    }
}
pub fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt(f64::powf(p1.x - p2.x, 2.) + f64::powf(p1.x - p2.x, 2.))
}