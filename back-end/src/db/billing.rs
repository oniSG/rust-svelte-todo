use crate::{
    error::AppError,
    models::{BillingCondition, BillingEntry},
};

use super::DatabaseService;

impl DatabaseService {
    /// Return all billing entries ordered by fans_count ascending.
    pub async fn list_billing_entries(&self) -> Result<Vec<BillingEntry>, AppError> {
        let entries = sqlx::query_as!(
            BillingEntry,
            r#"SELECT id,
                      fans_count,
                      condition AS "condition: BillingCondition",
                      basic_plan_price,
                      standard_plan_price,
                      premium_plan_price,
                      individual_plan_price,
                      created_at,
                      updated_at
               FROM admin_billing_entries
               ORDER BY fans_count"#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(entries)
    }

    /// Return a single billing entry by ID, or `None` if not found.
    pub async fn get_billing_entry(&self, id: &str) -> Result<Option<BillingEntry>, AppError> {
        let entry = sqlx::query_as!(
            BillingEntry,
            r#"SELECT id,
                      fans_count,
                      condition AS "condition: BillingCondition",
                      basic_plan_price,
                      standard_plan_price,
                      premium_plan_price,
                      individual_plan_price,
                      created_at,
                      updated_at
               FROM admin_billing_entries
               WHERE id = $1"#,
            id,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Insert a new billing entry and return the created row.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_billing_entry(
        &self,
        id: &str,
        fans_count: i32,
        condition: BillingCondition,
        basic_plan_price: Option<i32>,
        standard_plan_price: Option<i32>,
        premium_plan_price: Option<i32>,
        individual_plan_price: bool,
    ) -> Result<BillingEntry, AppError> {
        let entry = sqlx::query_as!(
            BillingEntry,
            r#"INSERT INTO admin_billing_entries
                   (id, fans_count, condition, basic_plan_price, standard_plan_price, premium_plan_price, individual_plan_price)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING id,
                         fans_count,
                         condition AS "condition: BillingCondition",
                         basic_plan_price,
                         standard_plan_price,
                         premium_plan_price,
                         individual_plan_price,
                         created_at,
                         updated_at"#,
            id,
            fans_count,
            condition as BillingCondition,
            basic_plan_price,
            standard_plan_price,
            premium_plan_price,
            individual_plan_price,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Update an existing billing entry. Returns `None` if the ID does not exist.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_billing_entry(
        &self,
        id: &str,
        fans_count: i32,
        condition: BillingCondition,
        basic_plan_price: Option<i32>,
        standard_plan_price: Option<i32>,
        premium_plan_price: Option<i32>,
        individual_plan_price: bool,
    ) -> Result<Option<BillingEntry>, AppError> {
        let entry = sqlx::query_as!(
            BillingEntry,
            r#"UPDATE admin_billing_entries
               SET fans_count          = $2,
                   condition           = $3,
                   basic_plan_price    = $4,
                   standard_plan_price = $5,
                   premium_plan_price  = $6,
                   individual_plan_price = $7,
                   updated_at          = NOW()
               WHERE id = $1
               RETURNING id,
                         fans_count,
                         condition AS "condition: BillingCondition",
                         basic_plan_price,
                         standard_plan_price,
                         premium_plan_price,
                         individual_plan_price,
                         created_at,
                         updated_at"#,
            id,
            fans_count,
            condition as BillingCondition,
            basic_plan_price,
            standard_plan_price,
            premium_plan_price,
            individual_plan_price,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(entry)
    }

    /// Delete a billing entry by ID.
    pub async fn delete_billing_entry(&self, id: &str) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM admin_billing_entries WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
