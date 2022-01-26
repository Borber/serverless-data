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
async fn index(req: Json<AddVO>) -> Json<serde_json::Value> {
    if !check(req.auth_code.clone()) {
        return Json(serde_json::json!{check_failed()})
    }
    match database::add(req.database.borrow(), req.add.clone()).await {
        Err(_) => Json(serde_json::json!{Status::fail("Add failed".to_string())}),
        _ => Json(serde_json::json!{Status::success()})
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    local_lib::run(index).await
}