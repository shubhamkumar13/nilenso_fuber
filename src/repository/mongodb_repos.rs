use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use rocket::serde::json::Json;

use crate::models::{cab_model::Cab, person_model::Person, point_model::Point};

pub fn hello() {
    println!("Hello from mongodb_repos.rs")
}

pub struct MongoRepo {
    cabs: Collection<Cab>,
    persons: Collection<Person>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = match Client::with_uri_str(uri) {
            Ok(c) => c,
            Err(_) => panic!("unable to get a client"),
        };

        let db = client.database("fuber");
        let cabs: Collection<Cab> = db.collection("Cab");
        let persons: Collection<Person> = db.collection("Person");
        MongoRepo { cabs, persons }
    }

    pub fn create_person(&self, new_person: Person) -> Result<InsertOneResult, Error> {
        let new_entry = new_person.clone();

        let person = self
            .persons
            .insert_one(new_entry, None)
            .ok()
            .expect("Error creating new person");

        Ok(person)
    }

    pub fn get_person(&self, id: &String) -> Result<Person, Error> {
        match ObjectId::parse_str(id) {
            Ok(obj_id) => {
                let filter = doc! {"_id": obj_id};
                Ok(self
                    .persons
                    .find_one(filter, None)
                    .ok()
                    .expect("Error getting person's detail")
                    .unwrap())
            }
            Err(_) => Err(Error::DeserializationError {
                message: "Error while parsing person object".to_string(),
            }),
        }
    }

    pub fn get_cab(&self, id: &String) -> Result<Cab, Error> {
        match ObjectId::parse_str(id) {
            Ok(obj_id) => {
                let filter = doc! {"_id": obj_id};
                Ok(self
                    .cabs
                    .find_one(filter, None)
                    .ok()
                    .expect("Error getting person's detail")
                    .unwrap())
            }
            Err(_) => Err(Error::DeserializationError {
                message: "Error while parsing person object".to_string(),
            }),
        }
    }

    pub fn create_cab(&self, new_cab: Cab) -> Result<InsertOneResult, Error> {
        let new_entry = new_cab.clone();

        let cab = self
            .cabs
            .insert_one(new_entry, None)
            .ok()
            .expect("Error creating new cab");

        Ok(cab)
    }

    pub fn create_fleet(&self, fleet: Vec<Cab>) -> Result<InsertManyResult, Error> {
        let new_entry_vec = fleet.clone();

        let cabs = self
            .cabs
            .insert_many(new_entry_vec, None)
            .ok()
            .expect("Error creating fleet");

        Ok(cabs)
    }

    pub fn get_fleet(&self) -> Result<Vec<Cab>, Error> {
        Ok(self
            .cabs
            .find(None, None)
            .ok()
            .expect("Error getting the fleet")
            .filter(|x| (*x).is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<Cab>>())
    }

    pub fn assign_person(&self, cab_id: &String, new_cab: Cab) -> Result<UpdateResult, Error> {
        unimplemented!()
    }
}
