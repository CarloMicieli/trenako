use common::unit_of_work::postgres::PgDatabase;
use configuration::Settings;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pg_pool: Arc<PgPool>,
}

impl AppState {
    pub fn from_settings(settings: &Settings) -> Self {
        let pg_pool = Arc::new(settings.database.get_connection_pool());
        AppState { pg_pool }
    }

    pub fn get_database(&self) -> PgDatabase<'_> {
        PgDatabase::new(&self.pg_pool)
    }
}
