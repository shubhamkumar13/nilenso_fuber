use crate::{
    models::cab_model::Cab, models::person_model::Person, repository::mongodb_repos::MongoRepo,
};

use mongodb::{
    bson::from_bson,
    bson::oid::ObjectId,
    results::{InsertManyResult, InsertOneResult},
};

use rocket::{http::Status, serde::json::Json, State};

#[get("/")]
pub fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from Fuber")))
}

#[post("/create", data = "<new_person>")]
pub fn create_person(
    db: &State<MongoRepo>,
    new_person: Json<Person>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Person::new(
        None,
        new_person.name.clone(),
        new_person.location.clone(),
        new_person.destination.clone(),
    );
    let person_detail = db.create_person(data);
    match person_detail {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/<path>")]
pub fn get_person(db: &State<MongoRepo>, path: String) -> Result<Json<Person>, Status> {
    if path.is_empty() {
        Err(Status::BadRequest)
    } else {
        match db.get_person(&path) {
            Ok(person) => Ok(Json(person)),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[get("/request_cab/<person_id>")]
pub fn assign_cab(db: &State<MongoRepo>, person_id: String) -> Result<Json<(Person, Cab)>, Status> {
    unimplemented!()
}
