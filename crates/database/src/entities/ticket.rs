use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::native_enums::Side;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ticket")]
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
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
