use super::point_model::Point;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Struct Cab to encapsulate what info a cab should be have
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cab {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub location: Point,
    pub destination: Option<Point>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_id: Option<ObjectId>,
}

// helper functions picking up things that can be accessed outside of the
// library using the pub keyword, updating destination is not public
// because we don't want the Cab instance outside library to change it.
impl Cab {
    pub fn new(location: Point) -> Self {
        Cab {
            id: None,
            location,
            destination: None,
            person_id: None,
        }
    }

    pub fn update_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn update_destination(&mut self, destination: Option<Point>) {
        self.destination = destination;
    }

    pub fn update_person_id(&mut self, id: Option<ObjectId>) {
        self.person_id = id;
    }
}
