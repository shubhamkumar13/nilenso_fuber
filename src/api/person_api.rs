use crate::{
    models::cab_model::Cab, models::person_model::Person, repository::mongodb_repos::MongoRepo,
};

use mongodb::{
    bson::from_bson,
    bson::{self, oid::ObjectId, DeserializerOptions},
    results::{InsertManyResult, InsertOneResult},
};

use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};

#[get("/")]
pub fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from Fuber")))
}

#[post("/create", data = "<new_person>")]
pub fn create_person(
    db: &State<MongoRepo>,
    new_person: Json<Person>,
) -> Result<Json<String>, Status> {
    let data = Person::new(
        None,
        new_person.name.clone(),
        new_person.location.clone(),
        new_person.destination.clone(),
    );
    let person_detail = db.create_person(data);
    match person_detail {
        Ok(person) => match person.inserted_id.as_object_id() {
            Some(obj_id) => Ok(Json(obj_id.to_hex())),
            None => Err(Status::ExpectationFailed),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<person_id>")]
pub fn get_person(db: &State<MongoRepo>, person_id: String) -> Result<Json<Person>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        match db.get_person(&person_id) {
            Ok(person) => Ok(Json(person)),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

// to check if cab is free
fn is_free(cab: Cab) -> Result<(), Status> {
    match cab.person_id {
        None => Ok(()),
        Some(_) => Err(Status::Forbidden),
    }
}

#[get("/request_cab/<person_id>")]
pub fn request_cab(
    db: &State<MongoRepo>,
    person_id: String,
) -> Result<Json<(Person, Cab)>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        // get person using person_id
        let person = db
            .get_person(&person_id)
            .expect("Cannot get a person's detail");
        // get fleet
        let fleet = db.get_fleet().expect("Unable to get the fleet");
        // find the nearest cab in the fleet from the person
        let mut nearest_cab = fleet
            .into_iter()
            .filter(|x| is_free((*x).clone()).is_ok())
            .reduce(|c1, c2| person.nearest_cab(&c1, &c2))
            .and_then(|x| Some(x.clone()))
            .expect("Unable to get the nearest cab");
        // check if the cab is assigned or not

        // update cab destination and person_id
        nearest_cab.update_destination(Some(person.location.clone()));
        nearest_cab.update_person_id(ObjectId::parse_str(person_id).ok());
        // update cab by using `assign_person`
        let cab_id = match nearest_cab.id {
            Some(obj_id) => obj_id.to_hex(),
            None => panic!("cannot get the cab id"),
        };
        let update_result = db.assign_person(&cab_id, nearest_cab.clone());
        // return result as person and cab tuple
        match update_result {
            Ok(update) => {
                if update.matched_count == 1 {
                    let updated_cab_info = db.get_cab(&cab_id);
                    match updated_cab_info {
                        Ok(cab) => Ok(Json((person, cab))),
                        Err(_) => Err(Status::InternalServerError),
                    }
                } else {
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[get("/unassign_cab/<person_id>")]
pub fn unassign_cab(
    db: &State<MongoRepo>,
    person_id: String,
) -> Result<Json<(Person, Cab)>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        // get person using person_id
        let person = db
            .get_person(&person_id)
            .expect("Cannot get a person's detail");
        // get fleet
        let fleet = db.get_fleet().expect("Unable to get the fleet");
        // find the assigned cab for the person in the fleet
        let assigned_cab = fleet.into_iter().fold(None, |acc, x| match x.person_id {
            Some(obj_id) => {
                if obj_id.to_hex() == person_id {
                    Some(x)
                } else {
                    None
                }
            }
            None => None,
        });
        match assigned_cab {
            None => Err(Status::Forbidden),
            Some(mut assigned_cab) => {
                // nullify cab destination and person_id
                assigned_cab.update_destination(None);
                assigned_cab.update_person_id(None);
                assigned_cab.update_location(person.destination.clone());
                // update cab by using `assign_person`
                let cab_id = match assigned_cab.id {
                    Some(obj_id) => obj_id.to_hex(),
                    None => panic!("cannot get the cab id"),
                };
                let update_result = db.unassign_person(&cab_id, assigned_cab.clone());
                // return result as person and cab tuple
                match update_result {
                    Ok(update) => {
                        if update.matched_count == 1 {
                            let updated_cab_info = db.get_cab(&cab_id);
                            match updated_cab_info {
                                Ok(cab) => Ok(Json((person, cab))),
                                Err(_) => Err(Status::InternalServerError),
                            }
                        } else {
                            Err(Status::NotFound)
                        }
                    }
                    Err(_) => Err(Status::InternalServerError),
                }
            }
        }
    }
}

#[put("/update_person/<person_id>", data = "<person_data>")]
pub fn update_person(
    db: &State<MongoRepo>,
    person_id: String,
    person_data: Json<Person>,
) -> Result<Json<Person>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        let new_person = Person {
            id: ObjectId::parse_str(person_id).ok(),
            name: person_data.name.clone(),
            location: person_data.location.clone(),
            destination: person_data.destination.clone(),
        };
        match db.update_person(new_person.clone()) {
            Ok(update) => {
                if update.matched_count == 1 {
                    Ok(Json(new_person))
                } else {
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[delete("/delete_person/<person_id>")]
pub fn delete_person(db: &State<MongoRepo>, person_id: String) -> Result<Json<String>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        match db.delete_person(&person_id) {
            Ok(res) => {
                if res.deleted_count == 1 {
                    Ok(Json("Person successfully deleted!".into()))
                } else {
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[delete("/delete_all_people")]
pub fn delete_all_people(db: &State<MongoRepo>) -> Result<Json<String>, Status> {
    match db.delete_all_people() {
        Ok(res) => {
            if res.deleted_count >= 1 {
                Ok(Json("Every person successfully deleted!".into()))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
