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
    models::{TenantNote, UpsertTenantNote},
};

/// Get the note for a tenant
///
/// Returns the admin note attached to the specified tenant, or 404 if none has been created yet.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    get,
    path = "/admin/tenants/{mongo_id}/note",
    params(
        ("mongo_id" = String, Path, description = "MongoDB ObjectId of the tenant"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Tenant note found", body = TenantNote),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "No note found for this tenant", body = crate::error::ErrorResponse),
    ),
    tag = "Tenant Notes"
)]
#[tracing::instrument(skip_all, fields(tenant.mongo_id = %mongo_id))]
pub async fn get_tenant_note(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(mongo_id): Path<String>,
) -> Result<Json<TenantNote>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let note = db.get_tenant_note(&mongo_id).await?.ok_or_else(|| {
        tracing::warn!(tenant.mongo_id = %mongo_id, "tenant note not found");
        AppError::NotFound
    })?;

    Ok(Json(note))
}

/// Create or update the note for a tenant
///
/// Upserts the admin note for the specified tenant. If a note already exists for this tenant
/// it is updated in place; otherwise a new one is created. There is exactly one note per tenant.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    put,
    path = "/admin/tenants/{mongo_id}/note",
    request_body = UpsertTenantNote,
    params(
        ("mongo_id" = String, Path, description = "MongoDB ObjectId of the tenant"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Tenant note upserted", body = TenantNote),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
    ),
    tag = "Tenant Notes"
)]
#[tracing::instrument(skip_all, fields(tenant.mongo_id = %mongo_id))]
pub async fn upsert_tenant_note(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(mongo_id): Path<String>,
    Json(payload): Json<UpsertTenantNote>,
) -> Result<Json<TenantNote>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let id = Ulid::new().to_string();
    let note = db
        .upsert_tenant_note(&id, &mongo_id, payload.note.as_deref(), payload.price)
        .await?;

    tracing::info!(note.id = %note.id, tenant.mongo_id = %mongo_id, "tenant note upserted");
    Ok(Json(note))
}

/// Delete the note for a tenant
///
/// Deletes the admin note for the specified tenant. Returns 204 No Content on success,
/// or 404 if no note exists for this tenant.
///
/// The request must include a valid Bearer token in the Authorization header for authentication
/// (use the `/auth/signin` endpoint to obtain a token).
#[utoipa::path(
    delete,
    path = "/admin/tenants/{mongo_id}/note",
    params(
        ("mongo_id" = String, Path, description = "MongoDB ObjectId of the tenant"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 204, description = "Tenant note deleted"),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "No note found for this tenant", body = crate::error::ErrorResponse),
    ),
    tag = "Tenant Notes"
)]
#[tracing::instrument(skip_all, fields(tenant.mongo_id = %mongo_id))]
pub async fn delete_tenant_note(
    OptionalAuthSession(user): OptionalAuthSession,
    State(db): State<DatabaseService>,
    Path(mongo_id): Path<String>,
) -> Result<StatusCode, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    if db.get_tenant_note(&mongo_id).await?.is_none() {
        tracing::warn!(tenant.mongo_id = %mongo_id, "tenant note not found for delete");
        return Err(AppError::NotFound);
    }

    db.delete_tenant_note(&mongo_id).await?;
    tracing::info!(tenant.mongo_id = %mongo_id, "tenant note deleted");
    Ok(StatusCode::NO_CONTENT)
}
