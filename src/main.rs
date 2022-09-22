mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use repository::mongodb_repos::MongoRepo;

use api::cab_api::{assign_person, create_cab, create_fleet, generate_fleet, get_cab, get_fleet};
use api::person_api::{cancel_cab, create_person, get_person, hello, request_cab};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount(
            "/person",
            routes![create_person, get_person, request_cab, cancel_cab],
        )
        .mount(
            "/cab",
            routes![
                create_cab,
                create_fleet,
                generate_fleet,
                get_fleet,
                get_cab,
                assign_person
            ],
        )
}
