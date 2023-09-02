mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, delete_user, get_user, update_user, get_all_users};
use api::teacher_api::{create_teacher,get_all_teachers, get_teacher, update_teacher};
use repository::mongodb_repo::MongoRepo;
/*
fn index() -> &'static str {
    "Hello, world!"
}
*/


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes! [get_teacher])
        .mount("/", routes![update_teacher])
        .mount("/", routes![get_all_teachers])
        .mount("/", routes![create_teacher])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}
