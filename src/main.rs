extern crate rocket;

use std::rc::Rc;
use rocket::routes;
use crate::auth::data::repository::auth_repository_impl::AuthRepositoryImpl;
use crate::auth::data::cloud::mongo_database::MongoDatabase;
use crate::auth::domain::routes::sign_up_route::sign_up_route;
use crate::auth::domain::usecases::sign_up_use_case::SignUpUseCase;

mod auth;

pub struct App {
    sign_up_use_case: SignUpUseCase
}

impl App {
    pub async fn new() -> Self {
        let database = MongoDatabase::init().await;
        let auth_repository = Rc::new(AuthRepositoryImpl::new(database));

        App {
            sign_up_use_case: SignUpUseCase::new(auth_repository)
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