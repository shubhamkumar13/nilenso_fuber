use crate::{
    models::cab_model::Cab, models::point_model::Point, repository::mongodb_repos::MongoRepo,
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
    let data = Cab::new(new_cab.location.clone());
    match db.create_cab(data) {
        Ok(cab) => Ok(Json(cab)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/create/fleet", data = "<fleet>")]
pub fn create_fleet(
    db: &State<MongoRepo>,
    fleet: Json<Vec<Cab>>,
) -> Result<Json<InsertManyResult>, Status> {
    let data = fleet.into_inner();
    match db.create_fleet(data) {
        Ok(fleet) => Ok(Json(fleet)),
        Err(_) => Err(Status::InternalServerError),
    }
}

fn simulate_fleet(n: usize) -> Json<Vec<Cab>> {
    let points: Vec<Point> = Point::create_random_points(n);

    Json(points.into_iter().map(|x| Cab::new(x)).collect())
}

#[get("/fleet/<size>")]
pub fn generate_fleet(_db: &State<MongoRepo>, size: usize) -> Json<Vec<Cab>> {
    simulate_fleet(size)
}

#[get("/<cab_id>")]
pub fn get_cab(db: &State<MongoRepo>, cab_id: String) -> Result<Json<Cab>, Status> {
    if cab_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        match db.get_cab(&cab_id) {
            Ok(cab) => Ok(Json(cab)),
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[get("/fleet")]
pub fn get_fleet(db: &State<MongoRepo>) -> Result<Json<Vec<Cab>>, Status> {
    let cabs = db.get_fleet();
    match cabs {
        Ok(cabs) => Ok(Json(cabs)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/assign_person/<person_id>", data = "<cab>")]
pub fn assign_person(
    db: &State<MongoRepo>,
    person_id: String,
    cab: Json<Cab>,
) -> Result<Json<Cab>, Status> {
    if person_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        let mut cab = cab.into_inner().clone();
        let person = db
            .get_person(&person_id)
            .expect("Cannot find the person in the db");
        cab.update_destination(Some(person.destination));
        cab.update_person_id(ObjectId::parse_str(&person_id).ok());
        let cab_id = cab.id.and_then(|x| Some(x.to_hex())).unwrap();
        let update_result = db.assign_person(&cab_id, cab.clone());
        match update_result {
            Ok(update) => {
                if update.matched_count == 1 {
                    let updated_cab_info = db.get_cab(&cab_id);
                    match updated_cab_info {
                        Ok(cab) => Ok(Json(cab)),
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
