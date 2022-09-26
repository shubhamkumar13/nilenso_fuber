pub mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use repository::mongodb_repos::MongoRepo;

use api::cab_api::{
    assign_person, create_cab, create_fleet, delete_cab, delete_fleet, generate_fleet, get_cab,
    get_fleet, update_cab, update_location,
};
use api::person_api::{
    create_person, delete_all_people, delete_person, get_person, hello, request_cab, unassign_cab,
    update_person,
};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/person/test", routes![delete_all_people])
        .mount(
            "/cab/test",
            routes![assign_person, generate_fleet, delete_fleet],
        )
        .mount(
            "/person",
            routes![
                create_person,
                get_person,
                request_cab,
                unassign_cab,
                update_person,
                delete_person
            ],
        )
        .mount(
            "/cab",
            routes![
                create_cab,
                create_fleet,
                get_fleet,
                get_cab,
                update_location,
                update_cab,
                delete_cab,
            ],
        )
}
