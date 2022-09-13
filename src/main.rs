use rand;
use std::collections::HashMap;

fn main() {
    let mut fleet = Fleet::new(3);
    let person1 = Person::new(0, Point::new(0, 0), Point::new(100, 100));
    if let Ok(cab) = person1.request_cab(&mut fleet) {
        println!("{:#?}", cab);
    }
    println!("{:#?}", fleet);
}

fn create_random_points(n: usize) -> Vec<Point> {
    (0..n).map(|_| Point::create_random_point()).collect()
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

    fn create_random_point() -> Self {
        let x = rand::random::<i8>() as i64;
        let y = rand::random::<i8>() as i64;
        Point::new(x, y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn update_destination(&mut self, destination: Point) {
        self.destination = Some(destination);
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

    fn request_cab(&self, fleet: &mut Fleet) -> Result<Cab, String> {
        match fleet.add_person(self.clone()) {
            None => Err("No cabs available right now".to_string()),
            Some(c) => Ok(c),
        }
    }
}

#[derive(Debug, Clone)]
struct Fleet(HashMap<Cab, Option<Person>>);

impl Fleet {
    fn new(n: usize) -> Self {
        let mut hmap: HashMap<Cab, Option<Person>> = HashMap::with_capacity(n);
        let points: Vec<Point> = create_random_points(n);

        let cabs: Vec<Cab> = points
            .into_iter()
            .enumerate()
            .map(|x| Cab::new(x.0, x.1))
            .collect();

        cabs.into_iter().for_each(|x| match hmap.insert(x, None) {
            _ => (),
        });

        Fleet(hmap)
    }

    fn add_person(&mut self, p: Person) -> Option<Cab> {
        let hmap = self.clone().0;

        let nearest_cab_to_p = hmap.into_iter().filter(|x| x.1.is_none()).reduce(|x, y| {
            let d1: f64 = x.0.location.dist(&p.location);
            let d2: f64 = y.0.location.dist(&p.location);

            if d1 < d2 {
                x
            } else if d1 > d2 {
                y
            } else {
                x
            }
        });

        match nearest_cab_to_p {
            None => None,
            Some((c, _)) => {
                let mut cab = c.clone();
                cab.update_destination(p.destination.clone());
                let _ = self.0.remove_entry(&c);
                let _ = self.0.insert(cab.clone(), Some(p));
                Some(cab.clone())
            }
        }
    }
}
