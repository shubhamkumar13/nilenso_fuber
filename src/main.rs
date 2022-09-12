use rand::prelude::*;

fn main() {
    let mut cabs = Fleet::new(1000).0;
    let cab1 = &cabs[0];
    let cab2 = &cabs[1];
    println!("{:#?}. {:#?}", cab1, cab2);
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
    fn fromTuple(t: (i64, i64)) -> Self {
        Point { x: t.0, y: t.1 }
    }
    fn dist(&self, p: Point) -> f64 {
        let x_sq = (self.x - p.x) * (self.x - p.x);
        let y_sq = (self.y - p.y) * (self.y - p.y);
        ((x_sq + y_sq) as f64).sqrt()
    }
}

#[derive(Debug, Clone)]
struct Points(Vec<Point>);

impl Points {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();

        let mut x_coords: Vec<usize> = (0..n).collect();
        let mut y_coords: Vec<usize> = (0..n).collect();
        x_coords.shuffle(&mut rng);
        y_coords.shuffle(&mut rng);

        Points(
            x_coords
                .iter()
                .zip(y_coords.iter())
                .map(|(&a, &b)| Point::fromTuple((a as i64, b as i64)))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
struct Cab {
    id: usize,
    location: Point,
    state: Option<Box<Person>>,
    destination: Option<Point>,
}

impl Cab {
    fn new(id: usize, location: Point) -> Self {
        Cab {
            id,
            location,
            state: None,
            destination: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Fleet(Vec<Cab>);

impl Fleet {
    fn new(n: usize) -> Self {
        let points = Points::new(n).0;
        Fleet(
            points
                .iter()
                .enumerate()
                .map(|(i, ival)| Cab::new(i, ival.clone()))
                .collect::<Vec<Cab>>(),
        )
    }

    fn to_vec(self) -> Vec<Cab> {
        self.0
    }

    fn from_vec(v: Vec<Cab>) -> Fleet {
        Fleet(v)
    }
}

#[derive(Debug, Clone)]
struct Person {
    id: i64,
    location: Point,
    state: Option<Box<Cab>>,
    destination: Point,
}

impl Person {
    fn new(id: i64, location: Point, destination: Point) -> Self {
        Person {
            id,
            location,
            state: None,
            destination,
        }
    }
}
