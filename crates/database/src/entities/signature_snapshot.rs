use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::native_enums::{Context, Event, Network};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "signature_snapshot")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub signature: String,
    pub event: Event,
    pub context: Context,
    #[sea_orm(primary_key, auto_increment = false)]
    pub network: Network,
    pub created_date: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
