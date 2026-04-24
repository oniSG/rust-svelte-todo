use chrono::NaiveDate;

use crate::{error::AppError, models::FinancePlanEntry};

use super::DatabaseService;

impl DatabaseService {
    /// Return all finance plan entries ordered by period date ascending.
    pub async fn list_finance_plan_entries(&self) -> Result<Vec<FinancePlanEntry>, AppError> {
        let entries = sqlx::query_as!(
            FinancePlanEntry,
            "SELECT id, period_date, income, created_at, updated_at
             FROM admin_finance_plan_entries
             ORDER BY period_date"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(entries)
    }

    /// Return a single finance plan entry by ID, or `None` if not found.
    pub async fn get_finance_plan_entry(
        &self,
        id: &str,
    ) -> Result<Option<FinancePlanEntry>, AppError> {
        let entry = sqlx::query_as!(
            FinancePlanEntry,
            "SELECT id, period_date, income, created_at, updated_at
             FROM admin_finance_plan_entries
             WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Insert a new finance plan entry and return the created row.
    pub async fn create_finance_plan_entry(
        &self,
        id: &str,
        period_date: NaiveDate,
        income: i32,
    ) -> Result<FinancePlanEntry, AppError> {
        let entry = sqlx::query_as!(
            FinancePlanEntry,
            "INSERT INTO admin_finance_plan_entries (id, period_date, income)
             VALUES ($1, $2, $3)
             RETURNING id, period_date, income, created_at, updated_at",
            id,
            period_date,
            income,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Update an existing finance plan entry. Returns `None` if the ID does not exist.
    pub async fn update_finance_plan_entry(
        &self,
        id: &str,
        period_date: NaiveDate,
        income: i32,
    ) -> Result<Option<FinancePlanEntry>, AppError> {
        let entry = sqlx::query_as!(
            FinancePlanEntry,
            "UPDATE admin_finance_plan_entries
             SET period_date = $2, income = $3, updated_at = NOW()
             WHERE id = $1
             RETURNING id, period_date, income, created_at, updated_at",
            id,
            period_date,
            income,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Delete a finance plan entry by ID.
    pub async fn delete_finance_plan_entry(&self, id: &str) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM admin_finance_plan_entries WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
