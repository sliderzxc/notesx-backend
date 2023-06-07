#[post("/auth/signup", data = "<new_user>")]
pub fn sign_up(
    db: &State<MongoRepo>,
    sign_up_data: Json<SignUpData>,
) -> Result<Json<InsertOneResult>, Status> {

}