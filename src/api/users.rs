use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection},
        Path, State,
    },
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use sea_orm::{entity::*, query::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{accounts, prelude::*, users};
use crate::error::Error;

use super::{auth::AuthUser, pagination::Pagination, ApiContext};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub provider_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub provider_id: Option<String>,
}

pub fn routes() -> Router<ApiContext> {
    Router::new()
        .route("/v1/users", get(list_users_handler))
        .route("/v1/users/:id", get(get_user_by_id_handler))
        .route("/v1/users", post(create_user_handler))
        .route("/v1/users/:id", patch(update_user_handler))
        .route("/v1/users/:id", delete(delete_user_handler))
        .route("/v1/users/:id/accounts", get(list_user_accounts_handler))
}

pub async fn list_users(ctx: &ApiContext, page: &Pagination) -> Result<Vec<users::Model>, Error> {
    let users = Users::find()
        .filter(users::Column::RowId.gte(page.after))
        .order_by_asc(users::Column::RowId)
        .limit(page.limit)
        .all(&ctx.db)
        .await?;
    Ok(users)
}

pub async fn create_user(ctx: &ApiContext, user: CreateUser) -> Result<users::Model, Error> {
    let user = users::ActiveModel {
        provider_id: Set(user.provider_id),
        ..Default::default()
    };
    let user = user.insert(&ctx.db).await?;
    Ok(user)
}

pub async fn get_user_by_id(ctx: &ApiContext, id: Uuid) -> Result<users::Model, Error> {
    Users::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or(Error::NotFound)
}

pub async fn get_user_by_provider_id(
    ctx: &ApiContext,
    id: &String,
) -> Result<Option<users::Model>, Error> {
    let user = Users::find()
        .filter(users::Column::ProviderId.eq(id))
        .one(&ctx.db)
        .await?;
    Ok(user)
}

pub async fn update_user(
    ctx: &ApiContext,
    id: Uuid,
    update: UpdateUser,
) -> Result<users::Model, Error> {
    let user = Users::find_by_id(id).one(&ctx.db).await?;
    let user = user.ok_or(Error::NotFound)?;
    let mut user: users::ActiveModel = user.into();
    if let Some(provider_id) = update.provider_id {
        user.provider_id = Set(provider_id);
    }
    let user = user.update(&ctx.db).await?;
    Ok(user)
}

pub async fn delete_user(ctx: &ApiContext, id: Uuid) -> Result<users::Model, Error> {
    let user = Users::find_by_id(id).one(&ctx.db).await?;
    let user = user.ok_or(Error::NotFound)?;
    let mut user: users::ActiveModel = user.into();
    user.deleted = Set(Some(DateTime::from(Utc::now())));
    let user = user.update(&ctx.db).await?;
    Ok(user)
}

pub async fn list_user_accounts(
    ctx: &ApiContext,
    id: Uuid,
    page: &Pagination,
) -> Result<Vec<accounts::Model>, Error> {
    let result = Users::find()
        .find_with_related(Accounts)
        .filter(users::Column::Id.eq(id))
        .filter(accounts::Column::RowId.gte(page.after))
        .order_by_asc(accounts::Column::RowId)
        .limit(page.limit)
        .all(&ctx.db)
        .await?;
    let (_, accounts) = result.first().ok_or(Error::NotFound)?.to_owned();
    Ok(accounts)
}

async fn list_users_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    page: Pagination,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("list:user")?;
    let users = list_users(&ctx, &page).await?;
    Ok(Json(users))
}

async fn get_user_by_id_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    user_id: Result<Path<Uuid>, PathRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("retrieve:user")?;
    let Path(user_id) = user_id?;
    let found = get_user_by_id(&ctx, user_id).await?;
    Ok(Json(found))
}

async fn create_user_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    body: Result<Json<CreateUser>, JsonRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("create:user")?;
    let Json(body) = body?;
    let created = create_user(&ctx, body).await?;
    Ok(Json(created))
}

async fn update_user_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    user_id: Result<Path<Uuid>, PathRejection>,
    body: Result<Json<UpdateUser>, JsonRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("update:user")?;
    let Path(user_id) = user_id?;
    let Json(body) = body?;
    let updated = update_user(&ctx, user_id, body).await?;
    Ok(Json(updated))
}

async fn delete_user_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    user_id: Result<Path<Uuid>, PathRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("delete:user")?;
    let Path(user_id) = user_id?;
    delete_user(&ctx, user_id).await?;
    Ok(())
}

async fn list_user_accounts_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    user_id: Result<Path<Uuid>, PathRejection>,
    page: Pagination,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("list:user:accounts")?;
    let Path(user_id) = user_id?;
    let found = list_user_accounts(&ctx, user_id, &page).await?;
    Ok(Json(found))
}
