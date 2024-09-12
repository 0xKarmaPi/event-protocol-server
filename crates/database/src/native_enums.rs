use program::{events, log};
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
    #[sea_orm(string_value = "claim_rewards")]
    ClaimRewards,
    #[sea_orm(string_value = "close_event")]
    CloseEvent,
    #[sea_orm(string_value = "deploy_event")]
    DeployEvent,
    #[sea_orm(string_value = "finish_event")]
    FinishEvent,
    #[sea_orm(string_value = "vote_event")]
    VoteEvent,
    #[sea_orm(string_value = "withdraw")]
    Withdraw,
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

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "rst")]
pub enum Rst {
    #[sea_orm(string_value = "lost")]
    Lost,
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "won")]
    Won,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "network")]
pub enum Network {
    #[sea_orm(string_value = "solana")]
    Solana,
    #[sea_orm(string_value = "sonic")]
    Sonic,
}

impl From<events::Side> for Side {
    fn from(side: events::Side) -> Self {
        match side {
            events::Side::Left => Self::Left,
            events::Side::Right => Self::Right,
        }
    }
}

impl Event {
    pub fn from_ref(event: &log::Event) -> Self {
        match event {
            log::Event::VoteEvent(_) => Self::VoteEvent,
            log::Event::DeployEvent(_) => Self::DeployEvent,
            log::Event::FinishEvent(_) => Self::FinishEvent,
            log::Event::CloseEvent(_) => Self::CloseEvent,
            log::Event::ClaimRewards(_) => Self::ClaimRewards,
            log::Event::Withdraw(_) => Self::Withdraw,
        }
    }
}
