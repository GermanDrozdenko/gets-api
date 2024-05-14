use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::AppState,
    entities::{
        beer,
        prelude::{Beer, FilterBeer, LocationFilter},
    },
};

#[get("/")]
async fn check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/all_beer")]
async fn get_all_beer(data: web::Data<AppState>, filter: Json<LocationFilter>) -> impl Responder {
    let conn = &data.conn;

    let beer = Beer::find()
        .filter(beer::Column::Location.contains(&filter.location))
        .all(conn)
        .await
        .unwrap();

    Json(beer)
}

#[post("/result_beer")]
async fn get_result_beer(data: web::Data<AppState>, filter: Json<FilterBeer>) -> impl Responder {
    let conn = &data.conn;

    let beer = Beer::find()
        .filter(beer::Column::Location.contains(&filter.location))
        .filter(beer::Column::Category.contains(&filter.category))
        .filter(beer::Column::Subcategory.contains(&filter.subcategory))
        .filter(beer::Column::Style.eq(&filter.style))
        .all(conn)
        .await
        .unwrap();

    Json(beer)
}
