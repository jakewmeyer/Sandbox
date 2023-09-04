use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users_accounts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub row_id: i32,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub created: DateTimeWithTimeZone,
    pub updated: DateTimeWithTimeZone,
    pub deleted: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::accounts::Entity",
        from = "Column::AccountId",
        to = "super::accounts::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Accounts,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl ActiveModelBehavior for ActiveModel {}
