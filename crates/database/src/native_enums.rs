use program::events;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "context")]
pub enum Context {
    #[sea_orm(string_value = "scanner")]
    Scanner,
    #[sea_orm(string_value = "stream")]
    Stream,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "event")]
pub enum Event {
    #[sea_orm(string_value = "deploy_event")]
    DeployEvent,
    #[sea_orm(string_value = "finish_event")]
    FinishEvent,
    #[sea_orm(string_value = "vote_event")]
    VoteEvent,
}

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "side")]
pub enum Side {
    #[sea_orm(string_value = "left")]
    Left,
    #[sea_orm(string_value = "right")]
    Right,
}

impl From<events::Side> for Side {
    fn from(side: events::Side) -> Self {
        match side {
            events::Side::Left => Self::Left,
            events::Side::Right => Self::Right,
        }
    }
}
