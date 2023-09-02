// assign student to teacher
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] // rename _id to id
    pub id: Option<ObjectId>,
    pub student_id: String,
    pub teacher_id: String,
    pub subject: String,
    pub date: String,
    pub time: String,
}
