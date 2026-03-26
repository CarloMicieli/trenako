use configuration::DatabaseSettings;
use sqlx::PgPool;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};

const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

pub async fn start_postgres() -> (ContainerAsync<GenericImage>, u16) {
    let container = GenericImage::new("postgres", "16.1-alpine")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stderr(
            r#"listening on IPv4 address "0.0.0.0", port 5432"#,
        ))
        .with_env_var("POSTGRES_USER", POSTGRES_USER)
        .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
        .with_env_var("POSTGRES_DB", POSTGRES_DB)
        .start()
        .await
        .expect("Failed to start postgres container");

    let port = container
        .get_host_port_ipv4(5432.tcp())
        .await
        .expect("Failed to get postgres port");
    (container, port)
}

#[derive(Debug)]
pub struct Database(DatabaseSettings);

impl Database {
    pub fn new(port: u16) -> Self {
        let database_settings = DatabaseSettings::new(POSTGRES_USER, POSTGRES_PASSWORD, "127.0.0.1", port, POSTGRES_DB);
        Database(database_settings)
    }

    pub fn test_settings(&self) -> DatabaseSettings {
        self.0.clone()
    }

    pub fn pg_pool(&self) -> PgPool {
        self.0.get_connection_pool()
    }

    pub async fn run_database_migrations(&self) {
        sqlx::migrate!("../../../migrations")
            .run(&self.pg_pool())
            .await
            .expect("Failed to migrate the database");
    }

    pub async fn apply_fixture(&self, sql: &str) {
        sqlx::raw_sql(sql)
            .execute(&self.pg_pool())
            .await
            .expect("Failed to apply SQL fixture");
    }
}
