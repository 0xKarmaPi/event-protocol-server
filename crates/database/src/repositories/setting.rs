use sea_orm::{sea_query::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entities::setting;

#[derive(Clone, Copy)]
pub enum Setting {
    LastestScannedSignature,
}

pub async fn get(db: &DatabaseConnection, key: Setting) -> Result<Option<String>, DbErr> {
    let val = setting::Entity::find_by_id(key.to_str_key())
        .one(db)
        .await?
        .map(|record| record.value);

    Ok(val)
}

pub async fn set(db: &DatabaseConnection, key: Setting, value: String) -> Result<(), DbErr> {
    setting::Entity::update_many()
        .col_expr(setting::Column::Value, Expr::value(value))
        .filter(setting::Column::Key.eq(key.to_str_key()))
        .exec(db)
        .await?;

    Ok(())
}

impl Setting {
    fn to_str_key(self) -> String {
        match self {
            Self::LastestScannedSignature => "lastest_scanned_signature".to_string(),
        }
    }
}
