use crate::common::database::Database;
use configuration::{LoggingFormat, LoggingLevel, LoggingSettings, ServerSettings, Settings};
use dockertest::{DockerTest, Source};
use server::app;
use sqlx::PgPool;
use std::future::IntoFuture;
use tokio::net::TcpListener;

pub const IMAGE_NAME: &str = "postgres";

pub mod database;
pub mod seeding;
pub mod templates;

#[derive(Debug)]
pub struct ServiceUnderTest {
    pub base_endpoint_url: String,
    pub database: Database,
}

impl ServiceUnderTest {
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint_url, path)
    }

    pub async fn run_database_migrations(&self) {
        self.database.run_database_migrations().await
    }

    pub fn pg_pool(&self) -> PgPool {
        self.database.pg_pool()
    }
}

pub async fn spawn_app(postgres_port: u32) -> ServiceUnderTest {
    let database = Database::new(postgres_port as u16);
    let database_settings = database.test_settings();
    let settings = Settings {
        server: ServerSettings {
            host: String::from("127.0.0.1"),
            port: 0,
        },
        database: database_settings,
        logging: LoggingSettings {
            level: LoggingLevel::Error,
            format: LoggingFormat::Compact,
        },
    };

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let http_server = axum::serve(listener, app::build_app(&settings)).into_future();
    let _handle = tokio::spawn(http_server);

    ServiceUnderTest {
        base_endpoint_url: format!("http://127.0.0.1:{port}"),
        database,
    }
}

pub fn create_docker_test() -> DockerTest {
    let mut test = DockerTest::new().with_default_source(Source::DockerHub);
    test.provide_container(database::create_postgres_container());
    test
}
