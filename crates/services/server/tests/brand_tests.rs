pub mod common;

use crate::common::database::start_postgres;
use crate::common::seeding::seed_brands;
use crate::common::templates::{render, setup_hbs};
use crate::common::spawn_app;
use ::common::contacts::{MailAddress, PhoneNumber};
use ::common::organizations::OrganizationEntityType;
use ::common::socials::Handler;
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_status::BrandStatus;
use isocountry::CountryCode;
use reqwest::StatusCode;
use serde_derive::Deserialize;
use serde_json::json;
use uuid::Uuid;

const API_BRANDS: &str = "/api/brands";

#[tokio::test]
async fn it_should_return_409_when_the_brand_already_exists() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_brands(&pg_pool).await;

    let brand_name = "ACME";

    let reg = setup_hbs();
    let data = json!({"brand_name": brand_name});
    let request = render(reg, "brands", data);

    let endpoint = sut.endpoint(API_BRANDS);
    let response = client
        .post(endpoint)
        .json(&request)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(StatusCode::from_u16(409).unwrap(), response.status());
}

#[tokio::test]
async fn it_should_create_new_brands() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let brand_name = Uuid::new_v4().to_string();
    let expected_location = format!("{}/{}", API_BRANDS, brand_name);

    let reg = setup_hbs();
    let data = json!({"brand_name": brand_name});
    let request = render(reg, "brands", data);

    let endpoint = sut.endpoint(API_BRANDS);
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
    let saved = sqlx::query_as!(Saved,
            r#"SELECT
                    brand_id, name, registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", group_name, 
                    description_de, description_en, description_fr, description_it, kind as "kind: BrandKind", status as "status: BrandStatus",
                    contact_email, contact_website_url, contact_phone,
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube
                FROM brands WHERE name = $1"#, &brand_name)
        .fetch_one(&pg_pool)
        .await
        .expect("Failed to fetch saved brand.");

    assert_eq!(brand_name, saved.brand_id);
    assert_eq!(brand_name, saved.name);
    assert_eq!(BrandKind::Industrial, saved.kind);
    assert_eq!(Some(String::from("beschreibung")), saved.description_de);
    assert_eq!(Some(String::from("description")), saved.description_en);
    assert_eq!(Some(String::from("description")), saved.description_fr);
    assert_eq!(Some(String::from("descrizione")), saved.description_it);
    assert_eq!(Some(String::from("UNKNOWN")), saved.group_name);
    assert_eq!(Some(String::from("Registered Company Ltd")), saved.registered_company_name);
    assert_eq!(Some(OrganizationEntityType::LimitedCompany), saved.organization_entity_type);
    assert_eq!(Some(BrandStatus::Active), saved.status);
    assert_eq!(Some(String::from("mail@mail.com")), saved.contact_email);
    assert_eq!(Some(String::from("+14152370800")), saved.contact_phone);
    assert_eq!(Some(String::from("https://www.site.com")), saved.contact_website_url);
    assert_eq!(Some(String::from("Rue Morgue 22")), saved.address_street_address);
    assert_eq!(Some(String::from("Apartment 42")), saved.address_extended_address);
    assert_eq!(Some(String::from("London")), saved.address_city);
    assert_eq!(Some(String::from("REG")), saved.address_region);
    assert_eq!(Some(String::from("1H2 4BB")), saved.address_postal_code);
    assert_eq!(Some(String::from("GB")), saved.address_country);
    assert_eq!(Some(String::from("facebook_handler")), saved.socials_facebook);
    assert_eq!(Some(String::from("instagram_handler")), saved.socials_instagram);
    assert_eq!(Some(String::from("linkedin_handler")), saved.socials_linkedin);
    assert_eq!(Some(String::from("twitter_handler")), saved.socials_twitter);
    assert_eq!(Some(String::from("youtube_handler")), saved.socials_youtube);
}

#[tokio::test]
async fn it_should_find_brands_by_id() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_brands(&pg_pool).await;

    let endpoint = sut.endpoint(API_BRANDS);
    let endpoint = format!("{}/acme", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert!(response.status().is_success());

    let body = response
        .json::<Brand>()
        .await
        .expect("Failed to fetch the response body");

    let address = body.address.unwrap();
    let contact_info = body.contact_info.unwrap();
    let socials = body.socials.unwrap();

    assert_eq!(BrandId::new("ACME"), body.brand_id);
    assert_eq!("ACME", body.name);
    assert_eq!(BrandKind::Industrial, body.kind);
    assert_eq!(Some(&String::from("description")), body.description.english());
    assert_eq!(Some(&String::from("descrizione")), body.description.italian());
    assert_eq!(Some(&String::from("description")), body.description.french());
    assert_eq!(Some(&String::from("beschreibung")), body.description.german());
    assert_eq!(Some(String::from("UNKNOWN")), body.group_name);
    assert_eq!(
        Some(String::from("Associazione Costruzioni Modellistiche Esatte")),
        body.registered_company_name
    );
    assert_eq!(
        Some(OrganizationEntityType::LimitedCompany),
        body.organization_entity_type
    );
    assert_eq!(Some(BrandStatus::Active), body.status);
    assert_eq!(Some(MailAddress::new("mail@acmetreni.com")), contact_info.email);
    assert_eq!(Some(PhoneNumber::new("+39029867556")), contact_info.phone);
    assert_eq!(
        Some(String::from("http://www.acmetreni.com")),
        contact_info.website_url.map(|it| it.to_string())
    );
    assert_eq!(String::from("Viale Lombardia, 27"), address.street_address);
    assert_eq!(Some(String::from("Interno 42")), address.extended_address);
    assert_eq!(String::from("Milano"), address.city);
    assert_eq!(Some(String::from("MI")), address.region);
    assert_eq!(String::from("20131"), address.postal_code);
    assert_eq!(CountryCode::ITA, address.country);
    assert_eq!(Some(Handler::new("facebook_handler")), socials.facebook);
    assert_eq!(Some(Handler::new("instagram_handler")), socials.instagram);
    assert_eq!(Some(Handler::new("linkedin_handler")), socials.linkedin);
    assert_eq!(Some(Handler::new("twitter_handler")), socials.twitter);
    assert_eq!(Some(Handler::new("youtube_handler")), socials.youtube);
}

#[tokio::test]
async fn it_should_return_404_not_found_when_the_brand_is_not_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_BRANDS);
    let endpoint = format!("{}/not-found", endpoint);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn it_should_return_the_brands_list() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let pg_pool = sut.pg_pool();
    seed_brands(&pg_pool).await;

    let endpoint = sut.endpoint(API_BRANDS);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Brands>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(2, body.items.len());
}

#[tokio::test]
async fn it_should_return_200_with_empty_result_set_when_no_brand_is_found() {
    let (_container, port) = start_postgres().await;
    let sut = spawn_app(port).await;
    let client = reqwest::Client::new();
    sut.run_database_migrations().await;

    let endpoint = sut.endpoint(API_BRANDS);
    let response = client.get(endpoint).send().await.expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let body = response
        .json::<Brands>()
        .await
        .expect("Failed to fetch the response body");

    assert_eq!(0, body.items.len());
}

#[derive(Debug)]
struct Saved {
    brand_id: String,
    name: String,
    registered_company_name: Option<String>,
    organization_entity_type: Option<OrganizationEntityType>,
    group_name: Option<String>,
    description_de: Option<String>,
    description_en: Option<String>,
    description_fr: Option<String>,
    description_it: Option<String>,
    kind: BrandKind,
    status: Option<BrandStatus>,
    contact_email: Option<String>,
    contact_website_url: Option<String>,
    contact_phone: Option<String>,
    address_street_address: Option<String>,
    address_extended_address: Option<String>,
    address_city: Option<String>,
    address_region: Option<String>,
    address_postal_code: Option<String>,
    address_country: Option<String>,
    socials_facebook: Option<String>,
    socials_instagram: Option<String>,
    socials_linkedin: Option<String>,
    socials_twitter: Option<String>,
    socials_youtube: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Brands {
    items: Vec<Brand>,
}
