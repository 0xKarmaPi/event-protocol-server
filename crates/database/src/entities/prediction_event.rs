use crate::native_enums::{Network, Side};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "prediction_event")]
#[schema(as = PredictionEvent)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub pubkey: String,
    pub creator: String,
    pub title: String,
    pub description: String,
    pub left_description: String,
    pub right_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_mint_decimals: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_mint_decimals: Option<i32>,
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
    pub burning: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Side>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: Network,
    pub created_date: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ticket::Entity")]
    Ticket,
}

impl Related<super::ticket::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ticket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
