use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub provider_id: String,
    #[sea_orm(column_type = "Text")]
    pub stripe_customer_id: String,
    pub created: DateTimeWithTimeZone,
    pub updated: DateTimeWithTimeZone,
    pub deleted: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::accounts::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_accounts::Relation::Accounts.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::users_accounts::Relation::Users.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
