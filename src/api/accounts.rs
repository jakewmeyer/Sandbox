use axum::{
    extract::{rejection::PathRejection, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
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
pub struct CreateAccount {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAccount {
    pub provider_id: Option<String>,
}

pub fn routes() -> Router<ApiContext> {
    Router::new()
        .route("/v1/accounts", get(list_accounts_handler))
        .route("/v1/accounts/:id", get(get_account_by_id_handler))
        .route("/v1/accounts", post(create_account_handler))
        .route("/v1/accounts/:id", delete(delete_account_handler))
        .route("/v1/accounts/:id/users", get(list_account_users_handler))
}

pub async fn list_accounts(
    ctx: &ApiContext,
    page: &Pagination,
) -> Result<Vec<accounts::Model>, Error> {
    let accounts = Accounts::find()
        .filter(accounts::Column::RowId.gte(page.after))
        .order_by_asc(accounts::Column::RowId)
        .limit(page.limit)
        .all(&ctx.db)
        .await?;
    Ok(accounts)
}

pub async fn get_account_by_id(ctx: &ApiContext, id: Uuid) -> Result<accounts::Model, Error> {
    Accounts::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or(Error::NotFound)
}

pub async fn create_account(ctx: &ApiContext) -> Result<accounts::Model, Error> {
    let account = accounts::ActiveModel {
        ..Default::default()
    };
    let account = account.insert(&ctx.db).await?;
    Ok(account)
}

pub async fn delete_account(ctx: &ApiContext, id: Uuid) -> Result<accounts::Model, Error> {
    let account = Accounts::find_by_id(id).one(&ctx.db).await?;
    let account = account.ok_or(Error::NotFound)?;
    let mut account: accounts::ActiveModel = account.into();
    account.deleted = Set(Some(DateTime::from(Utc::now())));
    let account = account.update(&ctx.db).await?;
    Ok(account)
}

pub async fn list_account_users(
    ctx: &ApiContext,
    id: Uuid,
    page: &Pagination,
) -> Result<Vec<users::Model>, Error> {
    let result = Accounts::find()
        .find_with_related(Users)
        .filter(accounts::Column::Id.eq(id))
        .filter(users::Column::RowId.gte(page.after))
        .order_by_asc(users::Column::RowId)
        .limit(page.limit)
        .all(&ctx.db)
        .await?;
    let (_, users) = result.first().ok_or(Error::NotFound)?.to_owned();
    Ok(users)
}

pub async fn list_accounts_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    page: Pagination,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("list:account")?;
    let users = list_accounts(&ctx, &page).await?;
    Ok(Json(users))
}

pub async fn create_account_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("create:account")?;
    let created = create_account(&ctx).await?;
    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn get_account_by_id_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    account_id: Result<Path<Uuid>, PathRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("retrieve:account")?;
    let Path(account_id) = account_id?;
    let account = get_account_by_id(&ctx, account_id).await?;
    Ok(Json(account))
}

pub async fn delete_account_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    account_id: Result<Path<Uuid>, PathRejection>,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("delete:account")?;
    let Path(account_id) = account_id?;
    let deleted = delete_account(&ctx, account_id).await?;
    Ok(Json(deleted))
}

pub async fn list_account_users_handler(
    user: AuthUser,
    State(ctx): State<ApiContext>,
    account_id: Result<Path<Uuid>, PathRejection>,
    page: Pagination,
) -> Result<impl IntoResponse, Error> {
    user.has_permission("list:user:account")?;
    let Path(account_id) = account_id?;
    let users = list_account_users(&ctx, account_id, &page).await?;
    Ok(Json(users))
}
