pub mod common;

use crate::common::database::start_postgres;
use crate::common::seeding::seed_scales;
use crate::common::spawn_app;
use crate::common::templates::{render, setup_hbs};
use ::common::length::Length;
use ::common::measure_units::MeasureUnit;
use catalog::common::TrackGauge;
use catalog::scales::ratio::Ratio;
use catalog::scales::scale::Scale;
use catalog::scales::scale_id::ScaleId;
use catalog::scales::standard::Standard;
use reqwest::StatusCode;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

const API_SCALES: &str = "/api/scales";

#[tokio::test]
async fn it_should_return_409_when_the_scale_already_exists() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_scales(&pg_pool).await;

    let scale_name = "H0";

    let reg = setup_hbs();
    let data = json!({"scale_name": scale_name});
    let request = render(reg, "scales", data);

    let endpoint = sut.endpoint(API_SCALES);
    let response = client
        .post(endpoint)
        .json(&request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::from_u16(409).unwrap(), response.status());
}

#[tokio::test]
async fn it_should_create_new_scales() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let scale_name = Uuid::new_v4().to_string();
    let scale_id = ScaleId::new(&scale_name);
    let expected_location = format!("{}/{}", API_SCALES, scale_id);

    let ratio_value = Decimal::from_str_exact("87").unwrap();

    let gauge_mm = Decimal::from_str_exact("16.5").unwrap();
    let gauge_in = Decimal::from_str_exact("0.65").unwrap();

    let reg = setup_hbs();
    let data = json!({"scale_name": scale_name});
    let request = render(reg, "scales", data);

    let endpoint = sut.endpoint(API_SCALES);
    let response = client
        .post(endpoint)
        .json(&request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    assert_eq!(expected_location, response.headers()["Location"]);

    let pg_pool = sut.pg_pool();
    let saved = sqlx::query_as!(
        Saved,
        r#"SELECT
                scale_id as "scale_id: ScaleId",
                name,
                ratio,
                gauge_millimeters,
                gauge_inches,
                track_gauge as "track_gauge: TrackGauge",
                description_de,
                description_en,
                description_fr,
                description_it,
                standards as "standards!: Vec<Standard>"
            FROM scales
            WHERE name = $1"#,
        &scale_name
    )
    .fetch_one(&pg_pool)
    .await
    .expect("Failed to fetch saved scale.");

    assert_eq!(scale_id, saved.scale_id);
    assert_eq!(scale_name, saved.name);
    assert_eq!(Some(String::from("description")), saved.description_en);
    assert_eq!(Some(String::from("descrizione")), saved.description_it);
    assert_eq!(Some(String::from("beschreibung")), saved.description_de);
    assert_eq!(Some(String::from("description")), saved.description_fr);
    assert_eq!(ratio_value, saved.ratio);
    assert_eq!(Some(gauge_mm), saved.gauge_millimeters);
    assert_eq!(Some(gauge_in), saved.gauge_inches);
    assert_eq!(TrackGauge::Standard, saved.track_gauge);
    assert_eq!(vec![Standard::NEM, Standard::NMRA], saved.standards);
}

#[tokio::test]
async fn it_should_find_scales_by_id() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_scales(&pg_pool).await;

    let endpoint = sut.endpoint(API_SCALES);
    let endpoint = format!("{}/h0", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());

    let body = response
        .json::<Scale>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(ScaleId::new("H0"), body.scale_id);
    assert_eq!("H0", body.name);
    assert_eq!(Some(&String::from("description")), body.description.english());
    assert_eq!(Some(&String::from("descrizione")), body.description.italian());
    assert_eq!(Some(&String::from("beschreibung")), body.description.german());
    assert_eq!(Some(&String::from("description")), body.description.french());

    let gauge = body.gauge;
    assert_eq!(TrackGauge::Standard, gauge.track_gauge);
    assert_eq!(Length::new(dec!(0.65), MeasureUnit::Inches), gauge.inches);
    assert_eq!(Length::new(dec!(16.5), MeasureUnit::Millimeters), gauge.millimeters);

    let ratio = body.ratio;
    assert_eq!(Ratio::try_from(dec!(87.0)).unwrap(), ratio);
    assert_eq!(HashSet::from([Standard::NEM]), body.standards);
}

#[tokio::test]
async fn it_should_return_404_not_found_when_the_scale_is_not_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_SCALES);
    let endpoint = format!("{}/not-found", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn it_should_get_the_scales_list() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_scales(&pg_pool).await;

    let endpoint = sut.endpoint(API_SCALES);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Scales>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(2, body.items.len());
}

#[tokio::test]
async fn it_should_return_200_with_empty_result_set_when_no_scale_is_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_SCALES);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Scales>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(0, body.items.len());
}

#[derive(Debug)]
struct Saved {
    scale_id: ScaleId,
    name: String,
    ratio: Decimal,
    gauge_millimeters: Option<Decimal>,
    gauge_inches: Option<Decimal>,
    track_gauge: TrackGauge,
    description_de: Option<String>,
    description_en: Option<String>,
    description_fr: Option<String>,
    description_it: Option<String>,
    standards: Vec<Standard>,
}

#[derive(Debug, Deserialize)]
struct Scales {
    items: Vec<Scale>,
}
