use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    config::{AppConfig, AppState},
    entities::{
        beer,
        prelude::{Beer, FilterBeer, LocationFilter},
    },
};
use rand::Rng;

#[get("/")]
async fn check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/all_beer")]
async fn get_all_beer(data: web::Data<AppState>, filter: Json<LocationFilter>) -> impl Responder {
    let beer = all_beer(data, filter).await;

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

#[post("/random_beer")]
async fn get_random_beer(
    data: web::Data<AppState>,
    filter: Json<LocationFilter>,
) -> impl Responder {
    let beer = all_beer(data, filter).await;

    let random_beer = beer[rand::thread_rng().gen_range(0..beer.len())].clone();

    Json(random_beer)
}

#[get("assets/images/{filename}")]
pub async fn get_file(req: HttpRequest, filename: web::Path<String>) -> impl Responder {
    let file_path = std::path::PathBuf::from(AppConfig::from_env().home_dir)
        .as_path()
        .join("static/")
        .join(filename.into_inner());

    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

    file.into_response(&req)
}

async fn all_beer(data: web::Data<AppState>, filter: Json<LocationFilter>) -> Vec<beer::Model> {
    let conn = &data.conn;

    Beer::find()
        .filter(beer::Column::Location.contains(&filter.location))
        .all(conn)
        .await
        .unwrap()
}
