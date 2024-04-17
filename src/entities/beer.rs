use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "beer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub beer_id: i32,
    pub name: String,
    pub style: String,
    pub group_filter: String,
    pub style_filter: Option<String>,
    pub brewery: String,
    #[sea_orm(column_type = "Float", nullable)]
    pub abv: Option<f32>,
    pub description: Option<String>,
    pub img_url: Option<String>,
    pub last_review: String,
    pub location: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
