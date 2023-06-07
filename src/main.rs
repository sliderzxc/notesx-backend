extern crate rocket;

use std::rc::Rc;
use rocket::routes;
use crate::auth::data::repository::auth_repository_impl::AuthRepositoryImpl;
use crate::auth::domain::repository::auth_repository::AuthRepository;
use crate::auth::data::cloud::mongo_database::MongoDatabase;
use crate::auth::domain::routes::sign_up_route::sign_up_route;

mod auth;

pub struct App {
    auth_repository: Rc<dyn AuthRepository>
}

impl App {
    pub async fn new() -> Self {
        let database = MongoDatabase::init().await;

        App {
            auth_repository: Rc::new(AuthRepositoryImpl::new(database))
        }
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}

#[rocket::main]
async fn main() {
    let app = App::new().await;
    let _server = rocket::build()
        .manage(app)
        .mount("/auth",routes![sign_up_route])
        .launch().await;
}