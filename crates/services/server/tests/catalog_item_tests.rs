use crate::common::seeding::{seed_brands, seed_catalog_items, seed_railways, seed_scales};
use crate::common::templates::{render, setup_hbs};
use crate::common::{IMAGE_NAME, create_docker_test, spawn_app};
use catalog::brands::brand_id::BrandId;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item::CatalogItem;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::delivery_date::DeliveryDate;
use catalog::catalog_items::epoch::Epoch;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::rolling_stock::RollingStock;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{BodyShellType, ChassisType, CouplingSocket, FeatureFlag};
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use reqwest::StatusCode;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;

pub mod common;

const API_CATALOG_ITEMS: &str = "/api/catalog-items";

#[tokio::test]
async fn it_should_return_404_not_found_when_the_catalog_item_is_not_found() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let endpoint = format!("{}/not-found", endpoint);
        let response = client.get(endpoint).send().await.expect("Failed to execute request.");

        assert_eq!(404, response.status().as_u16());
    })
    .await;
}

#[tokio::test]
async fn it_should_find_catalog_items_by_id() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        let client = reqwest::Client::new();
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;
        seed_catalog_items(&pg_pool).await;

        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let endpoint = format!("{}/acme-60011", endpoint);
        let response = client.get(endpoint).send().await.expect("Failed to execute request.");

        assert_eq!(StatusCode::from_u16(200).unwrap(), response.status());

        let body = response
            .json::<CatalogItem>()
            .await
            .expect("Failed to fetch the response body");

        assert_eq!(body.catalog_item_id, CatalogItemId::from_str("acme-60011").unwrap());
        assert_eq!(body.brand.brand_id, BrandId::new("ACME"));
        assert_eq!(body.brand.display, String::from("ACME"));
        assert_eq!(body.category, Category::Locomotives);
        assert_eq!(body.scale.scale_id, ScaleId::new("H0"));
        assert_eq!(body.scale.display, String::from("H0"));
        assert_eq!(body.power_method, PowerMethod::DC);
        assert_eq!(body.epoch, Epoch::V);
        assert_eq!(body.description.italian(), Some(&String::from("Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco versione di origine, pantografi 52 Sommerfeldt")));
        assert_eq!(body.delivery_date, Some(DeliveryDate::by_year(2005)));
        assert_eq!(body.availability_status, Some(AvailabilityStatus::Available));
        assert_eq!(body.count, 1);

        assert_eq!(body.rolling_stocks.len(), 1);

        let rolling_stock: &RollingStock = body.rolling_stocks.first().expect("no rolling stock found");
        let rolling_stock = rolling_stock.clone();
        assert_eq!(rolling_stock.category(), RollingStockCategory::Locomotive);

        match rolling_stock {
            RollingStock::Locomotive {
                id: _,
                railway,
                livery,
                length_over_buffer: _,
                technical_specifications: _,
                class_name,
                road_number,
                series,
                depot,
                locomotive_type,
                dcc_interface,
                control,
                is_dummy } => {
                assert_eq!(railway.railway_id, RailwayId::new("FS"));
                assert_eq!(railway.display, String::from("FS"));
                assert_eq!(livery, Some(String::from("rosso/bianco")));
                assert_eq!(dcc_interface, Some(DccInterface::Mtc21));
                assert_eq!(control, Some(Control::DccReady));
                assert_eq!(class_name, String::from("E402 A"));
                assert_eq!(road_number, String::from("E402 015"));
                assert_eq!(depot, Some(String::from("Milano Centrale")));
                assert_eq!(series, None);
                assert_eq!(locomotive_type, LocomotiveType::ElectricLocomotive);
                assert!(!is_dummy);
            }
            _ => unreachable!("expected a locomotive rolling stock"),
        }
    })
    .await
}

#[tokio::test]
async fn it_should_return_409_when_the_catalog_item_already_exists() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;
        seed_catalog_items(&pg_pool).await;

        let request = json!({
            "brand" : "ACME",
            "item_number" : "60011",
            "category" : "LOCOMOTIVES",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "it" : "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            },
            "details" : {
                "it" : "Motore a 5 poli"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 1,
            "rolling_stocks": []
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(StatusCode::from_u16(409).unwrap(), response.status());
    })
    .await
}

#[tokio::test]
async fn it_should_return_422_when_the_brand_is_not_found() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_scales(&pg_pool).await;

        let request = json!({
            "brand" : "ACME",
            "item_number" : "60011",
            "category" : "LOCOMOTIVES",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "it" : "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            },
            "details" : {
                "it" : "Motore a 5 poli"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 1,
            "rolling_stocks": []
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(StatusCode::from_u16(422).unwrap(), response.status());
    })
    .await
}

#[tokio::test]
async fn it_should_return_422_when_the_scale_is_not_found() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;

        let request = json!({
            "brand" : "ACME",
            "item_number" : "60011",
            "category" : "LOCOMOTIVES",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "it" : "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            },
            "details" : {
                "it" : "Motore a 5 poli"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 1,
            "rolling_stocks": []
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(StatusCode::from_u16(422).unwrap(), response.status());
    })
    .await
}

#[tokio::test]
async fn it_should_return_422_when_the_railway_is_not_found() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let reg = setup_hbs();
        let data = json!({});
        let request = render(reg, "catalog_items", data);

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(StatusCode::from_u16(422).unwrap(), response.status());
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_locomotive() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, catalog_item_id);

        let reg = setup_hbs();
        let data = json!({});
        let request = render(reg, "catalog_items", data);

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::Locomotives, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!("V", item.epoch);
        assert_eq!(
            Some(String::from(
                "Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco, pantografi 52 Sommerfeldt"
            )),
            item.description_it
        );
        assert_eq!(Some(String::from("Motore a 5 poli")), item.details_it);
        assert_eq!(
            Some(String::from("Electric Locomotive E 402A 015")),
            item.description_en
        );
        assert_eq!(Some(String::from("5-poles motor")), item.details_en);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.first().expect("no saved rolling stock found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::Locomotive, rs.rolling_stock_category);
        assert_eq!(Some("rosso/bianco".to_string()), rs.livery);
        assert_eq!(Some(dec!(220)), rs.length_over_buffers_mm);
        assert_eq!(Some(dec!(8.66)), rs.length_over_buffers_in);
        assert_eq!("E402 A".to_string(), rs.type_name);
        assert_eq!(Some("E402 026".to_string()), rs.road_number);
        assert_eq!(Some("PRIMA SERIE".to_string()), rs.series);
        assert_eq!(Some("Milano Smistamento".to_string()), rs.depot);
        assert_eq!(Some(DccInterface::Mtc21), rs.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs.control);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(None, rs.freight_car_type);
        assert_eq!(Some(LocomotiveType::ElectricLocomotive), rs.locomotive_type);
        assert_eq!(None, rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(None, rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs.chassis);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.sprung_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_electric_multiple_unit() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, catalog_item_id);

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "ELECTRIC_MULTIPLE_UNITS",
            "scale" : "H0",
            "power_method" : "AC",
            "epoch": "V",
            "description" : {
                "en" : "Electric multiple unit Ale.540 013",
                "it" : "Elettromotrice Ale.540 013 e rimorchiata Le.760 003"
            },
            "details" : {
                "en" : "With internal lighting",
                "it" : "il modello è dotato di illuminazione interna di serie"
            },
            "delivery_date" : "2005",
            "availability_status" : "AVAILABLE",
            "count" : 2,
            "rolling_stocks": [{
                "category" : "ELECTRIC_MULTIPLE_UNIT",
                "type_name" : "ALe 540",
                "road_number" : "ALe 540 013",
                "series" : "SECONDA SERIE",
                "electric_multiple_unit_type" : "POWER_CAR",
                "railway" : "FS",
                "livery" : "castano/isabella",
                "depot" : "Milano Smistamento",
                "dcc_interface" : "PLUX_22",
                "control" : "DCC_READY",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "YES",
                  "lights" : "YES",
                  "sprung_buffers" : "NO"
                },
                "is_dummy" : false
              },
            {
                "category" : "ELECTRIC_MULTIPLE_UNIT",
                "type_name" : "Le 760",
                "road_number" : "Le 760 010",
                "series" : "SECONDA SERIE",
                "electric_multiple_unit_type" : "TRAILER_CAR",
                "railway" : "FS",
                "livery" : "castano/isabella",
                "control" : "NO_DCC",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "YES",
                  "lights" : "YES",
                  "sprung_buffers" : "NO"
                },
                "is_dummy" : true
              }]
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::ElectricMultipleUnits, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::AC, item.power_method);
        assert_eq!("V", item.epoch);
        assert_eq!(
            Some(String::from("Elettromotrice Ale.540 013 e rimorchiata Le.760 003")),
            item.description_it
        );
        assert_eq!(
            Some(String::from("il modello è dotato di illuminazione interna di serie")),
            item.details_it
        );
        assert_eq!(
            Some(String::from("Electric multiple unit Ale.540 013")),
            item.description_en
        );
        assert_eq!(Some(String::from("With internal lighting")), item.details_en);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(2, item.count);

        assert_eq!(2, saved.rolling_stocks.len());

        let rs1 = saved
            .rolling_stocks
            .first()
            .expect("the first rolling stock is not found");

        assert_ne!("", rs1.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs1.railway_id);
        assert_eq!(RollingStockCategory::ElectricMultipleUnit, rs1.rolling_stock_category);
        assert_eq!(Some("castano/isabella".to_string()), rs1.livery);
        assert_eq!(Some(dec!(310)), rs1.length_over_buffers_mm);
        assert_eq!(None, rs1.length_over_buffers_in);
        assert_eq!("ALe 540".to_string(), rs1.type_name);
        assert_eq!(Some("ALe 540 013".to_string()), rs1.road_number);
        assert_eq!(Some("SECONDA SERIE".to_string()), rs1.series);
        assert_eq!(Some("Milano Smistamento".to_string()), rs1.depot);
        assert_eq!(Some(DccInterface::Plux22), rs1.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs1.control);
        assert_eq!(
            Some(ElectricMultipleUnitType::PowerCar),
            rs1.electric_multiple_unit_type
        );
        assert_eq!(None, rs1.freight_car_type);
        assert_eq!(None, rs1.locomotive_type);
        assert_eq!(None, rs1.passenger_car_type);
        assert_eq!(None, rs1.railcar_type);
        assert_eq!(None, rs1.service_level);
        assert_eq!(Some(false), rs1.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs1.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs1.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs1.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs1.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs1.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs1.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs1.chassis);
        assert_eq!(Some(FeatureFlag::Yes), rs1.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs1.lights);
        assert_eq!(Some(FeatureFlag::No), rs1.sprung_buffers);

        let rs2 = saved
            .rolling_stocks
            .get(1)
            .expect("the second rolling stock is not found");

        assert_ne!("", rs2.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs2.railway_id);
        assert_eq!(RollingStockCategory::ElectricMultipleUnit, rs2.rolling_stock_category);
        assert_eq!(Some("castano/isabella".to_string()), rs2.livery);
        assert_eq!(Some(dec!(310)), rs2.length_over_buffers_mm);
        assert_eq!(None, rs2.length_over_buffers_in);
        assert_eq!("Le 760".to_string(), rs2.type_name);
        assert_eq!(Some("Le 760 010".to_string()), rs2.road_number);
        assert_eq!(Some("SECONDA SERIE".to_string()), rs2.series);
        assert_eq!(None, rs2.depot);
        assert_eq!(None, rs2.dcc_interface);
        assert_eq!(Some(Control::NoDcc), rs2.control);
        assert_eq!(
            Some(ElectricMultipleUnitType::TrailerCar),
            rs2.electric_multiple_unit_type
        );
        assert_eq!(None, rs2.freight_car_type);
        assert_eq!(None, rs2.locomotive_type);
        assert_eq!(None, rs2.passenger_car_type);
        assert_eq!(None, rs2.railcar_type);
        assert_eq!(None, rs2.service_level);
        assert_eq!(Some(true), rs2.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs2.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs2.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs2.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs2.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs2.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs2.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs2.chassis);
        assert_eq!(Some(FeatureFlag::Yes), rs2.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs2.lights);
        assert_eq!(Some(FeatureFlag::No), rs2.sprung_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_railcar() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, catalog_item_id);

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "RAILCARS",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "en" : "Railcar FS ALn 668",
                "it" : "Automotrice FS ALn 668"
            },
            "details" : {
                "en" : "Green/yellow",
                "it" : "Verde lichene/giallo coloniale con mantice frontale, motorizzata + folle"
            },
            "delivery_date" : "2008",
            "availability_status" : "AVAILABLE",
            "count" : 2,
            "rolling_stocks": [{
                "category" : "RAILCAR",
                "type_name" : "ALn 668",
                "road_number" : "ALn 668 1449",
                "series" : "SERIE 1400",
                "railcar_type" : "POWER_CAR",
                "railway" : "FS",
                "livery" : "verde lichene/giallo coloniale",
                "dcc_interface" : "PLUX_22",
                "control" : "DCC_READY",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "YES",
                  "sprung_buffers" : "NO"
                },
                "is_dummy" : false
              },
              {
                "category" : "RAILCAR",
                "type_name" : "ALn 668",
                "road_number" : "ALn 668 1456",
                "series" : "SERIE 1400",
                "railcar_type" : "POWER_CAR",
                "railway" : "FS",
                "livery" : "verde lichene/giallo coloniale",
                "control" : "NO_DCC",
                "length_over_buffers" : {
                  "millimeters" : 310.0,
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "YES",
                  "sprung_buffers" : "NO"
                },
                "is_dummy" : true
              }]
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::Railcars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!("V", item.epoch);
        assert_eq!(Some(String::from("Automotrice FS ALn 668")), item.description_it);
        assert_eq!(
            Some(String::from(
                "Verde lichene/giallo coloniale con mantice frontale, motorizzata + folle"
            )),
            item.details_it
        );
        assert_eq!(Some(String::from("Railcar FS ALn 668")), item.description_en);
        assert_eq!(Some(String::from("Green/yellow")), item.details_en);
        assert_eq!(Some(DeliveryDate::by_year(2008).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Available), item.availability_status);
        assert_eq!(2, item.count);

        assert_eq!(2, saved.rolling_stocks.len());

        let rs1 = saved
            .rolling_stocks
            .first()
            .expect("the first rolling stock is not found");

        assert_ne!("", rs1.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs1.railway_id);
        assert_eq!(RollingStockCategory::Railcar, rs1.rolling_stock_category);
        assert_eq!(Some("verde lichene/giallo coloniale".to_string()), rs1.livery);
        assert_eq!(Some(dec!(310)), rs1.length_over_buffers_mm);
        assert_eq!(None, rs1.length_over_buffers_in);
        assert_eq!("ALn 668".to_string(), rs1.type_name);
        assert_eq!(Some("ALn 668 1449".to_string()), rs1.road_number);
        assert_eq!(Some("SERIE 1400".to_string()), rs1.series);
        assert_eq!(None, rs1.depot);
        assert_eq!(Some(DccInterface::Plux22), rs1.dcc_interface);
        assert_eq!(Some(Control::DccReady), rs1.control);
        assert_eq!(None, rs1.electric_multiple_unit_type);
        assert_eq!(None, rs1.freight_car_type);
        assert_eq!(None, rs1.locomotive_type);
        assert_eq!(None, rs1.passenger_car_type);
        assert_eq!(Some(RailcarType::PowerCar), rs1.railcar_type);
        assert_eq!(None, rs1.service_level);
        assert_eq!(Some(false), rs1.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs1.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs1.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs1.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs1.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs1.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs1.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs1.chassis);
        assert_eq!(Some(FeatureFlag::No), rs1.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs1.lights);
        assert_eq!(Some(FeatureFlag::No), rs1.sprung_buffers);

        let rs2 = saved
            .rolling_stocks
            .get(1)
            .expect("the second rolling stock is not found");

        assert_ne!("", rs2.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs2.railway_id);
        assert_eq!(RollingStockCategory::Railcar, rs2.rolling_stock_category);
        assert_eq!(Some("verde lichene/giallo coloniale".to_string()), rs2.livery);
        assert_eq!(Some(dec!(310)), rs2.length_over_buffers_mm);
        assert_eq!(None, rs2.length_over_buffers_in);
        assert_eq!("ALn 668".to_string(), rs2.type_name);
        assert_eq!(Some("ALn 668 1456".to_string()), rs2.road_number);
        assert_eq!(Some("SERIE 1400".to_string()), rs2.series);
        assert_eq!(None, rs2.depot);
        assert_eq!(None, rs2.dcc_interface);
        assert_eq!(Some(Control::NoDcc), rs2.control);
        assert_eq!(None, rs2.electric_multiple_unit_type);
        assert_eq!(None, rs2.freight_car_type);
        assert_eq!(None, rs2.locomotive_type);
        assert_eq!(None, rs2.passenger_car_type);
        assert_eq!(Some(RailcarType::PowerCar), rs2.railcar_type);
        assert_eq!(None, rs2.service_level);
        assert_eq!(Some(true), rs2.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs2.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs2.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs2.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs2.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs2.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs2.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs2.chassis);
        assert_eq!(Some(FeatureFlag::No), rs2.interior_lights);
        assert_eq!(Some(FeatureFlag::Yes), rs2.lights);
        assert_eq!(Some(FeatureFlag::No), rs2.sprung_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_passenger_car() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, catalog_item_id);

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "PASSENGER_CARS",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "en" : "Passenger car",
                "it" : "Carrozza passeggeri"
            },
            "details" : {
                "en" : "Golden doors",
                "it" : "porte dorate, carrelli MD50"
            },
            "delivery_date" : "2005",
            "availability_status" : "ANNOUNCED",
            "count" : 1,
            "rolling_stocks": [{
                "category" : "PASSENGER_CAR",
                "type_name" : "UIC-X",
                "road_number" : "50 83 10-88 076-2 A",
                "series" : "TIPO 1964",
                "passenger_car_type" : "COMPARTMENT_COACH",
                "railway" : "FS",
                "livery" : "rosso fegato/grigio beige",
                "length_over_buffers" : {
                  "millimeters" : 303.0
                },
                "service_level" : "FIRST_CLASS",
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "NO",
                  "sprung_buffers" : "NO"
                }
              }]
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::PassengerCars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!("V", item.epoch);
        assert_eq!(Some(String::from("Carrozza passeggeri")), item.description_it);
        assert_eq!(Some(String::from("porte dorate, carrelli MD50")), item.details_it);
        assert_eq!(Some(String::from("Passenger car")), item.description_en);
        assert_eq!(Some(String::from("Golden doors")), item.details_en);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Announced), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.first().expect("the rolling stock is not found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::PassengerCar, rs.rolling_stock_category);
        assert_eq!(Some("rosso fegato/grigio beige".to_string()), rs.livery);
        assert_eq!(Some(dec!(303)), rs.length_over_buffers_mm);
        assert_eq!(None, rs.length_over_buffers_in);
        assert_eq!("UIC-X".to_string(), rs.type_name);
        assert_eq!(Some("50 83 10-88 076-2 A".to_string()), rs.road_number);
        assert_eq!(Some("TIPO 1964".to_string()), rs.series);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(None, rs.freight_car_type);
        assert_eq!(None, rs.locomotive_type);
        assert_eq!(Some(PassengerCarType::CompartmentCoach), rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(Some(ServiceLevel::FirstClass), rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs.chassis);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::No), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.sprung_buffers);
    })
    .await
}

#[tokio::test]
async fn it_should_create_a_new_freight_car() {
    let test = create_docker_test();

    test.run_async(|ops| async move {
        let (_, port) = ops.handle(IMAGE_NAME).host_port(5432).unwrap();

        let sut = spawn_app(*port).await;
        sut.run_database_migrations().await;

        let pg_pool = sut.pg_pool();
        seed_brands(&pg_pool).await;
        seed_railways(&pg_pool).await;
        seed_scales(&pg_pool).await;

        let catalog_item_id = CatalogItemId::from_str("acme-123456").unwrap();
        let expected_location = format!("{}/{}", API_CATALOG_ITEMS, catalog_item_id);

        let request = json!({
            "brand" : "ACME",
            "item_number" : "123456",
            "category" : "FREIGHT_CARS",
            "scale" : "H0",
            "power_method" : "DC",
            "epoch": "V",
            "description" : {
                "en" : "Freight car type Hbbillns",
                "it" : "Carro FS Hbbillns coperto livrea livrea XMPR grigio/verde"
            },
            "details" : {
                "en" : "Some details go here",
                "it" : "Alcuni dettagli"
            },
            "delivery_date" : "2005",
            "availability_status" : "ANNOUNCED",
            "count" : 1,
            "rolling_stocks": [{
                "category" : "FREIGHT_CAR",
                "type_name" : "Hbbillns",
                "road_number" : "21 83 245 7 266-6 Hbbillns",
                "freight_car_type" : "SLIDING_WALL_BOXCARS",
                "railway" : "FS",
                "livery" : "XMPR",
                "length_over_buffers" : {
                  "millimeters" : 180.0
                },
                "technical_specifications" : {
                  "coupling" : {
                    "socket" : "NEM_362",
                    "close_couplers" : "YES",
                    "digital_shunting" : "NO"
                  },
                  "flywheel_fitted" : "NO",
                  "body_shell" : "PLASTIC",
                  "chassis" : "METAL_DIE_CAST",
                  "minimum_radius": 360.0,
                  "interior_lights" : "NO",
                  "lights" : "NO",
                  "sprung_buffers" : "NO"
                }
              }]
        });

        let client = reqwest::Client::new();
        let endpoint = sut.endpoint(API_CATALOG_ITEMS);
        let response = client
            .post(endpoint)
            .json(&request)
            .send()
            .await
            .expect("Failed to execute request.");

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
        assert_eq!(expected_location, response.headers()["Location"]);

        let saved = fetch_saved_catalog_item(catalog_item_id.clone(), &pg_pool).await;
        let item = saved.catalog_item;

        assert_eq!(BrandId::new("ACME"), item.brand_id);
        assert_eq!("123456", item.item_number);
        assert_eq!(Category::FreightCars, item.category);
        assert_eq!(ScaleId::new("H0"), item.scale_id);
        assert_eq!(PowerMethod::DC, item.power_method);
        assert_eq!("V", item.epoch);
        assert_eq!(
            Some(String::from(
                "Carro FS Hbbillns coperto livrea livrea XMPR grigio/verde"
            )),
            item.description_it
        );
        assert_eq!(Some(String::from("Alcuni dettagli")), item.details_it);
        assert_eq!(Some(String::from("Freight car type Hbbillns")), item.description_en);
        assert_eq!(Some(String::from("Some details go here")), item.details_en);
        assert_eq!(Some(DeliveryDate::by_year(2005).to_string()), item.delivery_date);
        assert_eq!(Some(AvailabilityStatus::Announced), item.availability_status);
        assert_eq!(1, item.count);

        let rs = saved.rolling_stocks.first().expect("the rolling stock is not found");

        assert_ne!("", rs.rolling_stock_id.to_string());
        assert_eq!(RailwayId::new("FS"), rs.railway_id);
        assert_eq!(RollingStockCategory::FreightCar, rs.rolling_stock_category);
        assert_eq!(Some("XMPR".to_string()), rs.livery);
        assert_eq!(Some(dec!(180)), rs.length_over_buffers_mm);
        assert_eq!(None, rs.length_over_buffers_in);
        assert_eq!("Hbbillns".to_string(), rs.type_name);
        assert_eq!(Some("21 83 245 7 266-6 Hbbillns".to_string()), rs.road_number);
        assert_eq!(None, rs.series);
        assert_eq!(None, rs.electric_multiple_unit_type);
        assert_eq!(Some(FreightCarType::SlidingWallBoxcars), rs.freight_car_type);
        assert_eq!(None, rs.locomotive_type);
        assert_eq!(None, rs.passenger_car_type);
        assert_eq!(None, rs.railcar_type);
        assert_eq!(None, rs.service_level);
        assert_eq!(Some(false), rs.is_dummy);
        assert_eq!(Some(dec!(360.0)), rs.minimum_radius);
        assert_eq!(Some(CouplingSocket::Nem362), rs.coupling_socket);
        assert_eq!(Some(FeatureFlag::Yes), rs.close_couplers);
        assert_eq!(Some(FeatureFlag::No), rs.digital_shunting_coupling);
        assert_eq!(Some(FeatureFlag::No), rs.flywheel_fitted);
        assert_eq!(Some(BodyShellType::Plastic), rs.body_shell);
        assert_eq!(Some(ChassisType::MetalDieCast), rs.chassis);
        assert_eq!(Some(FeatureFlag::No), rs.interior_lights);
        assert_eq!(Some(FeatureFlag::No), rs.lights);
        assert_eq!(Some(FeatureFlag::No), rs.sprung_buffers);
    })
    .await
}

async fn fetch_saved_catalog_item(catalog_item_id: CatalogItemId, pg_pool: &PgPool) -> Saved {
    let catalog_item = sqlx::query_as!(
        SavedCatalogItem,
        r#"SELECT
            item_number,
            brand_id as "brand_id: BrandId",
            scale_id as "scale_id: ScaleId",
            category as "category: Category",
            power_method as "power_method: PowerMethod",
            epoch,
            description_en,
            description_it,
            details_en,
            details_it,
            delivery_date,
            availability_status as "availability_status: AvailabilityStatus",
            count
        FROM catalog_items WHERE catalog_item_id = $1"#,
        catalog_item_id.clone() as CatalogItemId
    )
    .fetch_one(pg_pool)
    .await
    .expect("Failed to fetch saved catalog item.");

    let rolling_stocks = sqlx::query_as!(
        SavedRollingStock,
        r#"SELECT 
            rolling_stock_id as "rolling_stock_id: RollingStockId",
            railway_id as "railway_id: RailwayId",
            rolling_stock_category as "rolling_stock_category: RollingStockCategory",
            livery,
            length_over_buffers_mm,
            length_over_buffers_in,
            type_name,
            road_number,
            series,
            depot,
            dcc_interface as "dcc_interface: DccInterface",
            control as "control: Control",
            electric_multiple_unit_type as "electric_multiple_unit_type: ElectricMultipleUnitType",
            freight_car_type as "freight_car_type: FreightCarType",
            locomotive_type as "locomotive_type: LocomotiveType",
            passenger_car_type as "passenger_car_type: PassengerCarType",
            railcar_type as "railcar_type: RailcarType",
            service_level as "service_level: ServiceLevel",
            is_dummy,
            minimum_radius,
            coupling_socket as "coupling_socket: CouplingSocket",
            close_couplers as "close_couplers: FeatureFlag",
            digital_shunting_coupling as "digital_shunting_coupling: FeatureFlag",
            flywheel_fitted as "flywheel_fitted: FeatureFlag",
            body_shell as "body_shell: BodyShellType",
            chassis as "chassis: ChassisType",
            interior_lights as "interior_lights: FeatureFlag",
            lights as "lights: FeatureFlag",
            sprung_buffers as "sprung_buffers: FeatureFlag"
        FROM rolling_stocks
        WHERE catalog_item_id = $1"#,
        catalog_item_id as CatalogItemId
    )
    .fetch_all(pg_pool)
    .await
    .expect("Failed to fetch saved rolling stock(s).");

    Saved {
        catalog_item,
        rolling_stocks,
    }
}

#[derive(Debug)]
struct Saved {
    catalog_item: SavedCatalogItem,
    rolling_stocks: Vec<SavedRollingStock>,
}

#[derive(Debug)]
struct SavedCatalogItem {
    brand_id: BrandId,
    item_number: String,
    category: Category,
    scale_id: ScaleId,
    power_method: PowerMethod,
    epoch: String,
    description_en: Option<String>,
    description_it: Option<String>,
    details_en: Option<String>,
    details_it: Option<String>,
    delivery_date: Option<String>,
    availability_status: Option<AvailabilityStatus>,
    count: i32,
}

#[derive(Debug)]
struct SavedRollingStock {
    rolling_stock_id: RollingStockId,
    railway_id: RailwayId,
    rolling_stock_category: RollingStockCategory,
    livery: Option<String>,
    length_over_buffers_mm: Option<Decimal>,
    length_over_buffers_in: Option<Decimal>,
    type_name: String,
    road_number: Option<String>,
    series: Option<String>,
    depot: Option<String>,
    dcc_interface: Option<DccInterface>,
    control: Option<Control>,
    electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    freight_car_type: Option<FreightCarType>,
    locomotive_type: Option<LocomotiveType>,
    passenger_car_type: Option<PassengerCarType>,
    railcar_type: Option<RailcarType>,
    service_level: Option<ServiceLevel>,
    is_dummy: Option<bool>,
    minimum_radius: Option<Decimal>,
    coupling_socket: Option<CouplingSocket>,
    close_couplers: Option<FeatureFlag>,
    digital_shunting_coupling: Option<FeatureFlag>,
    flywheel_fitted: Option<FeatureFlag>,
    body_shell: Option<BodyShellType>,
    chassis: Option<ChassisType>,
    interior_lights: Option<FeatureFlag>,
    lights: Option<FeatureFlag>,
    sprung_buffers: Option<FeatureFlag>,
}
