use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::catalog_item::{CatalogItem, CatalogItemManufacturer, CatalogItemScale};
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::category::{Category, LocomotiveType};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::length_over_buffers::LengthOverBuffers;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock::{RollingStock, RollingStockRailway};
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::railways::railway_id::RailwayId;
use crate::scales::scale_id::ScaleId;
use chrono::prelude::*;
use common::length::Length;
use common::metadata::Metadata;
use rust_decimal_macros::dec;

pub fn acme() -> CatalogItemManufacturer {
    CatalogItemManufacturer::new(ManufacturerId::new("acme"), "ACME")
}

pub fn piko() -> CatalogItemManufacturer {
    CatalogItemManufacturer::new(ManufacturerId::new("piko"), "Piko")
}

pub fn roco() -> CatalogItemManufacturer {
    CatalogItemManufacturer::new(ManufacturerId::new("roco"), "Roco")
}

#[allow(non_snake_case)]
pub fn H0() -> CatalogItemScale {
    CatalogItemScale::new(ScaleId::new("h0"), "H0 (1:87)")
}

pub fn fs() -> RollingStockRailway {
    RollingStockRailway::new(RailwayId::new("fs"), "FS")
}

#[allow(non_snake_case)]
pub fn ACME_60142() -> CatalogItem {
    let manufacturer = acme();
    let item_number = ItemNumber::new("60142");

    let id = CatalogItemId::new(manufacturer.clone(), item_number.clone()); //TODO: fix me

    let rolling_stocks = vec![RollingStock::new_locomotive(
        RollingStockId::new(),
        "E645",
        "E645 019",
        Some("PRIMA SERIE (ex E646)"),
        fs(),
        LocomotiveType::ElectricLocomotive,
        Some("Ancona"),
        Some("castano/isabella"),
        false,
        Some(LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(210)))),
        Some(Control::DccReady),
        Some(DccInterface::Nem652),
        None,
    )];

    CatalogItem::new(
        id,
        manufacturer,
        item_number,
        Category::Locomotives,
        H0(),
        Some("Locomotiva elettrica FS E645 019 di prima serie"),
        None,
        rolling_stocks,
        PowerMethod::DC,
        Epoch::IV,
        Some(DeliveryDate::ByYear(2013)),
        Some(AvailabilityStatus::Available),
        1,
        metadata(),
    )
}

#[allow(non_snake_case)]
pub fn Piko_52848() -> CatalogItem {
    let manufacturer = piko();
    let item_number = ItemNumber::new("52848");

    let id = CatalogItemId::new(manufacturer.clone(), item_number.clone()); //TODO: fix me

    let rolling_stocks = vec![RollingStock::new_locomotive(
        RollingStockId::new(),
        "D145",
        "D145 2004",
        Some("Serie 2000 TIBB"),
        fs(),
        LocomotiveType::DieselLocomotive,
        Some("Genova Rivarolo"),
        Some("arancio, fasce gialle"),
        false,
        Some(LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(175)))),
        Some(Control::DccSound),
        Some(DccInterface::Mtc21),
        None,
    )];

    CatalogItem::new(
        id,
        manufacturer,
        item_number,
        Category::Locomotives,
        H0(),
        Some("Locomotiva diesel FS D145 2004"),
        None,
        rolling_stocks,
        PowerMethod::DC,
        Epoch::IV,
        Some(DeliveryDate::ByYear(2020)),
        Some(AvailabilityStatus::Available),
        1,
        metadata(),
    )
}

fn metadata() -> Metadata {
    let created_date = Utc.with_ymd_and_hms(2022, 12, 26, 23, 58, 0).unwrap();
    Metadata::created_at(created_date)
}
