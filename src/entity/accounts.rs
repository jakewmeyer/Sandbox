use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum AccountStatus {
    Inactive = 0,
    Active = 1,
    Suspended = 2,
    Cancelled = 3,
    Disabled = 4,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub status: AccountStatus,
    pub created: DateTimeWithTimeZone,
    pub updated: DateTimeWithTimeZone,
    pub deleted: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_accounts::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::users_accounts::Relation::Accounts.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
