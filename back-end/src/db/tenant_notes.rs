use crate::{error::AppError, models::TenantNote};

use super::DatabaseService;

impl DatabaseService {
    /// Return the note for a given tenant, or `None` if none exists yet.
    pub async fn get_tenant_note(&self, mongo_id: &str) -> Result<Option<TenantNote>, AppError> {
        let note = sqlx::query_as!(
            TenantNote,
            r#"SELECT id, mongo_id, note, price, created_at, updated_at
               FROM admin_finance_tenant_notes
               WHERE mongo_id = $1"#,
            mongo_id,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(note)
    }

    /// Insert or update the note for a tenant.
    ///
    /// Uses `INSERT … ON CONFLICT (mongo_id) DO UPDATE` so callers do not need to
    /// distinguish between create and update — there is at most one note per tenant.
    pub async fn upsert_tenant_note(
        &self,
        id: &str,
        mongo_id: &str,
        note: Option<&str>,
        price: Option<i32>,
    ) -> Result<TenantNote, AppError> {
        let entry = sqlx::query_as!(
            TenantNote,
            r#"INSERT INTO admin_finance_tenant_notes (id, mongo_id, note, price)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (mongo_id) DO UPDATE
                   SET note       = EXCLUDED.note,
                       price      = EXCLUDED.price,
                       updated_at = NOW()
               RETURNING id, mongo_id, note, price, created_at, updated_at"#,
            id,
            mongo_id,
            note,
            price,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Delete the note for a tenant. No-ops silently if no note exists.
    pub async fn delete_tenant_note(&self, mongo_id: &str) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM admin_finance_tenant_notes WHERE mongo_id = $1",
            mongo_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
