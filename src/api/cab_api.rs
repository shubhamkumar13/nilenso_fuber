use crate::{
    models::cab_model::Cab, models::point_model::Point, repository::mongodb_repos::MongoRepo,
};

use mongodb::{
    bson::oid::ObjectId,
    results::{InsertManyResult, InsertOneResult},
};

use rocket::{delete, get, http::Status, post, put, serde::json::Json, State};

#[post("/create", data = "<new_cab>")]
pub fn create_cab(db: &State<MongoRepo>, new_cab: Json<Cab>) -> Result<Json<String>, Status> {
    let data = Cab::new(new_cab.location.clone());

    let cab_detail = db.create_cab(data);
    match cab_detail {
        Ok(cab) => match cab.inserted_id.as_object_id() {
            Some(obj_id) => Ok(Json(obj_id.to_hex())),
            None => Err(Status::ExpectationFailed),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/create/fleet", data = "<fleet>")]
pub fn create_fleet(
    db: &State<MongoRepo>,
    fleet: Json<Vec<Cab>>,
) -> Result<Json<Vec<Option<String>>>, Status> {
    let data = fleet.into_inner();
    match db.create_fleet(data) {
        Ok(fleet) => {
            let bson = fleet.inserted_ids.values().clone();
            let vec_obj_id = bson
                .into_iter()
                .map(|x| match x.as_object_id() {
                    Some(s) => Some(s.to_hex()),
                    None => None,
                })
                .collect::<Vec<Option<String>>>();
            Ok(Json(vec_obj_id))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn simulate_fleet(n: usize) -> Json<Vec<Cab>> {
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

fn is_free(cab: Json<Cab>) -> Result<(), Status> {
    match cab.into_inner().person_id {
        None => Ok(()),
        Some(_) => Err(Status::Forbidden),
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
        match is_free(cab.clone()) {
            Ok(_) => {
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
            Err(e) => Err(e),
        }
    }
}

#[put("/update_location/<cab_id>", data = "<point>")]
pub fn update_location(
    db: &State<MongoRepo>,
    cab_id: String,
    point: Json<Option<Point>>,
) -> Result<Json<Cab>, Status> {
    if cab_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        match get_cab(db, cab_id) {
            Ok(Json(mut cab)) => match point.into_inner() {
                Some(p) => {
                    cab.update_location(p);
                    match db.update_cab(cab.clone()) {
                        Ok(update) => {
                            if update.matched_count == 1 {
                                Ok(Json(cab))
                            } else {
                                Err(Status::NotFound)
                            }
                        }
                        Err(_) => Err(Status::InternalServerError),
                    }
                }
                None => Err(Status::BadRequest),
            },
            Err(_) => Err(Status::BadRequest),
        }
    }
}

#[put("/update_cab/<cab_id>", data = "<new_cab_info>")]
pub fn update_cab(
    db: &State<MongoRepo>,
    cab_id: String,
    new_cab_info: Json<Cab>,
) -> Result<Json<Cab>, Status> {
    if cab_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        let new_cab = Cab {
            id: ObjectId::parse_str(cab_id).ok(),
            location: new_cab_info.location.clone(),
            destination: new_cab_info.destination.clone(),
            person_id: new_cab_info.person_id,
        };
        match db.update_cab(new_cab.clone()) {
            Ok(update) => {
                if update.matched_count == 1 {
                    Ok(Json(new_cab))
                } else {
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[delete("/delete_cab/<cab_id>")]
pub fn delete_cab(db: &State<MongoRepo>, cab_id: String) -> Result<Json<String>, Status> {
    if cab_id.is_empty() {
        Err(Status::BadRequest)
    } else {
        match db.delete_cab(&cab_id) {
            Ok(res) => {
                if res.deleted_count == 1 {
                    Ok(Json("Cab successfully deleted!".into()))
                } else {
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        }
    }
}

#[delete("/delete_fleet")]
pub fn delete_fleet(db: &State<MongoRepo>) -> Result<Json<String>, Status> {
    match db.delete_fleet() {
        Ok(res) => {
            if res.deleted_count >= 1 {
                Ok(Json("Fleet successfully deleted!".into()))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
