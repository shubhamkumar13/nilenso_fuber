use crate::{
    models::cab_model::Cab, models::person_model::Person, repository::mongodb_repos::MongoRepo,
};

use mongodb::{
    bson::oid::ObjectId,
    results::{InsertManyResult, InsertOneResult},
};

use rocket::{http::Status, serde::json::Json, State};

#[post("/create", data = "<new_cab>")]
pub fn create_cab(
    db: &State<MongoRepo>,
    new_cab: Json<Cab>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Cab::new(None, new_cab.location.clone());
    let cab_detail = db.create_cab(data);
    match cab_detail {
        Ok(cab) => Ok(Json(cab)),
        Err(_) => Err(Status::InternalServerError),
    }
}
