use std::env;
extern crate dotenv;

use dotenv::dotenv;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

use crate::models::user_model::User;
use crate::models::teacher_model::Teacher;

pub struct MongoRepo {
    col: Collection<User>,
    teacher_col: Collection<Teacher>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        let teacher_col: Collection<Teacher> = db.collection("Teacher");
        MongoRepo { col , teacher_col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }

    pub fn create_teacher(&self, new_teacher: Teacher) -> Result<InsertOneResult, Error> {
        let new_doc = Teacher {
            id: None,
            name: new_teacher.name,
            degree: new_teacher.degree,
            subexp: new_teacher.subexp,
            experience: new_teacher.experience,
        };
        let teacher = self
            .teacher_col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating teacher");

        Ok(teacher)
    }

    pub fn get_teacher(&self, id: &String) -> Result<Teacher, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let teacher_detail = self
            .teacher_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting teacher's detail");
        Ok(teacher_detail.unwrap())
    }
    
    pub fn update_teacher(&self, id: &String, new_teacher: Teacher) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_teacher.id,
                    "name": new_teacher.name,
                    "degree": new_teacher.degree,
                    "subexp": new_teacher.subexp,
                    "experience": new_teacher.experience
                },
        };
        let updated_doc = self
            .teacher_col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating teacher");
        Ok(updated_doc)
    }

    pub fn get_all_teachers(&self) -> Result<Vec<Teacher>, Error> {
        let cursors = self
            .teacher_col
            .find(None, None)
            .ok()
            .expect("Error getting list of teachers");
        let teachers = cursors.map(|doc| doc.unwrap()).collect();

        Ok(teachers)
    }
}
