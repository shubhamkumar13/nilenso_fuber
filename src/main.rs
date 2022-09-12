use rand::prelude::*;

#[derive(Debug, Clone)]
enum Entity {
    Cab,
    Person,
}

#[derive(Debug, Clone)]
enum State {
    Assigned(Entity),
    Unassigned,
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
    state: State,
    distance_to_person: Option<usize>,
    distance_to_destination: Option<usize>,
}

impl Cab {
    fn new(id: usize, location: Point, state: State) -> Self {
        Cab {
            id,
            location,
            state,
            distance_to_person: None,
            distance_to_destination: None,
        }
    }
    fn assign_person(&mut self, person: Person) -> () {
        unimplemented!()
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
                .map(|(i, ival)| Cab::new(i, ival.clone(), State::Unassigned))
                .collect::<Vec<Cab>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct Person {
    id: i64,
    location: Point,
    state: State,
    destination: Point,
}

impl Person {
    fn new(id: i64, location: Point, state: State, destination: Point) -> Self {
        Person {
            id,
            location,
            state,
            destination,
        }
    }
    fn assign_cab(&mut self, cab: Cab) {
        unimplemented!()
    }
}
