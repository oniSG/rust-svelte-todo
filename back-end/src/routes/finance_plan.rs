use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use ulid::Ulid;

use crate::{
    auth::OptionalAuthSession,
    db::DatabaseService,
    error::AppError,
    models::{CreateFinancePlanEntry, FinancePlanEntry, UpdateFinancePlanEntry},
};

/// List all finance plan entries
///
/// Returns all finance plan entries ordered by period date ascending.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    get,
    path = "/admin/finance-plan",
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "List of finance plan entries", body = Vec<FinancePlanEntry>),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Finance Plan"
)]
#[tracing::instrument(skip_all)]
pub async fn list_finance_plan_entries(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
) -> Result<Json<Vec<FinancePlanEntry>>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entries = db.list_finance_plan_entries().await?;
    tracing::debug!(count = entries.len(), "listed finance plan entries");
    Ok(Json(entries))
}

/// Get a finance plan entry by ID
///
/// Returns a single finance plan entry identified by its ID.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    get,
    path = "/admin/finance-plan/{id}",
    params(
        ("id" = String, Path, description = "The finance plan entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Finance plan entry found", body = FinancePlanEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Finance Plan"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn get_finance_plan_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
) -> Result<Json<FinancePlanEntry>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entry = db.get_finance_plan_entry(&id).await?.ok_or_else(|| {
        tracing::warn!(entry.id = %id, "finance plan entry not found");
        AppError::NotFound
    })?;

    Ok(Json(entry))
}

/// Create a finance plan entry
///
/// Creates a new finance plan entry for the given period and income amount.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    post,
    path = "/admin/finance-plan",
    request_body = CreateFinancePlanEntry,
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Finance plan entry created", body = FinancePlanEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Finance Plan"
)]
#[tracing::instrument(skip_all)]
pub async fn create_finance_plan_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Json(payload): Json<CreateFinancePlanEntry>,
) -> Result<(StatusCode, Json<FinancePlanEntry>), AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let id = Ulid::new().to_string();
    let entry = db
        .create_finance_plan_entry(&id, payload.period_date, payload.income)
        .await?;

    tracing::info!(entry.id = %entry.id, "finance plan entry created");
    Ok((StatusCode::CREATED, Json(entry)))
}

/// Update a finance plan entry
///
/// Replaces the period date and income of an existing finance plan entry.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    put,
    path = "/admin/finance-plan/{id}",
    request_body = UpdateFinancePlanEntry,
    params(
        ("id" = String, Path, description = "The finance plan entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Finance plan entry updated", body = FinancePlanEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Finance Plan"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn update_finance_plan_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateFinancePlanEntry>,
) -> Result<Json<FinancePlanEntry>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entry = db
        .update_finance_plan_entry(&id, payload.period_date, payload.income)
        .await?
        .ok_or_else(|| {
            tracing::warn!(entry.id = %id, "finance plan entry not found for update");
            AppError::NotFound
        })?;

    tracing::info!(entry.id = %entry.id, "finance plan entry updated");
    Ok(Json(entry))
}

/// Delete a finance plan entry
///
/// Deletes a finance plan entry identified by its ID. Returns 204 No Content on success.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    delete,
    path = "/admin/finance-plan/{id}",
    params(
        ("id" = String, Path, description = "The finance plan entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 204, description = "Finance plan entry deleted"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Finance Plan"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn delete_finance_plan_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    if db.get_finance_plan_entry(&id).await?.is_none() {
        tracing::warn!(entry.id = %id, "finance plan entry not found for delete");
        return Err(AppError::NotFound);
    }

    db.delete_finance_plan_entry(&id).await?;
    tracing::info!(entry.id = %id, "finance plan entry deleted");
    Ok(StatusCode::NO_CONTENT)
}
