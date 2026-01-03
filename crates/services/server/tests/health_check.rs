pub mod common;

use crate::common::{IMAGE_NAME, create_docker_test, spawn_app};

#[tokio::test]
async fn health_check_works() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let endpoint = sut.endpoint("/health-check");
        let response = client.get(endpoint).send().await.expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    })
    .await;
}
