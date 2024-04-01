use std::ops::Deref;
use rocket::form::Form;
use rocket::serde::json::Json;

use rocket::State;
use rocket_okapi::openapi;
use serde_json::Value;
use crate::error::{Code, Error};
use crate::model::{Context};

use crate::services::Services;
use crate::error::Error::ParamsError;
use crate::providers::Providers;
use crate::store::Store;


#[openapi(tag = "Hello World")]
#[get("/")]
pub async fn index() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "favicon.ico")]
#[get("/favicon.ico")]
pub async fn favicon() -> &'static str {
    "favicon.ico"
}


/// # Upload Doc With Any Json, Path as Collection Name
#[openapi(tag = "Doc")]
#[post("/<collection>", data="<req>")]
pub async fn upload_doc(store: &State<Store>, collection: String, req: Json<Value>) -> Result<Json<bool>, Error> {
    let req = req.into_inner();
    let pvd = Providers::new(store);
    let ctx = Context::new();
    let svc = Services::new(ctx, pvd);
    let res = svc.doc().upload_doc(collection, req).await?;
    Ok(Json(res))

}

