use rocket::{serde::json::Json, State};
use rocket::post;
use crate::App;
use crate::auth::data::entities::sign_up_data::SignUpData;

#[post("/signup", data = "<sign_up_data>")]
pub fn sign_up_route(
    app: &State<App>,
    sign_up_data: Json<SignUpData>
) -> Json<String>{
    app.auth_repository.sign_up(&sign_up_data);
    Json("Hello".parse().unwrap())
}