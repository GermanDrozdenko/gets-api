use actix_web::{
    get,
    web::{self, Json},
    HttpResponse, Responder,
};
use sea_orm::EntityTrait;

use crate::{config::AppState, entities::prelude::Beer};

#[get("/")]
async fn check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[get("/all_beer")]
async fn get_all_beer(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.conn;

    let beer = Beer::find().all(conn).await.unwrap();

    Json(beer)
}
