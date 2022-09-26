// [DEPRECATED TESTS]

use fuber::{generate_random_string, Cab, Fleet, Person, Point};

// test if the `request_cab` function is actually
// getting me the nearest cab by simulating using the same thing
// represented respectively in cab1 and cab2
#[ignore]
#[test]
fn test_get_nearest_cab() {
    let mut fleet = Fleet::new(3);
    let person1 = Person::new(
        0,
        "shubham".to_string(),
        Point::new(0, 0),
        Point::new(100, 100),
    );
    // cab1 is the cab generated by request_cab
    let cab1 = match person1.request_cab(&mut fleet) {
        Ok(cab) => cab,
        Err(s) => panic!("{}", s),
    };

    // element which contains cab2 is a simulated way to get the nearest cab
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

    // this match statement checks if cab is found and if cab1 and cab2 are same
    // if the assert fails or any panic! macros enabled the test fails
    match element {
        None => panic!("no element found"),
        Some((cab2, _)) => assert_eq!(cab1, cab2),
    }
}

// test to check if the location of the cab is the same as the destination
// of the person when fleet removes the person from the cab
// which should happen only when the person reaches the destination
#[ignore]
#[test]
fn test_remove_person_after_reaching_destination() {
    let mut fleet = Fleet::new(3);
    let person1 = Person::new(
        0,
        "shubham".to_string(),
        Point::new(0, 0),
        Point::new(100, 100),
    );

    let cab = match person1.request_cab(&mut fleet) {
        Ok(cab) => cab,
        Err(s) => panic!("{}", s),
    };

    match person1.end_cab_ride(&mut fleet) {
        Ok(updated_cab) => {
            assert_eq!(updated_cab.get_location(), person1.get_destination());
        }
        Err(s) => panic!("{}", s),
    }
}

// test that should panic because when a new person is trying to request cab
// it should throw an error because it is unable to find an empty cab
#[ignore]
#[test]
#[should_panic]
fn test_failure_when_all_cabs_occupied() {
    let mut fleet = Fleet::new(3);

    // simulate the occupied people
    let person_vec: Vec<Person> = {
        let loc_points: Vec<Point> = Point::create_random_points(3);
        let dest_points: Vec<Point> = Point::create_random_points(3);

        loc_points
            .into_iter()
            .zip(dest_points.into_iter())
            .enumerate()
            .map(|x| Person::new(x.0, generate_random_string(), x.1 .0, x.1 .1))
            .collect()
    };

    // add the respective people to their respective cabs
    let _ = {
        person_vec
            .iter()
            .filter_map(|p| match (*p).request_cab(&mut fleet) {
                Ok(cab) => Some(cab),
                Err(_) => None,
            })
            .collect::<Vec<Cab>>()
    };

    // when a new person is added the assertion always results in an error
    let new_person = Person::new(
        rand::random::<u8>() as usize,
        generate_random_string(),
        Point::new(0, 0),
        Point::new(100, 100),
    );

    assert!(new_person.request_cab(&mut fleet).is_ok())
}
