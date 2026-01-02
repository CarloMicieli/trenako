use anyhow::anyhow;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::delivery_date::DeliveryDate;
use catalog::catalog_items::epoch::Epoch;
use catalog::catalog_items::item_number::ItemNumber;
use catalog::catalog_items::length_over_buffers::LengthOverBuffers;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{
    BodyShellType, ChassisType, Coupling, CouplingSocket, FeatureFlag, Radius, TechnicalSpecifications,
};
use common::length::Length;
use common::localized_text::LocalizedText;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CsvRecord {
    pub brand: String,
    pub item_number: Option<ItemNumber>,
    pub scale: String,
    pub power_method: Option<PowerMethod>,
    pub epoch: Option<Epoch>,
    pub description: String,
    pub details: String,
    pub delivery_date: Option<DeliveryDate>,
    pub availability: Option<AvailabilityStatus>,
    pub count: Option<i32>,
    pub is_dummy: bool,
    pub category: Option<RollingStockCategory>,
    pub subcategory: String,
    pub railway: String,
    pub type_name: String,
    pub series: Option<String>,
    pub road_number: Option<String>,
    pub control: Option<Control>,
    pub dcc_interface: Option<DccInterface>,
    pub length: Option<u16>,
    pub livery: Option<String>,
    pub depot: Option<String>,
    pub couplers: Option<CouplingSocket>,
    pub flywheel_fitted: Option<FeatureFlag>,
    pub chassis: Option<ChassisType>,
    pub body_shell: Option<BodyShellType>,
    pub interior_lights: Option<FeatureFlag>,
    pub lights: Option<FeatureFlag>,
    pub sprung_buffers: Option<FeatureFlag>,
    pub minimum_radius: Option<u16>,
    pub service_level: Option<ServiceLevel>,
}

impl TryInto<CatalogItemRequest> for CsvRecord {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CatalogItemRequest, Self::Error> {
        let catalog_item_request = CatalogItemRequest {
            brand: self.brand,
            item_number: self.item_number.expect("the item number is required"),
            scale: self.scale,
            category: category_item_category(self.category.expect("the category is required")),
            power_method: self.power_method.expect("the power method is required"),
            epoch: self.epoch.expect("the epoch is required"),
            description: LocalizedText::with_italian(&self.description),
            details: LocalizedText::with_italian(&self.details),
            delivery_date: self.delivery_date,
            availability_status: self.availability,
            count: self.count.expect("the rolling stocks count is required"),
            rolling_stocks: vec![],
        };

        Ok(catalog_item_request)
    }
}

fn category_item_category(cat: RollingStockCategory) -> Category {
    match cat {
        RollingStockCategory::Locomotive => Category::Locomotives,
        RollingStockCategory::PassengerCar => Category::PassengerCars,
        RollingStockCategory::FreightCar => Category::FreightCars,
        RollingStockCategory::Railcar => Category::Railcars,
        RollingStockCategory::ElectricMultipleUnit => Category::ElectricMultipleUnits,
    }
}

impl TryInto<RollingStockRequest> for CsvRecord {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RollingStockRequest, Self::Error> {
        let tech_specs = technical_specification(&self)?;
        let length = if let Some(value) = self.length {
            let length = to_length_over_buffer(value)?;
            Some(length)
        } else {
            None
        };

        let category = self.category.ok_or_else(|| anyhow!("the category is required"))?;

        let rs = match category {
            RollingStockCategory::Locomotive => RollingStockRequest::LocomotiveRequest {
                railway: self.railway,
                livery: self.livery,
                length_over_buffers: length,
                technical_specifications: tech_specs,
                class_name: self.type_name,
                road_number: self
                    .road_number
                    .ok_or_else(|| anyhow!("missing road number for the locomotive"))?,
                series: self.series,
                depot: self.depot,
                locomotive_type: LocomotiveType::from_str(&self.subcategory)?,
                dcc_interface: self.dcc_interface,
                control: self.control,
                is_dummy: self.is_dummy,
            },
            RollingStockCategory::PassengerCar => RollingStockRequest::PassengerCarRequest {
                railway: self.railway,
                livery: self.livery,
                length_over_buffers: length,
                technical_specifications: tech_specs,
                type_name: self.type_name,
                road_number: self.road_number,
                series: self.series,
                passenger_car_type: PassengerCarType::from_str(&self.subcategory).ok(),
                service_level: self.service_level,
            },
            RollingStockCategory::FreightCar => RollingStockRequest::FreightCarRequest {
                railway: self.railway,
                livery: self.livery,
                length_over_buffers: length,
                technical_specifications: tech_specs,
                type_name: self.type_name,
                road_number: self.road_number,
                freight_car_type: FreightCarType::from_str(&self.subcategory).ok(),
            },
            RollingStockCategory::Railcar => RollingStockRequest::RailcarRequest {
                railway: self.railway,
                livery: self.livery,
                length_over_buffers: length,
                technical_specifications: tech_specs,
                type_name: self.type_name,
                road_number: self.road_number,
                series: self.series,
                depot: self.depot,
                railcar_type: RailcarType::from_str(&self.subcategory)
                    .map_err(|_| anyhow!("railcar type is required"))?,
                dcc_interface: self.dcc_interface,
                control: self.control,
                is_dummy: self.is_dummy,
            },
            RollingStockCategory::ElectricMultipleUnit => RollingStockRequest::ElectricMultipleUnitRequest {
                railway: self.railway,
                livery: self.livery,
                length_over_buffers: length,
                technical_specifications: tech_specs,
                type_name: self.type_name,
                road_number: self.road_number,
                series: self.series,
                depot: self.depot,
                electric_multiple_unit_type: ElectricMultipleUnitType::from_str(&self.subcategory)
                    .map_err(|_| anyhow!("electric multiple unit type is required"))?,
                dcc_interface: self.dcc_interface,
                control: self.control,
                is_dummy: self.is_dummy,
            },
        };
        Ok(rs)
    }
}

fn to_length_over_buffer(value: u16) -> Result<LengthOverBuffers, anyhow::Error> {
    let value = Decimal::new(value as i64, 0);
    let length = Length::try_new(value, MeasureUnit::Millimeters)?;
    Ok(LengthOverBuffers::from_millimeters(length))
}

fn technical_specification(record: &CsvRecord) -> Result<Option<TechnicalSpecifications>, anyhow::Error> {
    let minimum_radius = if let Some(radius) = record.minimum_radius {
        let min_radius = radius_from_u16(radius)?;
        Some(min_radius)
    } else {
        None
    };

    let tech_specs = TechnicalSpecifications {
        minimum_radius,
        coupling: record.couplers.map(|c| coupling(&c)),
        flywheel_fitted: record.flywheel_fitted,
        body_shell: record.body_shell,
        chassis: record.chassis,
        interior_lights: record.interior_lights,
        lights: record.lights,
        sprung_buffers: record.sprung_buffers,
    };
    Ok(Some(tech_specs))
}

fn radius_from_u16(input: u16) -> Result<Radius, anyhow::Error> {
    Radius::from_millimeters(Decimal::new(input as i64, 0)).map_err(|_| anyhow!("invalid value for a minimum radius"))
}

fn coupling(socket: &CouplingSocket) -> Coupling {
    match socket {
        CouplingSocket::None => Coupling {
            close_couplers: Some(FeatureFlag::No),
            digital_shunting: Some(FeatureFlag::No),
            socket: Some(CouplingSocket::None),
        },
        _ => Coupling {
            close_couplers: Some(FeatureFlag::Yes),
            digital_shunting: Some(FeatureFlag::No),
            socket: Some(*socket),
        },
    }
}
