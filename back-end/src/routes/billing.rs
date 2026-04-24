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
    models::{BillingEntry, CreateBillingEntry, UpdateBillingEntry},
};

/// List all billing entries
///
/// Returns all billing entries ordered by fans count ascending.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    get,
    path = "/admin/billing",
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "List of billing entries", body = Vec<BillingEntry>),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Billing"
)]
#[tracing::instrument(skip_all)]
pub async fn list_billing_entries(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
) -> Result<Json<Vec<BillingEntry>>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entries = db.list_billing_entries().await?;
    tracing::debug!(count = entries.len(), "listed billing entries");
    Ok(Json(entries))
}

/// Get a billing entry by ID
///
/// Returns a single billing entry identified by its ID.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    get,
    path = "/admin/billing/{id}",
    params(
        ("id" = String, Path, description = "The billing entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Billing entry found", body = BillingEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Billing"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn get_billing_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
) -> Result<Json<BillingEntry>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entry = db.get_billing_entry(&id).await?.ok_or_else(|| {
        tracing::warn!(entry.id = %id, "billing entry not found");
        AppError::NotFound
    })?;

    Ok(Json(entry))
}

/// Create a billing entry
///
/// Creates a new billing entry defining plan prices for a given fan count threshold and condition.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    post,
    path = "/admin/billing",
    request_body = CreateBillingEntry,
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 201, description = "Billing entry created", body = BillingEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Billing"
)]
#[tracing::instrument(skip_all)]
pub async fn create_billing_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Json(payload): Json<CreateBillingEntry>,
) -> Result<(StatusCode, Json<BillingEntry>), AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let id = Ulid::new().to_string();
    let entry = db
        .create_billing_entry(
            &id,
            payload.fans_count,
            payload.condition,
            payload.basic_plan_price,
            payload.standard_plan_price,
            payload.premium_plan_price,
            payload.individual_plan_price,
        )
        .await?;

    tracing::info!(entry.id = %entry.id, "billing entry created");
    Ok((StatusCode::CREATED, Json(entry)))
}

/// Update a billing entry
///
/// Replaces all fields of an existing billing entry.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    put,
    path = "/admin/billing/{id}",
    request_body = UpdateBillingEntry,
    params(
        ("id" = String, Path, description = "The billing entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Billing entry updated", body = BillingEntry),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Billing"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn update_billing_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBillingEntry>,
) -> Result<Json<BillingEntry>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let entry = db
        .update_billing_entry(
            &id,
            payload.fans_count,
            payload.condition,
            payload.basic_plan_price,
            payload.standard_plan_price,
            payload.premium_plan_price,
            payload.individual_plan_price,
        )
        .await?
        .ok_or_else(|| {
            tracing::warn!(entry.id = %id, "billing entry not found for update");
            AppError::NotFound
        })?;

    tracing::info!(entry.id = %entry.id, "billing entry updated");
    Ok(Json(entry))
}

/// Delete a billing entry
///
/// Deletes a billing entry identified by its ID. Returns 204 No Content on success.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    delete,
    path = "/admin/billing/{id}",
    params(
        ("id" = String, Path, description = "The billing entry ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 204, description = "Billing entry deleted"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Entry not found", body = crate::error::ErrorResponse),
    ),
    tag = "Billing"
)]
#[tracing::instrument(skip_all, fields(entry.id = %id))]
pub async fn delete_billing_entry(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    if db.get_billing_entry(&id).await?.is_none() {
        tracing::warn!(entry.id = %id, "billing entry not found for delete");
        return Err(AppError::NotFound);
    }

    db.delete_billing_entry(&id).await?;
    tracing::info!(entry.id = %id, "billing entry deleted");
    Ok(StatusCode::NO_CONTENT)
}
