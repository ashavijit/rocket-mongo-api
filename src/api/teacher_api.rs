use crate::{models::{user_model::User, teacher_model::Teacher}, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/teacher", data = "<new_teacher>")]
pub fn create_teacher(
    db: &State<MongoRepo>,
    new_teacher: Json<Teacher>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Teacher {
        id: None,
        name: new_teacher.name.to_owned(),
        degree: new_teacher.degree.to_owned(),
        subexp: new_teacher.subexp.to_owned(),
        experience: new_teacher.experience.to_owned(),
    };

    let teacher_detail = db.create_teacher(data);

    match teacher_detail {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/teacher/name?<x>")]
pub fn get_teacher(db: &State<MongoRepo>, x: String) -> Result<Json<Teacher>, Status> {
    let id = x;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let teacher_detail = db.get_teacher(&id);

    match teacher_detail {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/teacher/<path>", data = "<new_teacher>")]
pub fn update_teacher(
    db: &State<MongoRepo>,
    path: String,
    new_teacher: Json<Teacher>,
) -> Result<Json<Teacher>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Teacher {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_teacher.name.to_owned(),
        degree: new_teacher.degree.to_owned(),
        subexp: new_teacher.subexp.to_owned(),
        experience: new_teacher.experience.to_owned(),
    };
    let update_result = db.update_teacher(&id, data);

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_teacher_info = db.get_teacher(&id);
                match updated_teacher_info {
                    Ok(teacher) => Ok(Json(teacher)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

// #[delete("/teacher/<path>")]
// pub fn delete_teacher(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
//     let id = path;
//     if id.is_empty() {
//         return Err(Status::BadRequest);
//     };
//     let delete_result = db.delete_teacher(&id);

//     match delete_result {
//         Ok(delete) => {
//             if delete.deleted_count == 1 {
//                 let deleted_teacher_info = db.get_teacher(&id);
//                 match deleted_teacher_info {
//                     Ok(teacher) => Ok(Json(teacher)),
//                     Err(_) => Err(Status::InternalServerError),
//                 }
//             } else {
//                 Err(Status::NotFound)
//             }
//         }
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

#[get("/teachers")]
pub fn get_all_teachers(db: &State<MongoRepo>) -> Result<Json<Vec<Teacher>>, Status> {
    let teachers = db.get_all_teachers();

    match teachers {
        Ok(teacher) => Ok(Json(teacher)),
        Err(_) => Err(Status::InternalServerError),
    }
}


