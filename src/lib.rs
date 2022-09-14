use rand;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

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

    fn create_random_point() -> Self {
        let x = rand::random::<i8>() as i64;
        let y = rand::random::<i8>() as i64;
        Point::new(x, y)
    }

    fn create_random_points(n: usize) -> Vec<Self> {
        (0..n).map(|_| Point::create_random_point()).collect()
    }

    pub fn nearest_point(&self, p1: Point, p2: Point) -> Point {
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
pub struct Cab {
    id: usize,
    location: Point,
    destination: Option<Point>,
}

impl Cab {
    pub fn new(id: usize, location: Point) -> Self {
        Cab {
            id,
            location,
            destination: None,
        }
    }

    fn update_destination(&mut self, destination: Point) {
        self.destination = Some(destination);
    }

    pub fn get_location(&self) -> Point {
        self.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    id: i64,
    location: Point,
    destination: Point,
}

impl Person {
    pub fn new(id: i64, location: Point, destination: Point) -> Self {
        Person {
            id,
            location,
            destination,
        }
    }

    pub fn request_cab(&self, fleet: &mut Fleet) -> Result<Cab, String> {
        match fleet.add_person(self.clone()) {
            None => Err("No cabs available right now".to_string()),
            Some(c) => Ok(c),
        }
    }

    pub fn remove_cab(&self, fleet: &mut Fleet) -> Result<(), String> {
        fleet.remove_person(self)
    }

    pub fn get_id(&self) -> i64 {
        self.id.clone()
    }

    pub fn get_location(&self) -> Point {
        self.location.clone()
    }

    pub fn get_destination(&self) -> Point {
        self.destination.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Fleet(HashMap<Cab, Option<Person>>);

impl Fleet {
    pub fn new(n: usize) -> Self {
        let mut hmap: HashMap<Cab, Option<Person>> = HashMap::with_capacity(n);
        let points: Vec<Point> = Point::create_random_points(n);

        let cabs: Vec<Cab> = points
            .into_iter()
            .enumerate()
            .map(|x| Cab::new(x.0, x.1))
            .collect();

        cabs.into_iter().for_each(|x| {
            let _ = hmap.entry(x).or_default();
            ()
        });

        Fleet(hmap)
    }

    pub fn get_map_clone(&self) -> HashMap<Cab, Option<Person>> {
        self.0.clone()
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

    fn nearest_of_2_cabs(&self, person: &Person, cab1: &Cab, cab2: &Cab) -> Cab {
        let d = person
            .get_location()
            .nearest_point(cab1.get_location(), cab2.get_location());

        if d == cab1.location {
            cab1.clone()
        } else {
            cab2.clone()
        }
    }

    pub fn add_person(&mut self, p: Person) -> Option<Cab> {
        let hmap = self.clone().0;

        let nearest_cab_to_p = hmap
            .into_iter()
            .filter(|x| x.1.is_none())
            .reduce(|x, y| (self.nearest_of_2_cabs(&p, &x.0, &y.0), None));

        match nearest_cab_to_p {
            None => None,
            Some((c, _)) => {
                let update_cab = self.cab_to_some_person(c, p);
                Some(update_cab)
            }
        }
    }

    pub fn remove_person(&mut self, person: &Person) -> Result<(), String> {
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
