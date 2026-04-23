use axum::{Json, extract::State};

use crate::{
    auth::OptionalAuthSession,
    error::AppError,
    mongo::{MongoService, tenants::{FansDataPoint, FansDistributions, TenantResponse}},
};

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

    Ok(Json(mongo.list_tenants().await?))
}

/// Get tenant by ID
///
/// Returns the tenant document for the specified tenant ID.
#[utoipa::path(
    get,
    path = "/tenants/{tenant_id}",
    params(
        ("tenant_id" = String, Path, description = "Tenant ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Tenant details",      body = TenantResponse),
        (status = 401, description = "Unauthorized",       body = crate::error::ErrorResponse),
        (status = 404, description = "Tenant not found",   body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error",body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all)]
pub async fn get_tenant(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
    axum::extract::Path(tenant_id): axum::extract::Path<String>,
) -> Result<Json<TenantResponse>, AppError> {
    user.ok_or(AppError::Unauthorized)?;
    return mongo.get_tenant(&tenant_id).await.map(Json).or_else(|err| {
        if matches!(err, AppError::NotFound) {
            Err(AppError::NotFound)
        } else {
            Err(err)
        }
    });
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct TenantStatsResponse {
    /// Total number of fans in the tenant database.
    pub fans_count: u64,
    /// Number of new fans registered in the last 30 days.
    pub new_fans_last_month: u64,
    /// Cumulative monthly fan counts, one point per month (1st of month, UTC).
    /// Starts one month before the first registration (count = 0).
    pub fans_over_time: Vec<FansDataPoint>,
    /// Breakdowns by device, city, and gender.
    pub distributions: FansDistributions,
}

/// Stats for a tenant
///
/// Returns various statistics about the tenant, such as number of users, todos, etc. (not implemented yet)
#[utoipa::path(
    get,
    path = "/tenants/{tenant_id}/stats",
    params(
        ("tenant_id" = String, Path, description = "Tenant ID"),
        ("Authorization" = String, Header, description = "Bearer access token. Format: `Bearer <token>`")
    ),
    security(("bearerAuth" = [])),
    responses(
        (status = 200, description = "Tenant statistics", body = TenantStatsResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 404, description = "Tenant not found", body = crate::error::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::error::ErrorResponse),
    ),
    tag = "Tenants"
)]
#[tracing::instrument(skip_all)]
pub async fn get_tenant_stats(
    OptionalAuthSession(user): OptionalAuthSession,
    State(mongo): State<MongoService>,
    axum::extract::Path(tenant_id): axum::extract::Path<String>,
) -> Result<Json<TenantStatsResponse>, AppError> {
    user.ok_or(AppError::Unauthorized)?;
    let tenant = mongo.get_tenant(&tenant_id).await?;

    let (fans_count, (new_fans_last_month, fans_over_time), distributions) = tokio::try_join!(
        mongo.get_tenant_fans_count(&tenant.db_name),
        mongo.get_tenant_fans_count_in_time(&tenant.db_name),
        mongo.get_tenant_fans_distributions(&tenant.db_name),
    )?;

    Ok(Json(TenantStatsResponse {
        fans_count,
        new_fans_last_month,
        fans_over_time,
        distributions,
    }))
}
