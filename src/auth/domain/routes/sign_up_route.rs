use rocket::{serde::json::Json, State};
use rocket::post;
use crate::App;
use crate::auth::data::entities::sign_up_request_data::SignUpRequestData;

#[post("/signup", data = "<sign_up_data>")]
pub fn sign_up_route(
    app: &State<App>,
    sign_up_data: Json<SignUpRequestData>
) -> Json<String>{
    let result = app.sign_up_use_case.execute(&sign_up_data);
    if result == true {
        Json("Success".parse().unwrap())
    } else {
        Json("Error".parse().unwrap())
    }
}