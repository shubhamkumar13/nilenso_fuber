#[macro_use]
extern crate rocket;
use fuber::{Person, Point};
use rand::random;
use rocket::serde::json::Json;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![world])
        .mount("/person", routes![create_person])
        .launch()
        .await?;
    Ok(())
}

#[get("/")]
fn world() -> String {
    "Hello, world".to_string()
}

#[get("/create_person")]
fn create_person() -> String {
    let p = Person::new(
        random::<u8>() as usize,
        Point::create_random_point(),
        Point::create_random_point(),
    );

    let s = serde_json::to_string(&p);

    match s {
        Ok(s) => s,
        Err(_) => panic!(""),
    }
}
