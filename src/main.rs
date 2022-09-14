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

    fn create_random_points(n: usize) -> Vec<Self> {
        (0..n).map(|_| Point::create_random_point()).collect()
    }

    fn nearest_point(&self, p1: Point, p2: Point) -> Point {
        let d1 = self.dist(&p1);
        let d2 = self.dist(&p2);
        if d1 < d2 {
            p1
        } else if d1 > d2 {
            p2
        } else {
            p1
        }
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

    fn get_location(&self) -> Point {
        self.location.clone()
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

    fn get_location(&self) -> Point {
        self.location.clone()
    }

    fn get_destination(&self) -> Point {
        self.destination.clone()
    }
}

#[derive(Debug, Clone)]
struct Fleet(HashMap<Cab, Option<Person>>);

impl Fleet {
    fn new(n: usize) -> Self {
        let mut hmap: HashMap<Cab, Option<Person>> = HashMap::with_capacity(n);
        let points: Vec<Point> = Point::create_random_points(n);

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

    fn cab_to_none(&mut self, c: Cab) {
        let _ = self.0.insert(c, None);
        ()
    }

    fn cab_to_some_person(&mut self, cab: Cab, p: Person) -> Cab {
        let mut new_cab = cab.clone();
        new_cab.update_destination(p.destination.clone());
        let _ = self.0.remove_entry(&cab);
        let _ = self.0.insert(new_cab.clone(), Some(p));
        new_cab.clone()
    }

        let d = person
            .get_location()
            .nearest_point(cab1.get_location(), cab2.get_location());

        if d == cab1.location {
            cab1.clone()
        } else {
            cab2.clone()
        }
    }

    fn add_person(&mut self, p: Person) -> Option<Cab> {
        let hmap = self.clone().0;

        let nearest_cab_to_p = hmap
            .into_iter()
            .filter(|x| x.1.is_none())
            .reduce(|x, y| (self.nearest_cab(&p, &x.0, &y.0), None));

        match nearest_cab_to_p {
            None => None,
            Some((c, _)) => {
                let update_cab = self.cab_to_some_person(c, p);
                Some(update_cab)
            }
        }
    }

    fn remove_person(&mut self, person: &Person) -> Result<(), String> {
        let hmap = self.clone().0;

        let field = hmap.into_iter().fold(None, |_, x| match &x.1 {
            None => None,
            Some(p) => {
                if *p == *person {
                    Some(x)
                } else {
                    None
                }
            }
        });

        match field {
            None => Err(format!(
                "Something went wrong and couldn't find the Person with id : {} in our fleet",
                person.get_id()
            )),

            Some((c, _)) => {
                self.cab_to_none(c);
                Ok(())
            }
        }
    }
}
