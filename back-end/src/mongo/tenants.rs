use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::AppError;

use super::MongoService;

// ---------------------------------------------------------------------------
// Internal deserialization type (never leaves this module)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct Tenant {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    hostname: String,
    #[serde(rename = "dbName")]
    db_name: String,
    #[serde(rename = "BUSINESS_MODULE")]
    business_module: Option<String>,
    #[serde(rename = "FANS_MODULE")]
    fans_module: Option<String>,
    #[serde(rename = "CONTENT_TYPE")]
    content_type: Option<String>,
}

// ---------------------------------------------------------------------------
// Public response types
// ---------------------------------------------------------------------------

/// Public representation of a tenant returned by the API.
#[derive(Debug, Serialize, ToSchema)]
pub struct TenantResponse {
    /// MongoDB ObjectId serialised as a 24-character hex string.
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub db_name: String,
    pub business_module: Option<String>,
    pub fans_module: Option<String>,
    pub content_type: Option<String>,
}

impl From<Tenant> for TenantResponse {
    fn from(t: Tenant) -> Self {
        Self {
            id: t.id.to_hex(),
            name: t.name,
            hostname: t.hostname,
            db_name: t.db_name,
            business_module: t.business_module,
            fans_module: t.fans_module,
            content_type: t.content_type,
        }
    }
}

/// Fan count for a single tenant.
#[derive(Debug, Serialize, ToSchema)]
pub struct TenantFanCount {
    pub tenant_name: String,
    pub db_name: String,
    pub fan_count: u64,
}

/// Single-value wrapper for a fan count response.
#[derive(Debug, Serialize, ToSchema)]
pub struct FanCountResponse {
    pub fan_count: u64,
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl MongoService {
    /// Fetch raw tenant documents from `mt_admin.tenants`.
    async fn fetch_tenants(&self) -> Result<Vec<Tenant>, AppError> {
        let docs: Vec<Tenant> = self
            .admin_db()
            .collection::<Tenant>("tenants")
            .find(None, None)
            .await?
            .try_collect()
            .await?;
        Ok(docs)
    }

    /// Return all tenants from `mt_admin.tenants`.
    pub async fn list_tenants(&self) -> Result<Vec<TenantResponse>, AppError> {
        Ok(self
            .fetch_tenants()
            .await?
            .into_iter()
            .map(TenantResponse::from)
            .collect())
    }

    /// Return a single tenant by hostname, or `None` if not found.
    pub async fn get_tenant_by_hostname(
        &self,
        hostname: &str,
    ) -> Result<Option<TenantResponse>, AppError> {
        let result = self
            .admin_db()
            .collection::<Tenant>("tenants")
            .find_one(doc! { "hostname": hostname }, None)
            .await?;
        Ok(result.map(TenantResponse::from))
    }

    /// Count documents in every tenant's `fans` collection and return the
    /// results sorted by tenant name.
    pub async fn count_fans_per_tenant(&self) -> Result<Vec<TenantFanCount>, AppError> {
        let tenants = self.fetch_tenants().await?;
        let mut results = Vec::with_capacity(tenants.len());

        for tenant in tenants {
            let fan_count = self
                .tenant_db(&tenant.db_name)
                .collection::<mongodb::bson::Document>("fans")
                .count_documents(None, None)
                .await?;

            results.push(TenantFanCount {
                tenant_name: tenant.name,
                db_name: tenant.db_name,
                fan_count,
            });
        }

        Ok(results)
    }

    /// Count fans for a single tenant identified by hostname.
    /// Returns `AppError::NotFound` when no tenant matches.
    pub async fn count_fans_for_hostname(&self, hostname: &str) -> Result<u64, AppError> {
        let tenant = self
            .admin_db()
            .collection::<Tenant>("tenants")
            .find_one(doc! { "hostname": hostname }, None)
            .await?
            .ok_or(AppError::NotFound)?;

        let count = self
            .tenant_db(&tenant.db_name)
            .collection::<mongodb::bson::Document>("fans")
            .count_documents(None, None)
            .await?;

        Ok(count)
    }
}
