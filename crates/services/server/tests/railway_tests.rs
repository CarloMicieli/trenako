pub mod common;

use crate::common::database::start_postgres;
use crate::common::seeding::seed_railways;
use crate::common::templates::{render, setup_hbs};
use crate::common::spawn_app;
use ::common::contacts::{MailAddress, PhoneNumber};
use ::common::length::Length;
use ::common::measure_units::MeasureUnit;
use ::common::organizations::OrganizationEntityType;
use ::common::socials::Handler;
use catalog::common::TrackGauge;
use catalog::railways::period_of_activity::RailwayStatus;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use chrono::NaiveDate;
use isocountry::CountryCode;
use reqwest::StatusCode;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use serde_json::json;
use uuid::Uuid;

const API_RAILWAYS: &str = "/api/railways";

#[tokio::test]
async fn it_should_return_409_when_the_railway_already_exists() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_railways(&pg_pool).await;

    let railway_name = "FS";

    let reg = setup_hbs();
    let data = json!({"railway_name": railway_name});
    let request = render(reg, "railways", data);

    let endpoint = sut.endpoint(API_RAILWAYS);
    let response = client
        .post(endpoint)
        .json(&request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::from_u16(409).unwrap(), response.status());
}

#[tokio::test]
async fn it_should_create_new_railways() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let railway_name = Uuid::new_v4().to_string();
    let railway_id = RailwayId::new(&railway_name);
    let expected_location = format!("{}/{}", API_RAILWAYS, railway_id);

    let reg = setup_hbs();
    let data = json!({"railway_name": railway_name});
    let request = render(reg, "railways", data);

    let endpoint = sut.endpoint(API_RAILWAYS);
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
            railway_id as "railway_id: RailwayId",
            name,
            abbreviation,
            registered_company_name,
            organization_entity_type as "organization_entity_type?: OrganizationEntityType",
            description_de,
            description_en,
            description_fr,
            description_it,
            country,
            operating_since,
            operating_until,
            status as "status?: RailwayStatus",
            gauge_meters,
            track_gauge as "track_gauge?: TrackGauge",
            headquarters as "headquarters!: Vec<String>",
            total_length_mi,
            total_length_km,
            contact_email,
            contact_website_url,
            contact_phone,
            socials_facebook,
            socials_instagram,
            socials_linkedin,
            socials_twitter,
            socials_youtube
        FROM railways WHERE name = $1"#,
        &railway_name
    )
    .fetch_one(&pg_pool)
    .await
    .expect("Failed to fetch saved railway.");

    let operating_since = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
    let gauge_mt = Decimal::from_str("1.435").unwrap();
    let total_length_km = Decimal::from_str("10000").unwrap();
    let total_length_mi = Decimal::from_str("621.4").unwrap();

    assert_eq!(railway_id, saved.railway_id);
    assert_eq!(railway_name, saved.name);
    assert_eq!(Some(String::from("rr")), saved.abbreviation);
    assert_eq!(Some(String::from("Rust Raiload & Co")), saved.registered_company_name);
    assert_eq!(
        Some(OrganizationEntityType::StateOwnedEnterprise),
        saved.organization_entity_type
    );
    assert_eq!(Some(String::from("description")), saved.description_en);
    assert_eq!(Some(String::from("descrizione")), saved.description_it);
    assert_eq!(Some(String::from("beschreibung")), saved.description_de);
    assert_eq!(Some(String::from("description")), saved.description_fr);
    assert_eq!(CountryCode::USA.alpha2(), saved.country);
    assert_eq!(Some(operating_since), saved.operating_since);
    assert_eq!(None, saved.operating_until);
    assert_eq!(Some(RailwayStatus::Active), saved.status);
    assert_eq!(vec![String::from("Some City")], saved.headquarters);
    assert_eq!(Some(gauge_mt), saved.gauge_meters);
    assert_eq!(Some(TrackGauge::Standard), saved.track_gauge);
    assert_eq!(Some(total_length_km), saved.total_length_km);
    assert_eq!(Some(total_length_mi), saved.total_length_mi);
    assert_eq!(Some(String::from("mail@mail.com")), saved.contact_email);
    assert_eq!(Some(String::from("+14152370800")), saved.contact_phone);
    assert_eq!(Some(String::from("https://www.site.com")), saved.contact_website_url);
    assert_eq!(Some(String::from("facebook_handler")), saved.socials_facebook);
    assert_eq!(Some(String::from("instagram_handler")), saved.socials_instagram);
    assert_eq!(Some(String::from("linkedin_handler")), saved.socials_linkedin);
    assert_eq!(Some(String::from("twitter_handler")), saved.socials_twitter);
    assert_eq!(Some(String::from("youtube_handler")), saved.socials_youtube);
}

#[tokio::test]
async fn it_should_find_railways_by_id() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_railways(&pg_pool).await;

    let endpoint = sut.endpoint(API_RAILWAYS);
    let endpoint = format!("{}/fs", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());

    let body = response
        .json::<Railway>()
        .await
        .expect("Failed to fetch the response body");

    let contact_info = body.contact_info.unwrap();
    let socials = body.socials.unwrap();

    assert_eq!(RailwayId::new("FS"), body.railway_id);
    assert_eq!("FS", body.name);
    assert_eq!(Some(String::from("FS")), body.abbreviation);
    assert_eq!(
        Some(String::from("Ferrovie dello Stato Italiane S.p.A.")),
        body.registered_company_name
    );
    assert_eq!(
        Some(OrganizationEntityType::StateOwnedEnterprise),
        body.organization_entity_type
    );
    assert_eq!(Some(&String::from("description")), body.description.english());
    assert_eq!(Some(&String::from("descrizione")), body.description.italian());
    assert_eq!(Some(&String::from("description")), body.description.french());
    assert_eq!(Some(&String::from("beschreibung")), body.description.german());

    assert_eq!(Some(MailAddress::new("mail@mail.com")), contact_info.email);
    assert_eq!(Some(PhoneNumber::new("+14152370800")), contact_info.phone);
    assert_eq!(
        Some(String::from("https://www.fsitaliane.it")),
        contact_info.website_url.map(|it| it.to_string())
    );
    assert_eq!(Some(Handler::new("fsitaliane")), socials.facebook);
    assert_eq!(Some(Handler::new("fsitaliane")), socials.instagram);
    assert_eq!(Some(Handler::new("ferrovie-dello-stato-s-p-a-")), socials.linkedin);
    assert_eq!(Some(Handler::new("FSitaliane")), socials.twitter);
    assert_eq!(Some(Handler::new("fsitaliane")), socials.youtube);
    assert_eq!(vec![String::from("Roma")], body.headquarters);

    let gauge = body.gauge.expect("railway gauge is missing");
    assert_eq!(TrackGauge::Standard, gauge.track_gauge);
    assert_eq!(Length::new(dec!(1.435), MeasureUnit::Meters), gauge.meters);

    let total_length = body.total_length.expect("railway total length is missing");
    assert_eq!(Length::Kilometers(dec!(24564.0)), total_length.kilometers);
    assert_eq!(Length::Miles(dec!(15263.4)), total_length.miles);

    let period_of_activity = body.period_of_activity.expect("railway period of activity is missing");
    assert_eq!(RailwayStatus::Active, period_of_activity.status);
    assert_eq!(
        Some(NaiveDate::from_ymd_opt(1905, 7, 1).unwrap()),
        period_of_activity.operating_since
    );
    assert_eq!(None, period_of_activity.operating_until);
}

#[tokio::test]
async fn it_should_return_404_not_found_when_the_railway_is_not_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_RAILWAYS);
    let endpoint = format!("{}/not-found", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn it_should_return_200_with_empty_result_set_when_no_railway_is_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_RAILWAYS);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Railways>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(0, body.items.len());
}

#[tokio::test]
async fn it_should_return_the_railways_list() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_railways(&pg_pool).await;

    let endpoint = sut.endpoint(API_RAILWAYS);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Railways>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(2, body.items.len());
}

#[derive(Debug)]
struct Saved {
    railway_id: RailwayId,
    name: String,
    abbreviation: Option<String>,
    registered_company_name: Option<String>,
    organization_entity_type: Option<OrganizationEntityType>,
    description_de: Option<String>,
    description_en: Option<String>,
    description_fr: Option<String>,
    description_it: Option<String>,
    country: String,
    operating_since: Option<NaiveDate>,
    operating_until: Option<NaiveDate>,
    status: Option<RailwayStatus>,
    gauge_meters: Option<Decimal>,
    track_gauge: Option<TrackGauge>,
    headquarters: Vec<String>,
    total_length_mi: Option<Decimal>,
    total_length_km: Option<Decimal>,
    contact_email: Option<String>,
    contact_website_url: Option<String>,
    contact_phone: Option<String>,
    socials_facebook: Option<String>,
    socials_instagram: Option<String>,
    socials_linkedin: Option<String>,
    socials_twitter: Option<String>,
    socials_youtube: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Railways {
    items: Vec<Railway>,
}
