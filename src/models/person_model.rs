use super::point_model::Point;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: Point,
    pub destination: Point,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cab_id: Option<ObjectId>,
}

// all the methods are public because we want the Person instance
// to access all it's methods and not Fleet or Cab's
impl Person {
    pub fn new(id: Option<ObjectId>, name: String, location: Point, destination: Point) -> Self {
        Person {
            id,
            name,
            location,
            destination,
            cab_id: None,
        }
    }
}
