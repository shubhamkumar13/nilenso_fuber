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
