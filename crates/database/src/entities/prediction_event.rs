use crate::native_enums::Side;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "prediction_event")]
#[schema(as = PredictionEvent)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
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
    pub start_date: DateTimeWithTimeZone,
    pub end_date: DateTimeWithTimeZone,
    pub burning: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Side>,
    #[serde(skip)]
    pub deleted: bool,
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
