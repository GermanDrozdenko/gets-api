use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "beer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub beer_id: i32,
    pub name: String,
    pub untappd_style: String,
    pub category: String,
    pub subcategory: String,
    pub style: Option<String>,
    pub brewery: String,
    #[sea_orm(column_type = "Float")]
    pub abv: f32,
    pub description: Option<String>,
    pub img_url: Option<String>,
    pub last_review: String,
    pub location: String,
}

#[derive(Deserialize)]
pub struct FilterBeer {
    pub location: String,
    pub category: String,
    pub subcategory: String,
    pub style: String,
}

#[derive(Deserialize)]
pub struct LocationFilter {
    pub location: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
