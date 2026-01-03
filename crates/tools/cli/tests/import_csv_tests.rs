use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::category::{Category, LocomotiveType};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::delivery_date::DeliveryDate;
use catalog::catalog_items::epoch::Epoch;
use catalog::catalog_items::item_number::ItemNumber;
use catalog::catalog_items::length_over_buffers::LengthOverBuffers;
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest::{
    FreightCarRequest, LocomotiveRequest, PassengerCarRequest,
};
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{
    BodyShellType, ChassisType, Coupling, CouplingSocket, FeatureFlag, Radius,
};
use cli::cvs_files::read_catalog_items;
use common::length::Length;
use pretty_assertions::assert_eq;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const CSV_FILE_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/csv");

#[test]
fn it_should_parse_a_single_locomotive_from_csv_files() {
    let csv_file = format!("{}/locomotives_single.csv", CSV_FILE_ROOT);

    let catalog_items = read_catalog_items(&csv_file).expect("failed to parse the test csv file");

    assert_eq!(1, catalog_items.len());

    let rivarossi_hr2934 = &catalog_items[0];
    let rivarossi_hr2934_description = Some(String::from(
        "FS, locomotiva elettrica E.645, 1a serie, livrea castano/Isabella con logo FS semplificato, pantografi 42U, ep. IV-V",
    ));
    assert_eq!("Rivarossi/Hornby", rivarossi_hr2934.brand);
    assert_eq!(ItemNumber::new("HR2934"), rivarossi_hr2934.item_number);
    assert_eq!(Category::Locomotives, rivarossi_hr2934.category);
    assert_eq!(PowerMethod::DC, rivarossi_hr2934.power_method);
    assert_eq!(
        Epoch::Multiple(Box::new(Epoch::IV), Box::new(Epoch::V)),
        rivarossi_hr2934.epoch
    );
    assert_eq!("H0", rivarossi_hr2934.scale);
    assert_eq!(
        rivarossi_hr2934_description.as_ref(),
        rivarossi_hr2934.description.italian()
    );
    assert_eq!(Some(DeliveryDate::by_year(2023)), rivarossi_hr2934.delivery_date);
    assert_eq!(
        Some(AvailabilityStatus::Announced),
        rivarossi_hr2934.availability_status
    );
    assert_eq!(1, rivarossi_hr2934.count);
    assert_eq!(1, rivarossi_hr2934.rolling_stocks.len());

    let expected_length = LengthOverBuffers::from_millimeters(Length::Millimeters(Decimal::new(210, 0)));

    match &rivarossi_hr2934.rolling_stocks[0] {
        LocomotiveRequest {
            railway,
            livery,
            length_over_buffers,
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
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("castano/isabella")), livery);
            assert_eq!(&String::from("E.645"), class_name);
            assert_eq!(&String::from("E.645 005"), road_number);
            assert_eq!(&Some(String::from("PRIMA SERIE")), series);
            assert_eq!(&LocomotiveType::ElectricLocomotive, locomotive_type);
            assert_eq!(&Some(String::from("Milano Smistamento")), depot);
            assert_eq!(&Some(DccInterface::Mtc21), dcc_interface);
            assert_eq!(&Some(Control::DccReady), control);
            assert_eq!(&false, is_dummy);

            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );

            assert_eq!(
                Some(FeatureFlag::Yes),
                technical_specifications.as_ref().and_then(|specs| specs.lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.interior_lights)
            );
            assert_eq!(
                Some(FeatureFlag::No),
                technical_specifications.as_ref().and_then(|specs| specs.sprung_buffers)
            );
            assert_eq!(
                Some(BodyShellType::Plastic),
                technical_specifications.as_ref().and_then(|specs| specs.body_shell)
            );
            assert_eq!(
                Some(ChassisType::MetalDieCast),
                technical_specifications.as_ref().and_then(|specs| specs.chassis)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.flywheel_fitted)
            );
        }
        _ => panic!("rivarossi_hr2934.rolling_stocks[0] is not a LocomotiveRequest"),
    }
}

#[test]
fn it_should_parse_multiple_passenger_cars_from_csv_files() {
    let csv_file = format!("{}/passenger_cars_multiple.csv", CSV_FILE_ROOT);

    let catalog_items = read_catalog_items(&csv_file).expect("failed to parse the test csv file");

    assert_eq!(1, catalog_items.len());

    let rivarossi_hr4324 = &catalog_items[0];
    let rivarossi_hr4324_description = Some(String::from(
        "FS, set di 4 carrozze “Treno Azzurro”, composto da 2 carrozze di 1a classe tipo 1946 Az13010 e 2 carrozze di 2a classe tipo 1946 Bz33010, una con scompartimento ristoro, ep. IIIb",
    ));
    assert_eq!("Rivarossi/Hornby", rivarossi_hr4324.brand);
    assert_eq!(ItemNumber::new("HR4324"), rivarossi_hr4324.item_number);
    assert_eq!(Category::PassengerCars, rivarossi_hr4324.category);
    assert_eq!(PowerMethod::DC, rivarossi_hr4324.power_method);
    assert_eq!(Epoch::IIIb, rivarossi_hr4324.epoch);
    assert_eq!("H0", rivarossi_hr4324.scale);
    assert_eq!(
        rivarossi_hr4324_description.as_ref(),
        rivarossi_hr4324.description.italian()
    );
    assert_eq!(Some(DeliveryDate::by_year(2023)), rivarossi_hr4324.delivery_date);
    assert_eq!(
        Some(AvailabilityStatus::Announced),
        rivarossi_hr4324.availability_status
    );
    assert_eq!(4, rivarossi_hr4324.count);

    assert_eq!(4, rivarossi_hr4324.rolling_stocks.len());

    let expected_length = LengthOverBuffers::from_millimeters(Length::Millimeters(Decimal::new(263, 0)));

    match &rivarossi_hr4324.rolling_stocks[0] {
        PassengerCarRequest {
            railway,
            livery,
            length_over_buffers,
            technical_specifications,
            type_name,
            road_number,
            series,
            passenger_car_type,
            service_level,
        } => {
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("Treno Azzurro")), livery);
            assert_eq!(&String::from("Tipo 1946"), type_name);
            assert_eq!(&None, road_number);
            assert_eq!(&Some(String::from("Az13010")), series);
            assert_eq!(&None, passenger_car_type);
            assert_eq!(&Some(ServiceLevel::FirstClass), service_level);
            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.interior_lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.sprung_buffers)
            );
            assert_eq!(
                Some(BodyShellType::Plastic),
                technical_specifications.as_ref().and_then(|specs| specs.body_shell)
            );
            assert_eq!(None, technical_specifications.as_ref().and_then(|specs| specs.chassis));
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.flywheel_fitted)
            );
        }
        _ => panic!("rivarossi_hr4324.rolling_stocks[0] is not a PassengerCarRequest"),
    }

    match &rivarossi_hr4324.rolling_stocks[1] {
        PassengerCarRequest {
            railway,
            livery,
            length_over_buffers,
            technical_specifications,
            type_name,
            road_number,
            series,
            passenger_car_type,
            service_level,
        } => {
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("Treno Azzurro")), livery);
            assert_eq!(&String::from("Tipo 1946"), type_name);
            assert_eq!(&None, road_number);
            assert_eq!(&Some(String::from("Az13010")), series);
            assert_eq!(&None, passenger_car_type);
            assert_eq!(&Some(ServiceLevel::FirstClass), service_level);
            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.interior_lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.sprung_buffers)
            );
            assert_eq!(
                Some(BodyShellType::Plastic),
                technical_specifications.as_ref().and_then(|specs| specs.body_shell)
            );
            assert_eq!(None, technical_specifications.as_ref().and_then(|specs| specs.chassis));
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.flywheel_fitted)
            );
        }
        _ => panic!("rivarossi_hr4324.rolling_stocks[1] is not a PassengerCarRequest"),
    }

    match &rivarossi_hr4324.rolling_stocks[2] {
        PassengerCarRequest {
            railway,
            livery,
            length_over_buffers,
            technical_specifications,
            type_name,
            road_number,
            series,
            passenger_car_type,
            service_level,
        } => {
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("Treno Azzurro")), livery);
            assert_eq!(&String::from("Tipo 1946"), type_name);
            assert_eq!(&None, road_number);
            assert_eq!(&Some(String::from("Bz33010")), series);
            assert_eq!(&None, passenger_car_type);
            assert_eq!(&Some(ServiceLevel::SecondClass), service_level);
            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.interior_lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.sprung_buffers)
            );
            assert_eq!(
                Some(BodyShellType::Plastic),
                technical_specifications.as_ref().and_then(|specs| specs.body_shell)
            );
            assert_eq!(None, technical_specifications.as_ref().and_then(|specs| specs.chassis));
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.flywheel_fitted)
            );
        }
        _ => panic!("rivarossi_hr4324.rolling_stocks[2] is not a PassengerCarRequest"),
    }

    match &rivarossi_hr4324.rolling_stocks[3] {
        PassengerCarRequest {
            railway,
            livery,
            length_over_buffers,
            technical_specifications,
            type_name,
            road_number,
            series,
            passenger_car_type,
            service_level,
        } => {
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("Treno Azzurro")), livery);
            assert_eq!(&String::from("Tipo 1946"), type_name);
            assert_eq!(&None, road_number);
            assert_eq!(&Some(String::from("Bz33010")), series);
            assert_eq!(&None, passenger_car_type);
            assert_eq!(&Some(ServiceLevel::SecondClass), service_level);
            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );
        }
        _ => panic!("rivarossi_hr4324.rolling_stocks[3] is not a PassengerCarRequest"),
    }
}

#[test]
fn it_should_parse_a_single_freight_car_from_csv_files() {
    let csv_file = format!("{}/freight_cars_single.csv", CSV_FILE_ROOT);

    let catalog_items = read_catalog_items(&csv_file).expect("failed to parse the test csv file");

    assert_eq!(1, catalog_items.len());

    let rivarossi_hr6613 = &catalog_items[0];
    let rivarossi_hr6613_description = Some(String::from(
        "CEMAT, carro porta container a 4 assi tipo Sgnss, livrea verde, nuovo logo CEMAT, caricato con un container “Nothegger” da 45 piedi, ep. VI",
    ));
    assert_eq!("Rivarossi/Hornby", rivarossi_hr6613.brand);
    assert_eq!(ItemNumber::new("HR6613"), rivarossi_hr6613.item_number);
    assert_eq!(Category::FreightCars, rivarossi_hr6613.category);
    assert_eq!(PowerMethod::DC, rivarossi_hr6613.power_method);
    assert_eq!(Epoch::VI, rivarossi_hr6613.epoch);
    assert_eq!("H0", rivarossi_hr6613.scale);
    assert_eq!(
        rivarossi_hr6613_description.as_ref(),
        rivarossi_hr6613.description.italian()
    );
    assert_eq!(Some(DeliveryDate::by_year(2023)), rivarossi_hr6613.delivery_date);
    assert_eq!(
        Some(AvailabilityStatus::Announced),
        rivarossi_hr6613.availability_status
    );
    assert_eq!(1, rivarossi_hr6613.count);
    assert_eq!(1, rivarossi_hr6613.rolling_stocks.len());

    let expected_length = LengthOverBuffers::from_millimeters(Length::Millimeters(Decimal::new(227, 0)));

    match &rivarossi_hr6613.rolling_stocks[0] {
        FreightCarRequest {
            railway,
            livery,
            length_over_buffers,
            technical_specifications,
            type_name,
            road_number,
            freight_car_type,
        } => {
            assert_eq!("FS", railway);
            assert_eq!(&Some(String::from("verde")), livery);
            assert_eq!(&String::from("Sgnss"), type_name);
            assert_eq!(&None, road_number);
            assert_eq!(&None, freight_car_type);

            assert_eq!(&Some(expected_length), length_over_buffers);

            assert_eq!(
                Some(Radius::from_millimeters(dec!(360)).unwrap()),
                technical_specifications.as_ref().and_then(|specs| specs.minimum_radius)
            );

            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);
            assert_eq!(
                Some(coupling),
                technical_specifications.as_ref().and_then(|specs| specs.coupling)
            );

            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.interior_lights)
            );
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications.as_ref().and_then(|specs| specs.sprung_buffers)
            );
            assert_eq!(
                Some(BodyShellType::Plastic),
                technical_specifications.as_ref().and_then(|specs| specs.body_shell)
            );
            assert_eq!(None, technical_specifications.as_ref().and_then(|specs| specs.chassis));
            assert_eq!(
                Some(FeatureFlag::NotApplicable),
                technical_specifications
                    .as_ref()
                    .and_then(|specs| specs.flywheel_fitted)
            );
        }
        _ => panic!("rivarossi_hr6613.rolling_stocks[0] is not a FreightCarRequest"),
    }
}
