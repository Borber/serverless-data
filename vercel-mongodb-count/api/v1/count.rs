use std::borrow::Borrow;
use poem::handler;
use poem::web::Json;
use local_lib::{
    Error,
    api::*,
    database,
    auth::check
};
use local_lib::auth::check_failed;

#[handler]
async fn index(req: Json<CountVO>) -> Json<serde_json::Value> {
    match database::count(req.database.borrow()).await {
        Ok(i) => Json(serde_json::json!{
            CountDTO{
                status: Status::success(),
                total: i
            }}),
        _ => Json(serde_json::json!{Status::fail("Count failed".to_string())})
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    local_lib::run(index).await
}