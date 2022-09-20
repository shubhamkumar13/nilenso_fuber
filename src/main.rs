mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use repository::mongodb_repos::MongoRepo;

use api::cab_api::create_cab;
use api::person_api::{create_person, get_person, hello};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/person", routes![create_person, get_person])
        .mount("/cab", routes![create_cab])
}
