use serde::{Deserialize, Serialize};

// Point struct to abstract the nitty gritty stuff for locations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

// helper functions to generate new or random points even from tuples
// more helper functions can be added as and when the domain needs grow
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    pub fn from_tuple(t: (i64, i64)) -> Self {
        Point { x: t.0, y: t.1 }
    }
    pub fn dist(&self, p: &Point) -> f64 {
        let x_sq = (self.x - p.x) * (self.x - p.x);
        let y_sq = (self.y - p.y) * (self.y - p.y);
        ((x_sq + y_sq) as f64).sqrt()
    }

    pub fn create_random_point() -> Self {
        let x = rand::random::<i8>() as i64;
        let y = rand::random::<i8>() as i64;
        Point::new(x, y)
    }

    pub fn create_random_points(n: usize) -> Vec<Self> {
        (0..n).map(|_| Point::create_random_point()).collect()
    }

    // access the nearest point from p1 or p2
    // for eq. if p = (0, 0) and p1 = (1, 2), p2 = (3, 4)
    // p1 is nearer to p and the return value
    pub fn nearest_point(&self, p1: &Point, p2: &Point) -> Point {
        let d1 = self.dist(p1);
        let d2 = self.dist(p2);
        if d1 < d2 {
            p1.clone()
        } else if d1 > d2 {
            p2.clone()
        } else {
            p1.clone()
        }
    }
}
