//! the rolling stock view models

use crate::catalog_items::category::{
    ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType, RollingStockCategory,
};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::length_over_buffers::LengthOverBuffers;
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::TechnicalSpecifications;
use crate::railways::railway_id::RailwayId;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "category")]
pub enum RollingStock {
    /// an electric multiple unit rolling stock
    ElectricMultipleUnit {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the electric multiple unit type name
        type_name: String,
        /// the identification marking for this electric multiple unit
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the electric multiple unit type
        electric_multiple_unit_type: ElectricMultipleUnitType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a freight car rolling stock
    FreightCar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the freight car type name
        type_name: String,
        /// the identification marking for this freight car
        road_number: Option<String>,
        /// the freight car type
        freight_car_type: Option<FreightCarType>,
    },
    /// a locomotive rolling stock
    Locomotive {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specification
        technical_specifications: Option<TechnicalSpecifications>,
        /// the class of locomotives. The class is a group of locomotives built to a common design,
        /// typically for a single railroad or railway
        class_name: String,
        /// the identification marking for this locomotive
        road_number: String,
        /// the prototype series information
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the locomotive type
        locomotive_type: LocomotiveType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
    /// a passenger car rolling stock
    PassengerCar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the passenger car type name
        type_name: String,
        /// the identification marking for this passenger car
        road_number: Option<String>,
        /// the prototype series information
        series: Option<String>,
        /// the passenger car type
        passenger_car_type: Option<PassengerCarType>,
        /// the travel class for this passenger car. Passenger cars can have multiple service
        /// levels (ie, '1st/2nd')
        service_level: Option<ServiceLevel>,
    },
    /// a railcar rolling stock
    Railcar {
        /// the unique identifier for this rolling stock
        id: RollingStockId,
        /// the railway for this rolling stock
        railway: RollingStockRailway,
        /// the livery description
        livery: Option<String>,
        /// the overall length
        length_over_buffer: Option<LengthOverBuffers>,
        /// the technical specifications
        technical_specifications: Option<TechnicalSpecifications>,
        /// the railcar type name
        type_name: String,
        /// the identification marking for this railcar
        road_number: Option<String>,
        /// the railcar series
        series: Option<String>,
        /// the depot name
        depot: Option<String>,
        /// the railcar type
        railcar_type: RailcarType,
        /// the dcc interface
        dcc_interface: Option<DccInterface>,
        /// the control
        control: Option<Control>,
        /// indicate whether the rolling stock has a motor or not
        is_dummy: bool,
    },
}

impl RollingStock {
    /// Creates a new electric multiple unit rolling stock
    pub fn new_electric_multiple_unit(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        series: Option<&str>,
        railway: RollingStockRailway,
        electric_multiple_unit_type: ElectricMultipleUnitType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffers>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        technical_specifications: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::ElectricMultipleUnit {
            id,
            railway,
            livery: livery.map(str::to_string),
            length_over_buffer,
            technical_specifications,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            series: series.map(str::to_string),
            depot: depot.map(str::to_string),
            electric_multiple_unit_type,
            dcc_interface,
            control,
            is_dummy,
        }
    }

    /// Creates a new freight car rolling stock
    pub fn new_freight_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        railway: RollingStockRailway,
        freight_car_type: Option<FreightCarType>,
        livery: Option<&str>,
        length_over_buffer: Option<LengthOverBuffers>,
        technical_specifications: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::FreightCar {
            id,
            railway,
            livery: livery.map(str::to_string),
            length_over_buffer,
            technical_specifications,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            freight_car_type,
        }
    }

    /// Creates a new locomotive rolling stock
    pub fn new_locomotive(
        id: RollingStockId,
        class_name: &str,
        road_number: &str,
        series: Option<&str>,
        railway: RollingStockRailway,
        locomotive_type: LocomotiveType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffers>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        technical_specifications: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::Locomotive {
            id,
            railway,
            livery: livery.map(str::to_string),
            length_over_buffer,
            technical_specifications,
            class_name: String::from(class_name),
            road_number: String::from(road_number),
            series: series.map(str::to_string),
            depot: depot.map(str::to_string),
            locomotive_type,
            dcc_interface,
            control,
            is_dummy,
        }
    }

    /// Creates a new passenger car rolling stock
    pub fn new_passenger_car(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        series: Option<&str>,
        railway: RollingStockRailway,
        passenger_car_type: Option<PassengerCarType>,
        service_level: Option<ServiceLevel>,
        livery: Option<&str>,
        length_over_buffer: Option<LengthOverBuffers>,
        technical_specifications: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::PassengerCar {
            id,
            railway,
            livery: livery.map(str::to_string),
            length_over_buffer,
            technical_specifications,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            series: series.map(str::to_string),
            passenger_car_type,
            service_level,
        }
    }

    /// Creates a new railcar rolling stock
    pub fn new_railcar(
        id: RollingStockId,
        type_name: &str,
        road_number: Option<&str>,
        series: Option<&str>,
        railway: RollingStockRailway,
        railcar_type: RailcarType,
        depot: Option<&str>,
        livery: Option<&str>,
        is_dummy: bool,
        length_over_buffer: Option<LengthOverBuffers>,
        control: Option<Control>,
        dcc_interface: Option<DccInterface>,
        technical_specifications: Option<TechnicalSpecifications>,
    ) -> Self {
        RollingStock::Railcar {
            id,
            railway,
            livery: livery.map(str::to_string),
            length_over_buffer,
            technical_specifications,
            type_name: String::from(type_name),
            road_number: road_number.map(str::to_string),
            series: series.map(str::to_string),
            depot: depot.map(str::to_string),
            railcar_type,
            dcc_interface,
            control,
            is_dummy,
        }
    }

    /// The category for this rolling stock
    pub fn category(&self) -> RollingStockCategory {
        match self {
            RollingStock::ElectricMultipleUnit { .. } => RollingStockCategory::ElectricMultipleUnit,
            RollingStock::Locomotive { .. } => RollingStockCategory::Locomotive,
            RollingStock::FreightCar { .. } => RollingStockCategory::FreightCar,
            RollingStock::PassengerCar { .. } => RollingStockCategory::PassengerCar,
            RollingStock::Railcar { .. } => RollingStockCategory::Railcar,
        }
    }

    /// The unique identifier for this rolling stock
    pub fn id(&self) -> RollingStockId {
        match self {
            RollingStock::ElectricMultipleUnit { id, .. } => *id,
            RollingStock::Locomotive { id, .. } => *id,
            RollingStock::FreightCar { id, .. } => *id,
            RollingStock::PassengerCar { id, .. } => *id,
            RollingStock::Railcar { id, .. } => *id,
        }
    }

    /// The livery for this rolling stock
    pub fn livery(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { livery, .. } => livery.as_deref(),
            RollingStock::Locomotive { livery, .. } => livery.as_deref(),
            RollingStock::FreightCar { livery, .. } => livery.as_deref(),
            RollingStock::PassengerCar { livery, .. } => livery.as_deref(),
            RollingStock::Railcar { livery, .. } => livery.as_deref(),
        }
    }

    /// The overall length for this rolling stock
    pub fn length_over_buffer(&self) -> Option<&LengthOverBuffers> {
        match self {
            RollingStock::ElectricMultipleUnit { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Locomotive { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::FreightCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::PassengerCar { length_over_buffer, .. } => length_over_buffer.as_ref(),
            RollingStock::Railcar { length_over_buffer, .. } => length_over_buffer.as_ref(),
        }
    }

    /// The railway company for this rolling stock
    pub fn railway(&self) -> &RollingStockRailway {
        match self {
            RollingStock::ElectricMultipleUnit { railway, .. } => railway,
            RollingStock::Locomotive { railway, .. } => railway,
            RollingStock::FreightCar { railway, .. } => railway,
            RollingStock::PassengerCar { railway, .. } => railway,
            RollingStock::Railcar { railway, .. } => railway,
        }
    }

    /// The road number for this rolling stock
    pub fn road_number(&self) -> Option<&str> {
        match self {
            RollingStock::ElectricMultipleUnit { road_number, .. } => road_number.as_deref(),
            RollingStock::Locomotive { road_number, .. } => Some(road_number),
            RollingStock::FreightCar { road_number, .. } => road_number.as_deref(),
            RollingStock::PassengerCar { road_number, .. } => road_number.as_deref(),
            RollingStock::Railcar { road_number, .. } => road_number.as_deref(),
        }
    }

    /// The technical specification for this rolling stock
    pub fn technical_specifications(&self) -> Option<&TechnicalSpecifications> {
        match self {
            RollingStock::ElectricMultipleUnit {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::Locomotive {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::FreightCar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::PassengerCar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
            RollingStock::Railcar {
                technical_specifications: tech_specs,
                ..
            } => tech_specs.as_ref(),
        }
    }

    /// The control method for this rolling stock
    pub fn control(&self) -> Option<Control> {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control), ..
            } => Some(*control),
            RollingStock::Locomotive {
                control: Some(control), ..
            } => Some(*control),
            RollingStock::Railcar {
                control: Some(control), ..
            } => Some(*control),
            _ => None,
        }
    }

    /// The dcc interface for this rolling stock
    pub fn dcc_interface(&self) -> Option<DccInterface> {
        match self {
            RollingStock::ElectricMultipleUnit {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Locomotive {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            RollingStock::Railcar {
                dcc_interface: Some(dcc_interface),
                ..
            } => Some(*dcc_interface),
            _ => None,
        }
    }

    /// Return true if the rolling stock has a decoder, false otherwise
    pub fn with_decoder(&self) -> bool {
        match self {
            RollingStock::ElectricMultipleUnit {
                control: Some(control), ..
            } => control.with_decoder(),
            RollingStock::Locomotive {
                control: Some(control), ..
            } => control.with_decoder(),
            RollingStock::Railcar {
                control: Some(control), ..
            } => control.with_decoder(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct RollingStockRailway {
    /// the railway unique identifier
    pub railway_id: RailwayId,
    /// the railway display name
    pub display: String,
}

impl RollingStockRailway {
    /// Creates a new railway with the display text.
    pub fn new(railway_id: RailwayId, display: &str) -> Self {
        RollingStockRailway {
            railway_id,
            display: display.to_owned(),
        }
    }

    /// Returns this railway unique identifier
    pub fn id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// Returns this railway display text
    pub fn display_text(&self) -> &str {
        &self.display
    }
}

impl Display for RollingStockRailway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod locomotives {
        use super::*;
        use crate::catalog_items::rolling_stock_id::RollingStockId;
        use crate::catalog_items::technical_specifications::{
            Coupling, CouplingSocket, Radius, TechnicalSpecificationsBuilder,
        };
        use common::length::Length;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_locomotives() {
            let id = RollingStockId::new();
            let length = LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(210)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let locomotive = RollingStock::new_locomotive(
                id,
                "E.656",
                "E.656 077",
                Some("I serie"),
                fs.clone(),
                LocomotiveType::ElectricLocomotive,
                Some("Milano Centrale"),
                Some("blu/grigio"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, locomotive.id());
            assert_eq!(RollingStockCategory::Locomotive, locomotive.category());
            assert_eq!(Some("blu/grigio"), locomotive.livery());
            assert_eq!(Some(&length), locomotive.length_over_buffer());
            assert_eq!(&fs, locomotive.railway());
            assert_eq!(Some("E.656 077"), locomotive.road_number());
            assert_eq!(Some(DccInterface::Nem652), locomotive.dcc_interface());
            assert_eq!(Some(Control::DccReady), locomotive.control());
            assert_eq!(Some(&tech_specs), locomotive.technical_specifications());
        }

        #[test]
        fn it_should_create_new_electric_multiple_units() {
            let id = RollingStockId::new();
            let length = LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let power_car = RollingStock::new_electric_multiple_unit(
                id,
                "ALe 801",
                Some("ALe 801 003"),
                None,
                fs.clone(),
                ElectricMultipleUnitType::PowerCar,
                Some("Milano Centrale"),
                Some("livrea originale giallo/arancio"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, power_car.id());
            assert_eq!(RollingStockCategory::ElectricMultipleUnit, power_car.category());
            assert_eq!(Some("livrea originale giallo/arancio"), power_car.livery());
            assert_eq!(Some(&length), power_car.length_over_buffer());
            assert_eq!(&fs, power_car.railway());
            assert_eq!(Some("ALe 801 003"), power_car.road_number());
            assert_eq!(Some(DccInterface::Nem652), power_car.dcc_interface());
            assert_eq!(Some(Control::DccReady), power_car.control());
            assert_eq!(Some(&tech_specs), power_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_passenger_cars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let passenger_car = RollingStock::new_passenger_car(
                id,
                "UIC-Z1",
                Some("61 83 19-90 105-3 A"),
                None,
                fs.clone(),
                Some(PassengerCarType::CompartmentCoach),
                Some(ServiceLevel::FirstClass),
                Some("XMPR"),
                Some(length),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, passenger_car.id());
            assert_eq!(RollingStockCategory::PassengerCar, passenger_car.category());
            assert_eq!(Some("XMPR"), passenger_car.livery());
            assert_eq!(Some(&length), passenger_car.length_over_buffer());
            assert_eq!(&fs, passenger_car.railway());
            assert_eq!(Some("61 83 19-90 105-3 A"), passenger_car.road_number());
            assert_eq!(None, passenger_car.dcc_interface());
            assert_eq!(None, passenger_car.control());
            assert_eq!(Some(&tech_specs), passenger_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_railcars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let power_car = RollingStock::new_railcar(
                id,
                "ALn 668",
                Some("ALn 668 1449"),
                None,
                fs.clone(),
                RailcarType::PowerCar,
                Some("Milano Centrale"),
                Some("verde lichene/giallo coloniale"),
                false,
                Some(length),
                Some(Control::DccReady),
                Some(DccInterface::Nem652),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, power_car.id());
            assert_eq!(RollingStockCategory::Railcar, power_car.category());
            assert_eq!(Some("verde lichene/giallo coloniale"), power_car.livery());
            assert_eq!(Some(&length), power_car.length_over_buffer());
            assert_eq!(&fs, power_car.railway());
            assert_eq!(Some("ALn 668 1449"), power_car.road_number());
            assert_eq!(Some(DccInterface::Nem652), power_car.dcc_interface());
            assert_eq!(Some(Control::DccReady), power_car.control());
            assert_eq!(Some(&tech_specs), power_car.technical_specifications());
        }

        #[test]
        fn it_should_create_new_freight_cars() {
            let id = RollingStockId::new();
            let length = LengthOverBuffers::from_millimeters(Length::Millimeters(dec!(303)));
            let fs = RollingStockRailway::new(RailwayId::new("fs"), "FS");

            let tech_specs = technical_specification();

            let freight_car = RollingStock::new_freight_car(
                id,
                "Fals",
                Some("31 83 665 0 150-6"),
                fs.clone(),
                Some(FreightCarType::Gondola),
                Some("castano"),
                Some(length),
                Some(tech_specs.clone()),
            );

            assert_eq!(id, freight_car.id());
            assert_eq!(RollingStockCategory::FreightCar, freight_car.category());
            assert_eq!(Some("castano"), freight_car.livery());
            assert_eq!(Some(&length), freight_car.length_over_buffer());
            assert_eq!(&fs, freight_car.railway());
            assert_eq!(Some("31 83 665 0 150-6"), freight_car.road_number());
            assert_eq!(None, freight_car.dcc_interface());
            assert_eq!(None, freight_car.control());
            assert_eq!(Some(&tech_specs), freight_car.technical_specifications());
        }

        fn technical_specification() -> TechnicalSpecifications {
            let radius = Radius::from_millimeters(dec!(360.0)).unwrap();
            let coupling = Coupling::with_close_couplers(CouplingSocket::Nem362);
            TechnicalSpecificationsBuilder::default()
                .with_coupling(coupling)
                .with_minimum_radius(radius)
                .build()
        }
    }
}
