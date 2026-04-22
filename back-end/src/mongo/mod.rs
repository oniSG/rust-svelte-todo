pub mod tenants;

use mongodb::Client;

/// MongoDB client that can reach any database on the Atlas cluster —
/// `mt_admin` for tenant registry data and each tenant's own database.
#[derive(Debug, Clone)]
pub struct MongoService {
    client: Client,
}

impl MongoService {
    /// Connect to the Atlas cluster and return a `MongoService`.
    pub async fn new(uri: &str) -> Result<Self, mongodb::error::Error> {
        let options = mongodb::options::ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        Ok(Self { client })
    }

    pub(crate) fn admin_db(&self) -> mongodb::Database {
        self.client.database("mt_admin")
    }

    pub(crate) fn tenant_db(&self, db_name: &str) -> mongodb::Database {
        self.client.database(db_name)
    }
}
