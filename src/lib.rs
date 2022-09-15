use rand;
use std::collections::HashMap;

// A TL;DR for non-rust users what #[derive(Debug, CLone, PartialEq ...)] does :

//  So anything inside a #[derive(..)] is proc macro which generates code for
//  the struct, enum or function it is placed just above
//  so it essentially adds super powers using code generation
//  although I understand it's usage it's internals are a bit complex for me right now

// Q : Ok, so what are the super powers?
// A : Let's look at the stuff I've used also these super powers are
//     implemented as traits which are an equivalent of typeclasses

// Debug and Clone :
// Debug helps in displaying the struct or enum as a string using println! macro
// Clone helps in making any struct or enum clonable which means it's instances
//  in the memory can be cloned to another place without having an effect on the
//  the current instance of the struct or enum using the `clone()` function.
// Reference for Debug : https://doc.rust-lang.org/std/fmt/trait.Debug.html
//               Clone : https://doc.rust-lang.org/std/clone/trait.Clone.html

// In general this makes copying memory and debugging the structural
//  changes make it easy for an average developers like me :) to program.

// PartialEq and Eq :
// These 2 are a bit complex and comes under the purview of total and
//  partial relations.

// If a struct or enum derives `Eq` it means that there is a way in which I am
//  enforcing that reflexivity, symmetry and transivity hold. As there is no
//  way for the compiler to check otherwise.
// So if instances a and b derives Eq, a == b and a != b are truly inverse.
// Reference for more : https://doc.rust-lang.org/std/cmp/trait.Eq.html

// While PartialEq only enforces symmetry and transitivity and not reflexivity.
// Reference for more : https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

// Hash : I have very little idea on it right now but if one wants a struct or
//        enum to be hashable in a key one needs to derive Hash trait.
// Reference : https://doc.rust-lang.org/std/hash/trait.Hash.html

// Point struct to abstract the nitty gritty stuff for locations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
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

    fn create_random_point() -> Self {
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

// Struct Cab to encapsulate what info a cab should be have
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cab {
    id: usize,
    location: Point,
    destination: Option<Point>,
}

// helper functions picking up things that can be accessed outside of the
// library using the pub keyword, updating destination is not public
// because we don't want the Cab instance outside library to change it.
impl Cab {
    pub fn new(id: usize, location: Point) -> Self {
        Cab {
            id,
            location,
            destination: None,
        }
    }

    fn update_destination(&mut self, destination: Option<Point>) {
        self.destination = destination;
    }

    pub fn get_location(&self) -> Point {
        self.location.clone()
    }

    fn update_location(&mut self, location: Point) {
        self.location = location
    }
}

// Very similar struct for Person and Cab which can be generalized further
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    id: usize,
    location: Point,
    destination: Point,
}

// all the methods are public because we want the Person instance
// to access all it's methods and not Fleet or Cab's
impl Person {
    pub fn new(id: usize, location: Point, destination: Point) -> Self {
        Person {
            id,
            location,
            destination,
        }
    }

    // the return value is either a Cab instance or a error message
    pub fn request_cab(&self, fleet: &mut Fleet) -> Result<Cab, String> {
        match fleet.add_person(self.clone()) {
            None => Err("No cabs available right now".to_string()),
            Some(c) => Ok(c),
        }
    }

    // this also has a Result type return value just to make the error message
    // clearer, it returns unit if everything goes right
    pub fn end_cab_ride(&self, fleet: &mut Fleet) -> Result<Cab, String> {
        fleet.remove_person(self)
    }

    pub fn get_id(&self) -> usize {
        self.id.clone()
    }

    pub fn get_location(&self) -> Point {
        self.location.clone()
    }

    pub fn get_destination(&self) -> Point {
        self.destination.clone()
    }
}

// This struct essentially holds the Map of (Cab -> Person)
// which is None when nothing is assigned
// This is the only mutable entity which is mutated whenever there is a request
// from the person entity
#[derive(Debug, Clone)]
pub struct Fleet(HashMap<Cab, Option<Person>>);

// Ideally only `Fleet::new()` would be the method that would be public
// because we don't want anyone using the methods of Fleet every request or
// remove method in the Person instances requires a fleet instance
// which means we do need to create it but not access any instance of it
// Right now pub is used only for testing otherwise it's not visible in test.rs
impl Fleet {
    // random points help to populate the Fleet with cab instances
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

    // get the clone of the map at any instant
    pub fn get_map_clone(&self) -> HashMap<Cab, Option<Person>> {
        self.0.clone()
    }

    fn get_tuple_by_person(&self, p: &Person) -> Result<(Cab, Option<Person>), String> {
        let hmap = self.0.clone().into_iter();

        for (k, v) in hmap {
            match v.clone() {
                None => continue,
                Some(person) => {
                    if person == *p {
                        return Ok((k, v));
                    }
                }
            }
        }

        return Err(format!("Cannot find person {} in the fleet", p.get_id()));
    }

    fn get_tuple_by_cab(&self, c: &Cab) -> Result<(Cab, Option<Person>), String> {
        unimplemented!()
    }

    // deallocate a person instance from a cab instance in the fleet and set
    // the new location of the cab which should be the destination where the
    // person is deallocated
    fn cab_to_none(&mut self, cab: Cab, new_location: Point) -> Cab {
        let mut new_cab = cab.clone();
        new_cab.update_location(new_location);
        new_cab.update_destination(None);
        let _ = self.0.remove_entry(&cab);
        let _ = self.0.insert(new_cab.clone(), None);
        new_cab
    }

    // allocate a person instance to a cab instance in the fleet
    fn cab_to_some_person(&mut self, cab: Cab, p: Person) -> Cab {
        let mut new_cab = cab.clone();
        new_cab.update_destination(Some(p.location.clone()));
        let _ = self.0.remove_entry(&cab);
        let _ = self.0.insert(new_cab.clone(), Some(p));
        new_cab.clone()
    }

    // calculate the nearest cab out of 2 cabs from a person
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

    // takes in the request to add a person to the fleet if possible
    // Returns an option type, where if a person is assigned to a cab
    // a clone of the cab instance is returned and if a cab isn't found it
    // returns a None type
    pub fn add_person(&mut self, p: Person) -> Option<Cab> {
        let hmap = self.clone().0;

        let nearest_cab_to_p = hmap
            .into_iter()
            .filter(|x| x.1.is_none())
            .reduce(|x, y| (self.nearest_of_2_cabs(&p, &x.0, &y.0), None));

        match nearest_cab_to_p {
            None => None,
            Some((c, _)) => {
                let updated_cab = self.cab_to_some_person(c, p);
                Some(updated_cab)
            }
        }
    }

    // takes in the request to remove a person from the fleet if possible
    // Returns a Result type, where if a person is unassigned to a cab
    // a Cab instance is returned with updated location which is
    // the current person's destination and if a cab isn't unassigned because
    // of some unknowable reason a error message is returned.
    pub fn remove_person(&mut self, person: &Person) -> Result<Cab, String> {
        // get the field which contains the cab -> person mapping
        // which needs to be removed
        let field = self.get_tuple_by_person(person);

        // most probably because the person wasn't assigned at first so it
        // cannot be removed
        match field {
            Err(s) => Err(format!(
                "{}\nThis error happened inside [Fleet::remove_person(..)] -> field pattern match -> Err arm\n",
                s
            )),

            Ok((c, p)) => match p {
                None => Err(format!(
                    "Expected Some(Person) found None\nThis error happened inside [Fleet::remove_person(..)] -> field pattern match -> Ok (..) arm\n"
                )),
                Some(p) => Ok(self.cab_to_none(c, p.get_destination())),
            },
        }
    }
}
