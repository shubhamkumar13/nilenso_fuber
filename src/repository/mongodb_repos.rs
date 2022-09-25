use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};
use rocket::serde::json::Json;

use crate::{
    api::cab_api::update_cab,
    models::{cab_model::Cab, person_model::Person, point_model::Point},
};

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
        match ObjectId::parse_str(cab_id) {
            Ok(obj_id) => {
                let filter = doc! { "_id" : obj_id};
                let new_doc = doc! {
                    "$set":
                    {
                        "id": new_cab.id,
                        "location" : {
                            "x" : new_cab.location.x,
                            "y" : new_cab.location.y
                        },
                        "destination" : {
                            "x" : new_cab
                                    .destination
                                    .clone()
                                    .expect("cannot get the destination for this cab")
                                    .x,
                            "y" : new_cab
                                    .destination
                                    .clone()
                                    .expect("cannot get the destination for this cab")
                                    .y,
                        },
                        "person_id" : new_cab.person_id,
                    },
                };
                let updated_doc = self
                    .cabs
                    .update_one(filter, new_doc, None)
                    .ok()
                    .expect("cannot update the new cab");

                Ok(updated_doc)
            }
            Err(_) => panic!("assigning person failed"),
        }
    }

    pub fn unassign_person(&self, cab_id: &String, new_cab: Cab) -> Result<UpdateResult, Error> {
        match ObjectId::parse_str(cab_id) {
            Ok(obj_id) => {
                let filter = doc! { "_id" : obj_id};
                let new_doc = doc! {
                    "$set":
                    {
                        "id": new_cab.id,
                        "location" : {
                            "x" : new_cab.location.x,
                            "y" : new_cab.location.y
                        },
                        "destination" : null,
                        "person_id" : null
                    },
                };
                let updated_doc = self
                    .cabs
                    .update_one(filter, new_doc, None)
                    .ok()
                    .expect("cannot update the new cab");

                Ok(updated_doc)
            }
            Err(_) => panic!("unassign person failed"),
        }
    }

    pub fn update_cab(&self, new_cab: Cab) -> Result<UpdateResult, Error> {
        match new_cab.id {
            Some(obj_id) => {
                let filter = doc! { "_id" : obj_id };
                match new_cab.person_id {
                    Some(_) => {
                        let new_doc = doc! {
                            "$set":
                            {
                                "id": new_cab.id,
                                "location" : {
                                    "x" : new_cab.location.x,
                                    "y" : new_cab.location.y
                                },
                                "destination" : {
                                    "x" : new_cab
                                            .destination
                                            .clone()
                                            .expect("cannot get the destination for this cab")
                                            .x,
                                    "y" : new_cab
                                            .destination
                                            .clone()
                                            .expect("cannot get the destination for this cab")
                                            .y,
                                },
                                "person_id" : new_cab.person_id,
                            },
                        };
                        let updated_doc = self
                            .cabs
                            .update_one(filter, new_doc, None)
                            .ok()
                            .expect("cannot update the new cab");

                        Ok(updated_doc)
                    }
                    None => {
                        let new_doc = doc! {
                            "$set":
                            {
                                "id": new_cab.id,
                                "location" : {
                                    "x" : new_cab.location.x,
                                    "y" : new_cab.location.y
                                },
                                "destination" : null,
                                "person_id" : null
                            },
                        };

                        let updated_doc = self
                            .cabs
                            .update_one(filter, new_doc, None)
                            .ok()
                            .expect("cannot update the new cab");

                        Ok(updated_doc)
                    }
                }
            }
            None => Err(Error::DeserializationError {
                message: "Couldn't find the object id".to_string(),
            }),
        }
    }

    pub fn delete_cab(&self, cab_id: &String) -> Result<DeleteResult, Error> {
        match ObjectId::parse_str(cab_id) {
            Ok(obj_id) => {
                let filter = doc! {"_id" : obj_id};
                let deleted_doc = self.cabs.delete_one(filter, None);
                match deleted_doc {
                    Ok(d) => Ok(d),
                    Err(_) => Err(Error::DeserializationError {
                        message: "Cannot delete the cab".into(),
                    }),
                }
            }
            Err(_) => Err(Error::DeserializationError {
                message: "Cannot find the object".to_string(),
            }),
        }
    }

    pub fn update_person(&self, new_person: Person) -> Result<UpdateResult, Error> {
        match new_person.id.clone() {
            Some(obj_id) => {
                let filter = doc! {"_id" : obj_id};
                let new_doc = doc! {
                    "$set":
                    {
                        "id" : new_person.id,
                        "name" : new_person.name,
                        "location" : {
                            "x" : new_person.location.x.clone(),
                            "y" : new_person.location.y.clone(),
                        },
                        "destination" : {
                            "x" : new_person.destination.x.clone(),
                            "y" : new_person.destination.y.clone(),
                        },
                    }
                };

                let updated_doc = self.persons.update_one(filter, new_doc, None).ok();

                match updated_doc {
                    Some(update) => Ok(update),
                    None => Err(Error::DeserializationError {
                        message: "Cannot update the doc".into(),
                    }),
                }
            }
            None => Err(Error::DeserializationError {
                message: "ObjectId for the person doesn't exist".into(),
            }),
        }
    }

    pub fn delete_person(&self, person_id: &String) -> Result<DeleteResult, Error> {
        match ObjectId::parse_str(person_id) {
            Ok(obj_id) => {
                let filter = doc! {"_id" : obj_id};
                let deleted_doc = self.persons.delete_one(filter, None);
                match deleted_doc {
                    Ok(d) => Ok(d),
                    Err(_) => Err(Error::DeserializationError {
                        message: "Cannot delete the person".into(),
                    }),
                }
            }
            Err(_) => Err(Error::DeserializationError {
                message: "Cannot find the ObjectId for the person".to_string(),
            }),
        }
    }

    pub fn delete_fleet(&self) -> Result<DeleteResult, Error> {
        let filter = doc! {};
        let deleted_fleet_docs = self.cabs.delete_many(filter, None);
        match deleted_fleet_docs {
            Ok(d) => Ok(d),
            Err(_) => Err(Error::DeserializationError {
                message: "Cannot delete the complete fleet".into(),
            }),
        }
    }

    pub fn delete_all_people(&self) -> Result<DeleteResult, Error> {
        let filter = doc! {};
        let deleted_people_docs = self.persons.delete_many(filter, None);
        match deleted_people_docs {
            Ok(d) => Ok(d),
            Err(_) => Err(Error::DeserializationError {
                message: "Cannot delete the complete fleet".into(),
            }),
        }
    }
}
