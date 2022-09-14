use fuber::{Cab, Fleet, Person, Point};

#[test]
fn test_get_nearest_cab() {
    let mut fleet = Fleet::new(3);
    let person1 = Person::new(0, Point::new(0, 0), Point::new(100, 100));
    let cab1 = match person1.request_cab(&mut fleet) {
        Ok(cab) => cab,
        Err(s) => panic!("{}", s),
    };

    let element = fleet.get_map_clone().into_iter().reduce(|c1, c2| {
        let d = person1
            .get_location()
            .nearest_point(c1.0.get_location(), c2.0.get_location());
        if d == c1.0.get_location() {
            c1
        } else {
            c2
        }
    });

    match element {
        None => panic!("no element found"),
        Some((cab2, _)) => assert_eq!(cab1, cab2),
    }
}
