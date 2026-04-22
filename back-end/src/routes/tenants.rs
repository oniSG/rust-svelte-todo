use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    auth::OptionalAuthSession,
    error::AppError,
    mongo::{
        MongoService,
        tenants::{FanCountResponse, TenantFanCount, TenantResponse},
    },
};

// ---------------------------------------------------------------------------
// GET /tenants
// ---------------------------------------------------------------------------

/// List all tenants
///
/// Returns all tenant documents from `mt_admin.tenants`.
/// Each tenant describes a club or organisation hosted on the platform,
/// together with its enabled feature modules and the name of its own database.
#[utoipa::path(
    get,
    path = "/tenants",
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "List of tenants",      body = Vec<TenantResponse>),
        (status = 401, description = "Unauthorized",         body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error",body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all)]
pub async fn list_tenants(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
) -> Result<Json<Vec<TenantResponse>>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let tenants = mongo.list_tenants().await?;
    tracing::debug!(count = tenants.len(), "listed tenants");
    Ok(Json(tenants))
}

// ---------------------------------------------------------------------------
// GET /tenants/fans/count
// ---------------------------------------------------------------------------

/// Fan counts for all tenants
///
/// Iterates every tenant in `mt_admin.tenants`, counts the documents in that
/// tenant's `fans` collection, and returns the results in a single response.
///
/// Useful for a quick cross-tenant overview without needing to know individual
/// hostnames up front.
#[utoipa::path(
    get,
    path = "/tenants/fans/count",
    params(
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Fan counts per tenant",body = Vec<TenantFanCount>),
        (status = 401, description = "Unauthorized",         body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error",body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all)]
pub async fn count_all_fans(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
) -> Result<Json<Vec<TenantFanCount>>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let counts = mongo.count_fans_per_tenant().await?;
    tracing::debug!(tenants = counts.len(), "counted fans for all tenants");
    Ok(Json(counts))
}

// ---------------------------------------------------------------------------
// GET /tenants/:hostname
// ---------------------------------------------------------------------------

/// Get tenant by hostname
///
/// Returns the tenant document whose `hostname` field matches the given value
/// (e.g. `fkteplice.relatoo.app`).
#[utoipa::path(
    get,
    path = "/tenants/{hostname}",
    params(
        ("hostname" = String, Path, description = "The tenant's full hostname, e.g. `fkteplice.relatoo.app`"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Tenant found",         body = TenantResponse),
        (status = 401, description = "Unauthorized",         body = crate::error::ErrorResponse),
        (status = 404, description = "Tenant not found",     body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error",body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all, fields(hostname = %hostname))]
pub async fn get_tenant(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
    Path(hostname): Path<String>,
) -> Result<Json<TenantResponse>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let tenant = mongo
        .get_tenant_by_hostname(&hostname)
        .await?
        .ok_or_else(|| {
            tracing::warn!(hostname = %hostname, "tenant not found");
            AppError::NotFound
        })?;

    Ok(Json(tenant))
}

// ---------------------------------------------------------------------------
// GET /tenants/:hostname/fans/count
// ---------------------------------------------------------------------------

/// Fan count for a specific tenant
///
/// Returns the number of documents in the `fans` collection of the tenant
/// identified by `hostname`.
///
/// The count is read directly from the tenant's own database, so it always
/// reflects the current state without any caching.
#[utoipa::path(
    get,
    path = "/tenants/{hostname}/fans/count",
    params(
        ("hostname" = String, Path, description = "The tenant's full hostname, e.g. `fkteplice.relatoo.app`"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Fan count",            body = FanCountResponse),
        (status = 401, description = "Unauthorized",         body = crate::error::ErrorResponse),
        (status = 404, description = "Tenant not found",     body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error",body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all, fields(hostname = %hostname))]
pub async fn count_fans_for_tenant(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
    Path(hostname): Path<String>,
) -> Result<Json<FanCountResponse>, AppError> {
    user.ok_or(AppError::Unauthorized)?;

    let fan_count = mongo.count_fans_for_hostname(&hostname).await?;
    tracing::debug!(hostname = %hostname, fan_count, "counted fans for tenant");
    Ok(Json(FanCountResponse { fan_count }))
}
