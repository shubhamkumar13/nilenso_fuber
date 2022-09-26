use fuber::api::cab_api;
use fuber::api::person_api;
use fuber::generate_random_string;
use fuber::models::person_model::Person;
use fuber::models::point_model::Point;
use fuber::repository::mongodb_repos::MongoRepo;
use rocket::serde::json::Json;
use rocket::State;

#[test]
fn test_get_nearest_cab() {
    // create a db client
    let db = MongoRepo::init();
    let rocket = rocket::build().manage(db);
    let state = State::get(&rocket).expect("cannot get the state");

    // check if fleet is empty or not
    match cab_api::get_fleet(state) {
        Ok(Json(v)) => {
            if !v.is_empty() {
                // delete all the stuff from the db
                let _ = cab_api::delete_fleet(state).expect("cannot delete fleet");
                let _ = person_api::delete_all_people(state).expect("cannot delete all people");
            }
        }
        Err(_) => panic!("Cannot get a fleet"),
    }

    // generate a fleet
    let fleet = cab_api::generate_fleet(state, 3);
    // insert fleet to db
    let Json(fleet_id_vec) =
        cab_api::create_fleet(state, fleet).expect("cannot insert fleet into the db");
    let fleet = cab_api::get_fleet(state).expect("cannot get fleet");

    // generate a person
    let person = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    // insert person to db
    let Json(person_id) = person_api::create_person(state, Json(person.clone()))
        .expect("cannot insert the person into db");
    let person = person_api::get_person(state, person_id.clone())
        .expect("cannot get the person data after insertion");

    // use the api to get a cab nearest to the person
    let Json((_, api_cab)) = person_api::request_cab(state, person_id.clone())
        .expect("cannot find the nearest cab to the person requesting the cab");

    // manually find out the nearest cab to the person
    let mut manual_cab = fleet
        .into_inner()
        .into_iter()
        .reduce(|c1, c2| person.nearest_cab(&c1, &c2))
        .expect("cannot find the nearest cab to the person, manually");
    manual_cab.update_destination(Some(person.location.clone()));
    manual_cab.update_person_id(person.id);

    // assert they are equal
    assert_eq!(api_cab, manual_cab)
}

#[test]
#[should_panic]
fn test_assign_cab_panic() {
    // create a db client
    let db = MongoRepo::init();
    let rocket = rocket::build().manage(db);
    let state = State::get(&rocket).expect("cannot get the state");

    // check if fleet is empty or not
    match cab_api::get_fleet(state) {
        Ok(Json(v)) => {
            if !v.is_empty() {
                // delete all the stuff from the db
                let _ = cab_api::delete_fleet(state).expect("cannot delete fleet");
                let _ = person_api::delete_all_people(state).expect("cannot delete all people");
            }
        }
        Err(_) => panic!("Cannot get a fleet"),
    }

    // generate a fleet
    let fleet = cab_api::generate_fleet(state, 3);
    // insert fleet to db
    let Json(fleet_id_vec) =
        cab_api::create_fleet(state, fleet).expect("cannot insert fleet into the db");
    let fleet = cab_api::get_fleet(state).expect("cannot get fleet");

    // generate a person1
    let person1 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    // insert person1 to db
    let Json(person_id_1) = person_api::create_person(state, Json(person1.clone()))
        .expect("cannot insert the person1 into db");

    // use the api to get a cab nearest to the person
    let Json((_, api_cab)) = person_api::request_cab(state, person_id_1.clone())
        .expect("cannot find the nearest cab to the person requesting the cab");

    // generate a person2
    let person2 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    // insert person2 to db
    let Json(person_id_2) = person_api::create_person(state, Json(person2.clone()))
        .expect("cannot insert the person2 into db");

    // try to assign the cab with another person
    let status = cab_api::assign_person(state, person_id_2, Json(api_cab.clone()));

    // this assertion should panic
    assert!(status.is_ok())
}

#[test]
#[should_panic]
fn test_request_cab_panic_when_fleet_occupied() {
    // create a db client
    let db = MongoRepo::init();
    let rocket = rocket::build().manage(db);
    let state = State::get(&rocket).expect("cannot get the state");

    // check if fleet is empty or not
    match cab_api::get_fleet(state) {
        Ok(Json(v)) => {
            if !v.is_empty() {
                // delete all the stuff from the db
                let _ = cab_api::delete_fleet(state).expect("cannot delete fleet");
                let _ = person_api::delete_all_people(state).expect("cannot delete all people");
            }
        }
        Err(_) => panic!("Cannot get a fleet"),
    }

    // generate a fleet
    let fleet = cab_api::generate_fleet(state, 3);
    // insert fleet to db
    let Json(fleet_id_vec) =
        cab_api::create_fleet(state, fleet).expect("cannot insert fleet into the db");
    let fleet = cab_api::get_fleet(state).expect("cannot get fleet");

    // generate person1, person2 and person3 to occupy a fleet of 3
    let person1 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    let person2 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    let person3 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );

    // insert all the persons
    let Json(person_id_1) = person_api::create_person(state, Json(person1.clone()))
        .expect("cannot insert the person1 into db");
    let Json(person_id_2) = person_api::create_person(state, Json(person2.clone()))
        .expect("cannot insert the person2 into db");
    let Json(person_id_3) = person_api::create_person(state, Json(person3.clone()))
        .expect("cannot insert the person3 into db");

    // all persons request cab
    let Json((person_1, cab_1)) =
        person_api::request_cab(state, person_id_1).expect("person1 cab request failed");
    let Json((person_2, cab_2)) =
        person_api::request_cab(state, person_id_2).expect("person1 cab request failed");
    let Json((person_3, cab_3)) =
        person_api::request_cab(state, person_id_3).expect("person1 cab request failed");

    // create the person4 which will be rejected when requested for a cab
    let person4 = Person::new(
        None,
        generate_random_string(),
        Point::create_random_point(),
        Point::create_random_point(),
    );
    // insert the person4
    let Json(person_id_4) =
        person_api::create_person(state, Json(person4)).expect("cannot insert person4 into db");

    let res = person_api::request_cab(state, person_id_4);

    assert!(res.is_ok())
}
