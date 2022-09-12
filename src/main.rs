use rand::prelude::*;

const INF: f64 = f64::INFINITY;

fn main() {
    let mut fleet = Fleet::new(10);
    // // let mut cab1 = &fleet[0];
    // // let mut cab2 = &fleet[1];
    // println!("{:#?}. {:#?}", cab1, cab2);
    println!("{:#?}", fleet);
}

fn create_random_points(n: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();

    let mut x_coords: Vec<usize> = (0..n).collect();
    let mut y_coords: Vec<usize> = (0..n).collect();
    x_coords.shuffle(&mut rng);
    y_coords.shuffle(&mut rng);

    x_coords
        .iter()
        .zip(y_coords.iter())
        .map(|(&a, &b)| Point::fromTuple((a as i64, b as i64)))
        .collect()
}

enum Message {
    Success,
    Failure,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn dist(&self, p: &Point) -> f64 {
        let x_sq = (self.x - p.x) * (self.x - p.x);
        let y_sq = (self.y - p.y) * (self.y - p.y);
        ((x_sq + y_sq) as f64).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cab {
    id: usize,
    location: Point,
    destination: Option<Point>,
}

impl Cab {
    fn new(id: usize, location: Point) -> Self {
        Cab {
            id,
            location,
            destination: None,
        }
    }

    fn is_assigned(&self, fleet: Fleet) -> Result<Message, String> {
        fleet.is_cab_assigned(*self)
    }
}

#[derive(Debug, Clone)]
struct Person {
    id: i64,
    location: Point,
    destination: Point,
}

impl Person {
    fn new(id: i64, location: Point, destination: Point) -> Self {
        Person {
            id,
            location,
            destination,
        }
    }

    fn request_cab(&self, fleet: &mut Fleet) -> Message {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct Entity(Cab, Option<Person>);

impl Entity {
    fn new(cab: Cab) -> Self {
        Entity(cab, None)
    }

    fn assign_person(&mut self, person: Person) -> Message {
        match self.1 {
            None => {
                self.1 = Some(person);
                Message::Success
            }
            Some(_) => Message::Failure,
        }
    }

    fn is_cab_assigned(&self, cab: Cab) -> Message {
        if self.0.id == cab.id {
            Message::Success
        } else {
            Message::Failure
        }
    }
}

#[derive(Debug, Clone)]
struct Fleet(Vec<Entity>);

impl Fleet {
    fn new(n: usize) -> Self {
        let points = create_random_points(n);
        Fleet(
            points
                .iter()
                .enumerate()
                .map(|(i, ival)| Entity(Cab::new(i, ival.clone()), None))
                .collect(),
        )
    }

    fn is_cab_assigned(&self, cab: Cab) -> Result<Message, String> {
        if let Some(e) = self.0.iter().find(|Entity(x, _)| *x == cab) {
            Ok((*e).is_cab_assigned(cab))
        } else {
            Err("The cab doesn't exist".to_string())
        }
    }

    fn assign_person(&mut self, person : Person) 
}
