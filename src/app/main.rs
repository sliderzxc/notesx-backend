#[macro_use]
extern crate rocket;

use my_project::auth::common::auth_repository::MongoRepo;

#[launch]
fn rocket() -> _ {
    let database = MongoRepo::init();
    rocket::build()
        .manage(database)
}