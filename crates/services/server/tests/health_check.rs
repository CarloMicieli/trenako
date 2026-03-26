pub mod common;

use crate::common::database::start_postgres;
use crate::common::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint("/health-check");
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
