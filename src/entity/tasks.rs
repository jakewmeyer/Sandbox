use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum TaskState {
    Created = 0,
    Scheduled = 1,
    Running = 2,
    Failed = 3,
    Canceled = 4,
    Completed = 5,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub state: TaskState,
    pub priority: i16,
    pub attempt: i16,
    pub max_attempts: i16,
    pub created_at: DateTimeWithTimeZone,
    pub attempted_at: Option<DateTimeWithTimeZone>,
    pub scheduled_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text")]
    pub queue: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub payload: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
