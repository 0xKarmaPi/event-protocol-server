use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::native_enums::{Rst, Side};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "ticket")]
#[schema(as = Ticket)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub pubkey: String,
    pub event_pubkey: String,
    pub creator: String,
    pub selection: Side,
    #[sea_orm(column_type = "Decimal(Some((80, 0)))")]
    pub amount: Decimal,
    pub claimed: bool,
    pub withdrawn: bool,
    pub created_date: DateTimeWithTimeZone,
    pub result: Rst,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::prediction_event::Entity",
        from = "Column::EventPubkey",
        to = "super::prediction_event::Column::Pubkey"
    )]
    PredictionEvent,
}

impl Related<super::prediction_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PredictionEvent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
