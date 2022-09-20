mod models;

#[macro_use]
extern crate rocket;
use repository::mongodb_repos::MongoRepo;

    let s = serde_json::to_string(&p);

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
}
