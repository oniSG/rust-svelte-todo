pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub mongo_db_url: String,
}

pub async fn load_config() -> Result<Config, std::env::VarError> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mongo_db_url = std::env::var("MONGO_DB_URL").expect("MONGO_DB_URL must be set");

    Ok(Config {
        database_url,
        jwt_secret,
        mongo_db_url,
    })
}
