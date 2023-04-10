use crate::catalog_items::catalog_item::{CatalogItem, CatalogItemBrand, CatalogItemScale};
use crate::catalog_items::category::RollingStockCategory;
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::length_over_buffers::LengthOverBuffers;
use crate::catalog_items::queries::catalog_item_row::CatalogItemRow;
use crate::catalog_items::queries::rolling_stock_row::RollingStockRow;
use crate::catalog_items::rolling_stock::{RollingStock, RollingStockRailway};
use crate::catalog_items::technical_specifications::{Coupling, Radius, TechnicalSpecifications};
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::queries::converters::{ConversionErrors, Converter, OptionConverter, ToOutputConverter};
use std::str::FromStr;

impl ToOutputConverter<CatalogItem> for CatalogItemRow {
    fn to_output(self) -> Result<CatalogItem, ConversionErrors> {
        let row = self;

        let item_number = ItemNumber::try_convert(&row)?;
        let description = try_convert_description(&row)?;
        let details = try_convert_details(&row)?;
        let delivery_date = DeliveryDate::try_convert(&row)?;
        let metadata = Metadata::try_convert(&row)?;

        Ok(CatalogItem {
            catalog_item_id: row.catalog_item_id,
            brand: CatalogItemBrand {
                brand_id: row.brand_id,
                display: row.brand_display,
            },
            item_number,
            scale: CatalogItemScale {
                scale_id: row.scale_id,
                display: row.scale_display,
            },
            category: row.category,
            power_method: row.power_method,
            description,
            details,
            delivery_date,
            availability_status: row.availability_status,
            rolling_stocks: vec![],
            count: row.count as u8,
            metadata,
        })
    }
}

fn try_convert_description(value: &CatalogItemRow) -> Result<LocalizedText, ConversionErrors> {
    let mut localized_text = LocalizedText::default();

    localized_text.add_english(value.description_en.as_ref());
    localized_text.add_italian(value.description_it.as_ref());

    Ok(localized_text)
}

fn try_convert_details(value: &CatalogItemRow) -> Result<LocalizedText, ConversionErrors> {
    let mut localized_text = LocalizedText::default();

    localized_text.add_english(value.details_en.as_ref());
    localized_text.add_italian(value.details_it.as_ref());

    Ok(localized_text)
}

impl OptionConverter<CatalogItemRow> for DeliveryDate {
    fn try_convert(row: &CatalogItemRow) -> Result<Option<Self>, ConversionErrors> {
        if let Some(dd) = &row.delivery_date {
            DeliveryDate::from_str(dd)
                .map(Some)
                .map_err(|_| ConversionErrors::new())
        } else {
            Ok(None)
        }
    }
}

impl Converter<CatalogItemRow> for ItemNumber {
    fn try_convert(row: &CatalogItemRow) -> Result<Self, ConversionErrors> {
        Ok(ItemNumber::new(&row.item_number))
    }
}

impl Converter<CatalogItemRow> for Metadata {
    fn try_convert(value: &CatalogItemRow) -> Result<Self, ConversionErrors> {
        Ok(Metadata::new(
            value.version as u8,
            value.created_at,
            value.last_modified_at,
        ))
    }
}

impl ToOutputConverter<RollingStock> for RollingStockRow {
    fn to_output(self) -> Result<RollingStock, ConversionErrors> {
        let row = self;

        let epoch = Epoch::try_convert(&row)?;
        let technical_specifications = TechnicalSpecifications::try_convert(&row)?;
        let length_over_buffer = LengthOverBuffers::try_convert(&row)?;

        match row.rolling_stock_category {
            RollingStockCategory::Locomotive => Ok(RollingStock::Locomotive {
                id: row.rolling_stock_id,
                railway: RollingStockRailway {
                    railway_id: row.railway_id,
                    display: row.railway_label,
                },
                epoch,
                livery: row.livery,
                length_over_buffer,
                technical_specifications,
                class_name: row.type_name.expect("missing class name"),
                road_number: row.road_number.expect("missing road number"),
                series: row.series,
                depot: row.depot,
                locomotive_type: row.locomotive_type.expect("missing locomotive type"),
                dcc_interface: row.dcc_interface,
                control: row.control,
                is_dummy: row.is_dummy.unwrap_or(false),
            }),
            RollingStockCategory::PassengerCar => Ok(RollingStock::PassengerCar {
                id: row.rolling_stock_id,
                railway: RollingStockRailway {
                    railway_id: row.railway_id,
                    display: row.railway_label,
                },
                epoch,
                livery: row.livery,
                length_over_buffer,
                technical_specifications,
                type_name: row.type_name.expect("missing type name"),
                road_number: row.road_number,
                series: row.series,
                passenger_car_type: row.passenger_car_type,
                service_level: row.service_level,
            }),
            RollingStockCategory::FreightCar => Ok(RollingStock::FreightCar {
                id: row.rolling_stock_id,
                railway: RollingStockRailway {
                    railway_id: row.railway_id,
                    display: row.railway_label,
                },
                epoch,
                livery: row.livery,
                length_over_buffer,
                technical_specifications,
                type_name: row.type_name.expect("missing type name"),
                road_number: row.road_number,
                freight_car_type: row.freight_car_type,
            }),
            RollingStockCategory::ElectricMultipleUnit => Ok(RollingStock::ElectricMultipleUnit {
                id: row.rolling_stock_id,
                railway: RollingStockRailway {
                    railway_id: row.railway_id,
                    display: row.railway_label,
                },
                epoch,
                livery: row.livery,
                length_over_buffer,
                technical_specifications,
                type_name: row.type_name.expect("missing type name"),
                road_number: row.road_number,
                series: row.series,
                depot: row.depot,
                electric_multiple_unit_type: row
                    .electric_multiple_unit_type
                    .expect("missing electric multiple unit type"),
                dcc_interface: row.dcc_interface,
                control: row.control,
                is_dummy: row.is_dummy.unwrap_or(false),
            }),
            RollingStockCategory::Railcar => Ok(RollingStock::Railcar {
                id: row.rolling_stock_id,
                railway: RollingStockRailway {
                    railway_id: row.railway_id,
                    display: row.railway_label,
                },
                epoch,
                livery: row.livery,
                length_over_buffer,
                technical_specifications,
                type_name: row.type_name.expect("missing type name"),
                road_number: row.road_number,
                series: row.series,
                depot: row.depot,
                railcar_type: row.railcar_type.expect("missing railcar type"),
                dcc_interface: row.dcc_interface,
                control: row.control,
                is_dummy: row.is_dummy.unwrap_or(false),
            }),
        }
    }
}

impl Converter<RollingStockRow> for Epoch {
    fn try_convert(row: &RollingStockRow) -> Result<Self, ConversionErrors> {
        Epoch::from_str(&row.epoch).map_err(|_| ConversionErrors::new())
    }
}

impl OptionConverter<RollingStockRow> for TechnicalSpecifications {
    fn try_convert(row: &RollingStockRow) -> Result<Option<Self>, ConversionErrors> {
        let minimum_radius = row
            .minimum_radius
            .map(Radius::from_millimeters)
            .transpose()
            .map_err(|_| ConversionErrors::new())?;

        Ok(Some(TechnicalSpecifications {
            minimum_radius,
            coupling: Some(Coupling {
                close_couplers: row.close_couplers.unwrap_or_default(),
                socket: row.coupling_socket.unwrap_or_default(),
                digital_shunting: row.digital_shunting_coupling.unwrap_or_default(),
            }),
            flywheel_fitted: row.flywheel_fitted.unwrap_or_default(),
            metal_body: row.metal_body.unwrap_or_default(),
            interior_lights: row.interior_lights.unwrap_or_default(),
            lights: row.lights.unwrap_or_default(),
            spring_buffers: row.spring_buffers.unwrap_or_default(),
        }))
    }
}

impl OptionConverter<RollingStockRow> for LengthOverBuffers {
    fn try_convert(row: &RollingStockRow) -> Result<Option<Self>, ConversionErrors> {
        LengthOverBuffers::new(row.length_over_buffers_in, row.length_over_buffers_mm)
            .map(Some)
            .map_err(|_| ConversionErrors::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_converter {
        use super::*;
        use crate::brands::brand_id::BrandId;
        use crate::catalog_items::availability_status::AvailabilityStatus;
        use crate::catalog_items::catalog_item_id::CatalogItemId;
        use crate::catalog_items::category::Category;
        use crate::catalog_items::power_method::PowerMethod;
        use crate::catalog_items::queries::catalog_item_row::test::new_catalog_item_row;
        use crate::scales::scale_id::ScaleId;
        use chrono::Utc;
        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        #[test]
        fn it_should_convert_catalog_item_id() {
            let row = CatalogItemRow { ..default_row() };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(
                catalog_item.catalog_item_id,
                CatalogItemId::from_str("acme-123456").unwrap()
            );
        }

        #[test]
        fn it_should_convert_catalog_item_number() {
            let row = CatalogItemRow { ..default_row() };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(catalog_item.item_number, ItemNumber::new("123456"));
        }

        #[test]
        fn it_should_convert_catalog_item_brand() {
            let row = CatalogItemRow { ..default_row() };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(
                catalog_item.brand,
                CatalogItemBrand {
                    brand_id: BrandId::new("ACME"),
                    display: String::from("ACME"),
                }
            );
        }

        #[test]
        fn it_should_convert_catalog_item_scale() {
            let row = CatalogItemRow { ..default_row() };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(
                catalog_item.scale,
                CatalogItemScale {
                    scale_id: ScaleId::new("H0"),
                    display: String::from("H0"),
                }
            );
        }

        #[test]
        fn it_should_convert_catalog_item_category() {
            let row = CatalogItemRow {
                category: Category::ElectricMultipleUnits,
                ..default_row()
            };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(catalog_item.category, Category::ElectricMultipleUnits);
        }

        #[test]
        fn it_should_convert_catalog_item_power_method() {
            let row = CatalogItemRow {
                power_method: PowerMethod::DC,
                ..default_row()
            };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(catalog_item.power_method, PowerMethod::DC);
        }

        #[test]
        fn it_should_convert_catalog_item_availability_status() {
            let row = CatalogItemRow {
                availability_status: Some(AvailabilityStatus::Announced),
                ..default_row()
            };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(catalog_item.availability_status, Some(AvailabilityStatus::Announced));
        }

        #[test]
        fn it_should_convert_catalog_item_delivery_date() {
            let row = CatalogItemRow {
                delivery_date: Some(String::from("2022/Q1")),
                ..default_row()
            };

            let catalog_item = row.to_output().expect("the catalog item conversion failed");

            assert_eq!(catalog_item.delivery_date, Some(DeliveryDate::by_quarter(2022, 1)));
        }

        fn default_row() -> CatalogItemRow {
            new_catalog_item_row("ACME", "123456", "H0", Utc::now())
        }
    }

    mod rolling_stock_converter {
        use super::*;
        use crate::catalog_items::catalog_item_id::CatalogItemId;
        use crate::catalog_items::category::{
            ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
        };
        use crate::catalog_items::control::{Control, DccInterface};
        use crate::catalog_items::queries::rolling_stock_row::test::new_rolling_stock_row;
        use crate::catalog_items::technical_specifications::{CouplingSocket, FeatureFlag};
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;
        use std::str::FromStr;

        #[test]
        fn it_should_convert_a_locomotive_rolling_stock() {
            let row = RollingStockRow {
                rolling_stock_category: RollingStockCategory::Locomotive,
                locomotive_type: Some(LocomotiveType::ElectricLocomotive),
                livery: Some(String::from("blue")),
                length_over_buffers_mm: Some(dec!(16.5)),
                length_over_buffers_in: Some(dec!(0.65)),
                type_name: Some(String::from("Group 1")),
                road_number: Some(String::from("Number 42")),
                depot: Some(String::from("Depot")),
                series: Some(String::from("prototype")),
                dcc_interface: Some(DccInterface::Mtc21),
                control: Some(Control::DccReady),
                minimum_radius: Some(dec!(360)),
                close_couplers: Some(FeatureFlag::Yes),
                lights: Some(FeatureFlag::Yes),
                coupling_socket: Some(CouplingSocket::Nem362),
                ..default_row()
            };

            let rolling_stock = row.clone().to_output().expect("the rolling stock conversion failed");

            match rolling_stock {
                RollingStock::Locomotive {
                    id,
                    railway,
                    epoch,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    class_name,
                    road_number,
                    series,
                    depot,
                    locomotive_type,
                    dcc_interface,
                    control,
                    is_dummy,
                } => {
                    assert_eq!(id, row.rolling_stock_id);
                    assert_eq!(railway.railway_id, row.railway_id);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(livery, row.livery);
                    assert_eq!(depot, row.depot);

                    assert!(length_over_buffer.is_some());
                    let length_over_buffer = length_over_buffer.unwrap();
                    assert_eq!(
                        length_over_buffer.inches.map(|l| l.quantity()),
                        row.length_over_buffers_in
                    );
                    assert_eq!(
                        length_over_buffer.millimeters.map(|l| l.quantity()),
                        row.length_over_buffers_mm
                    );

                    assert_eq!(class_name, row.type_name.unwrap());
                    assert_eq!(road_number, row.road_number.unwrap());
                    assert_eq!(series, row.series);

                    assert!(technical_specifications.is_some());
                    let technical_specifications = technical_specifications.unwrap();
                    assert_eq!(technical_specifications.lights, row.lights.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.interior_lights,
                        row.interior_lights.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications.spring_buffers,
                        row.spring_buffers.unwrap_or_default()
                    );
                    assert_eq!(technical_specifications.metal_body, row.metal_body.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.flywheel_fitted,
                        row.flywheel_fitted.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications
                            .minimum_radius
                            .map(|r| r.value())
                            .map(|l| l.quantity()),
                        row.minimum_radius
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.close_couplers),
                        row.close_couplers
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.digital_shunting),
                        row.digital_shunting_coupling
                    );
                    assert_eq!(technical_specifications.coupling.map(|c| c.socket), row.coupling_socket);

                    assert_eq!(locomotive_type, row.locomotive_type.unwrap());
                    assert_eq!(dcc_interface, row.dcc_interface);
                    assert_eq!(control, row.control);
                    assert!(!is_dummy);
                }
                _ => panic!("invalid rolling stock type"),
            }
        }

        #[test]
        fn it_should_convert_an_electric_multiple_unit_rolling_stock() {
            let row = RollingStockRow {
                rolling_stock_category: RollingStockCategory::ElectricMultipleUnit,
                electric_multiple_unit_type: Some(ElectricMultipleUnitType::PowerCar),
                livery: Some(String::from("blue")),
                length_over_buffers_mm: Some(dec!(16.5)),
                length_over_buffers_in: Some(dec!(0.65)),
                type_name: Some(String::from("Group 1")),
                road_number: Some(String::from("Number 42")),
                depot: Some(String::from("Depot")),
                series: Some(String::from("prototype")),
                dcc_interface: Some(DccInterface::Mtc21),
                control: Some(Control::DccReady),
                minimum_radius: Some(dec!(360)),
                close_couplers: Some(FeatureFlag::Yes),
                lights: Some(FeatureFlag::Yes),
                coupling_socket: Some(CouplingSocket::Nem362),
                ..default_row()
            };

            let rolling_stock = row.clone().to_output().expect("the rolling stock conversion failed");

            match rolling_stock {
                RollingStock::ElectricMultipleUnit {
                    id,
                    railway,
                    epoch,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    type_name,
                    road_number,
                    series,
                    depot,
                    electric_multiple_unit_type,
                    dcc_interface,
                    control,
                    is_dummy,
                } => {
                    assert_eq!(id, row.rolling_stock_id);
                    assert_eq!(railway.railway_id, row.railway_id);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(livery, row.livery);
                    assert_eq!(depot, row.depot);

                    assert!(length_over_buffer.is_some());
                    let length_over_buffer = length_over_buffer.unwrap();
                    assert_eq!(
                        length_over_buffer.inches.map(|l| l.quantity()),
                        row.length_over_buffers_in
                    );
                    assert_eq!(
                        length_over_buffer.millimeters.map(|l| l.quantity()),
                        row.length_over_buffers_mm
                    );

                    assert_eq!(type_name, row.type_name.unwrap());
                    assert_eq!(road_number, row.road_number);
                    assert_eq!(series, row.series);

                    assert!(technical_specifications.is_some());
                    let technical_specifications = technical_specifications.unwrap();
                    assert_eq!(technical_specifications.lights, row.lights.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.interior_lights,
                        row.interior_lights.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications.spring_buffers,
                        row.spring_buffers.unwrap_or_default()
                    );
                    assert_eq!(technical_specifications.metal_body, row.metal_body.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.flywheel_fitted,
                        row.flywheel_fitted.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications
                            .minimum_radius
                            .map(|r| r.value())
                            .map(|l| l.quantity()),
                        row.minimum_radius
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.close_couplers),
                        row.close_couplers
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.digital_shunting),
                        row.digital_shunting_coupling
                    );
                    assert_eq!(technical_specifications.coupling.map(|c| c.socket), row.coupling_socket);

                    assert_eq!(electric_multiple_unit_type, row.electric_multiple_unit_type.unwrap());
                    assert_eq!(dcc_interface, row.dcc_interface);
                    assert_eq!(control, row.control);
                    assert!(!is_dummy);
                }
                _ => panic!("invalid rolling stock type"),
            }
        }

        #[test]
        fn it_should_convert_a_railcar_rolling_stock() {
            let row = RollingStockRow {
                rolling_stock_category: RollingStockCategory::Railcar,
                railcar_type: Some(RailcarType::PowerCar),
                livery: Some(String::from("blue")),
                length_over_buffers_mm: Some(dec!(16.5)),
                length_over_buffers_in: Some(dec!(0.65)),
                type_name: Some(String::from("Group 1")),
                road_number: Some(String::from("Number 42")),
                depot: Some(String::from("Depot")),
                series: Some(String::from("prototype")),
                dcc_interface: Some(DccInterface::Mtc21),
                control: Some(Control::DccReady),
                minimum_radius: Some(dec!(360)),
                close_couplers: Some(FeatureFlag::Yes),
                lights: Some(FeatureFlag::Yes),
                coupling_socket: Some(CouplingSocket::Nem362),
                ..default_row()
            };

            let rolling_stock = row.clone().to_output().expect("the rolling stock conversion failed");

            match rolling_stock {
                RollingStock::Railcar {
                    id,
                    railway,
                    epoch,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    type_name,
                    road_number,
                    series,
                    depot,
                    railcar_type,
                    dcc_interface,
                    control,
                    is_dummy,
                } => {
                    assert_eq!(id, row.rolling_stock_id);
                    assert_eq!(railway.railway_id, row.railway_id);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(livery, row.livery);
                    assert_eq!(depot, row.depot);

                    assert!(length_over_buffer.is_some());
                    let length_over_buffer = length_over_buffer.unwrap();
                    assert_eq!(
                        length_over_buffer.inches.map(|l| l.quantity()),
                        row.length_over_buffers_in
                    );
                    assert_eq!(
                        length_over_buffer.millimeters.map(|l| l.quantity()),
                        row.length_over_buffers_mm
                    );

                    assert_eq!(type_name, row.type_name.unwrap());
                    assert_eq!(road_number, row.road_number);
                    assert_eq!(series, row.series);

                    assert!(technical_specifications.is_some());
                    let technical_specifications = technical_specifications.unwrap();
                    assert_eq!(technical_specifications.lights, row.lights.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.interior_lights,
                        row.interior_lights.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications.spring_buffers,
                        row.spring_buffers.unwrap_or_default()
                    );
                    assert_eq!(technical_specifications.metal_body, row.metal_body.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.flywheel_fitted,
                        row.flywheel_fitted.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications
                            .minimum_radius
                            .map(|r| r.value())
                            .map(|l| l.quantity()),
                        row.minimum_radius
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.close_couplers),
                        row.close_couplers
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.digital_shunting),
                        row.digital_shunting_coupling
                    );
                    assert_eq!(technical_specifications.coupling.map(|c| c.socket), row.coupling_socket);

                    assert_eq!(railcar_type, row.railcar_type.unwrap());
                    assert_eq!(dcc_interface, row.dcc_interface);
                    assert_eq!(control, row.control);
                    assert!(!is_dummy);
                }
                _ => panic!("invalid rolling stock type"),
            }
        }

        #[test]
        fn it_should_convert_a_passenger_car_rolling_stock() {
            let row = RollingStockRow {
                rolling_stock_category: RollingStockCategory::PassengerCar,
                passenger_car_type: Some(PassengerCarType::CombineCar),
                livery: Some(String::from("blue")),
                length_over_buffers_mm: Some(dec!(16.5)),
                length_over_buffers_in: Some(dec!(0.65)),
                type_name: Some(String::from("Group 1")),
                road_number: Some(String::from("Number 42")),
                series: Some(String::from("prototype")),
                minimum_radius: Some(dec!(360)),
                close_couplers: Some(FeatureFlag::Yes),
                lights: Some(FeatureFlag::Yes),
                coupling_socket: Some(CouplingSocket::Nem362),
                ..default_row()
            };

            let rolling_stock = row.clone().to_output().expect("the rolling stock conversion failed");

            match rolling_stock {
                RollingStock::PassengerCar {
                    id,
                    railway,
                    epoch,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    type_name,
                    road_number,
                    series,
                    passenger_car_type,
                    service_level,
                } => {
                    assert_eq!(id, row.rolling_stock_id);
                    assert_eq!(railway.railway_id, row.railway_id);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(livery, row.livery);

                    assert!(length_over_buffer.is_some());
                    let length_over_buffer = length_over_buffer.unwrap();
                    assert_eq!(
                        length_over_buffer.inches.map(|l| l.quantity()),
                        row.length_over_buffers_in
                    );
                    assert_eq!(
                        length_over_buffer.millimeters.map(|l| l.quantity()),
                        row.length_over_buffers_mm
                    );

                    assert_eq!(type_name, row.type_name.unwrap());
                    assert_eq!(road_number, row.road_number);
                    assert_eq!(series, row.series);

                    assert!(technical_specifications.is_some());
                    let technical_specifications = technical_specifications.unwrap();
                    assert_eq!(technical_specifications.lights, row.lights.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.interior_lights,
                        row.interior_lights.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications.spring_buffers,
                        row.spring_buffers.unwrap_or_default()
                    );
                    assert_eq!(technical_specifications.metal_body, row.metal_body.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.flywheel_fitted,
                        row.flywheel_fitted.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications
                            .minimum_radius
                            .map(|r| r.value())
                            .map(|l| l.quantity()),
                        row.minimum_radius
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.close_couplers),
                        row.close_couplers
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.digital_shunting),
                        row.digital_shunting_coupling
                    );
                    assert_eq!(technical_specifications.coupling.map(|c| c.socket), row.coupling_socket);

                    assert_eq!(passenger_car_type, row.passenger_car_type);
                    assert_eq!(service_level, row.service_level);
                }
                _ => panic!("invalid rolling stock type"),
            }
        }

        #[test]
        fn it_should_convert_a_freight_car_rolling_stock() {
            let row = RollingStockRow {
                rolling_stock_category: RollingStockCategory::FreightCar,
                freight_car_type: Some(FreightCarType::CoveredFreightCars),
                livery: Some(String::from("blue")),
                length_over_buffers_mm: Some(dec!(16.5)),
                length_over_buffers_in: Some(dec!(0.65)),
                type_name: Some(String::from("Group 1")),
                road_number: Some(String::from("Number 42")),
                series: Some(String::from("prototype")),
                minimum_radius: Some(dec!(360)),
                close_couplers: Some(FeatureFlag::Yes),
                lights: Some(FeatureFlag::Yes),
                coupling_socket: Some(CouplingSocket::Nem362),
                ..default_row()
            };

            let rolling_stock = row.clone().to_output().expect("the rolling stock conversion failed");

            match rolling_stock {
                RollingStock::FreightCar {
                    id,
                    railway,
                    epoch,
                    livery,
                    length_over_buffer,
                    technical_specifications,
                    type_name,
                    road_number,
                    freight_car_type,
                } => {
                    assert_eq!(id, row.rolling_stock_id);
                    assert_eq!(railway.railway_id, row.railway_id);
                    assert_eq!(epoch, Epoch::IV);
                    assert_eq!(livery, row.livery);

                    assert!(length_over_buffer.is_some());
                    let length_over_buffer = length_over_buffer.unwrap();
                    assert_eq!(
                        length_over_buffer.inches.map(|l| l.quantity()),
                        row.length_over_buffers_in
                    );
                    assert_eq!(
                        length_over_buffer.millimeters.map(|l| l.quantity()),
                        row.length_over_buffers_mm
                    );

                    assert_eq!(type_name, row.type_name.unwrap());
                    assert_eq!(road_number, row.road_number);

                    assert!(technical_specifications.is_some());
                    let technical_specifications = technical_specifications.unwrap();
                    assert_eq!(technical_specifications.lights, row.lights.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.interior_lights,
                        row.interior_lights.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications.spring_buffers,
                        row.spring_buffers.unwrap_or_default()
                    );
                    assert_eq!(technical_specifications.metal_body, row.metal_body.unwrap_or_default());
                    assert_eq!(
                        technical_specifications.flywheel_fitted,
                        row.flywheel_fitted.unwrap_or_default()
                    );
                    assert_eq!(
                        technical_specifications
                            .minimum_radius
                            .map(|r| r.value())
                            .map(|l| l.quantity()),
                        row.minimum_radius
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.close_couplers),
                        row.close_couplers
                    );
                    assert_eq!(
                        technical_specifications.coupling.map(|c| c.digital_shunting),
                        row.digital_shunting_coupling
                    );
                    assert_eq!(technical_specifications.coupling.map(|c| c.socket), row.coupling_socket);

                    assert_eq!(freight_car_type, row.freight_car_type);
                }
                _ => panic!("invalid rolling stock type"),
            }
        }

        fn default_row() -> RollingStockRow {
            new_rolling_stock_row(CatalogItemId::from_str("acme-123456").unwrap(), "FS")
        }
    }
}
