//! the rolling stock technical specifications

use common::length::{Length, validate_length_range};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use strum_macros;
use strum_macros::{Display, EnumString};
use thiserror::Error;
use validator::Validate;
use validator::ValidationError;

/// It represents the coupling configuration for a rolling stock.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coupling {
    /// the rolling stock coupling socket
    pub socket: Option<CouplingSocket>,
    /// the rolling stock has a close coupling mechanism
    pub close_couplers: Option<FeatureFlag>,
    /// the rolling stock has a digital shunting couplers mechanism
    pub digital_shunting: Option<FeatureFlag>,
}

impl Coupling {
    /// Creates a new rolling stock coupling configuration
    pub fn new(socket: CouplingSocket, close_couplers: FeatureFlag, digital_shunting: FeatureFlag) -> Self {
        Coupling {
            socket: Some(socket),
            close_couplers: Some(close_couplers),
            digital_shunting: Some(digital_shunting),
        }
    }

    /// Creates a new close coupling configuration with the `socket` socket
    pub fn with_close_couplers(socket: CouplingSocket) -> Self {
        Coupling {
            socket: Some(socket),
            close_couplers: Some(FeatureFlag::Yes),
            digital_shunting: Some(FeatureFlag::No),
        }
    }

    /// Creates a new digital shunting coupling configuration
    pub fn with_digital_shunting_couplers() -> Self {
        Coupling {
            socket: Some(CouplingSocket::None),
            close_couplers: Some(FeatureFlag::No),
            digital_shunting: Some(FeatureFlag::Yes),
        }
    }

    /// the coupling socket if present
    pub fn socket(&self) -> Option<CouplingSocket> {
        self.socket
    }

    /// true if the coupling configuration include a mechanism to reduce the gaps between two
    /// rolling stocks; false otherwise
    pub fn close_couplers(&self) -> Option<FeatureFlag> {
        self.close_couplers
    }

    /// true if the coupling configuration implements digital control functionalities,
    /// false otherwise  
    pub fn digital_shunting(&self) -> Option<FeatureFlag> {
        self.digital_shunting
    }
}

/// The NEM coupling socket standards
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type, Default)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "socket_type")]
pub enum CouplingSocket {
    #[serde(rename = "NONE")]
    #[strum(serialize = "NONE")]
    #[sqlx(rename = "NONE")]
    #[default]
    None,

    /// Receptacle for Replaceable Coupling Heads in Scales TT and N
    #[serde(rename = "NEM_355")]
    #[strum(serialize = "NEM_355")]
    #[sqlx(rename = "NEM_355")]
    Nem355,

    /// Coupler Head for Scale N
    #[serde(rename = "NEM_356")]
    #[strum(serialize = "NEM_356")]
    #[sqlx(rename = "NEM_356")]
    Nem356,

    /// Coupler Head for Scale N
    #[serde(rename = "NEM_357")]
    #[strum(serialize = "NEM_357")]
    #[sqlx(rename = "NEM_357")]
    Nem357,

    /// Coupler Head for Scale TT
    #[serde(rename = "NEM_359")]
    #[strum(serialize = "NEM_359")]
    #[sqlx(rename = "NEM_359")]
    Nem359,

    /// Standard Coupling for Scale H0
    #[serde(rename = "NEM_360")]
    #[strum(serialize = "NEM_360")]
    #[sqlx(rename = "NEM_360")]
    Nem360,

    /// NEM shaft 362 with close coupling mechanism
    #[serde(rename = "NEM_362")]
    #[strum(serialize = "NEM_362")]
    #[sqlx(rename = "NEM_362")]
    Nem362,

    /// Coupler Head for Scale 0
    #[serde(rename = "NEM_365")]
    #[strum(serialize = "NEM_365")]
    #[sqlx(rename = "NEM_365")]
    Nem365,
}

/// The technical specification data for a rolling stock model
#[derive(Debug, Eq, PartialEq, Clone, Default, Serialize, Deserialize, Validate)]
pub struct TechnicalSpecifications {
    /// the minimum drivable radius
    #[validate(custom(function = "validate_radius"))]
    pub minimum_radius: Option<Radius>,
    /// the coupling
    pub coupling: Option<Coupling>,
    /// has a flywheel fitted
    pub flywheel_fitted: Option<FeatureFlag>,
    /// body shell type
    pub body_shell: Option<BodyShellType>,
    /// chassis type
    pub chassis: Option<ChassisType>,
    /// has interior lighting
    pub interior_lights: Option<FeatureFlag>,
    /// has lights
    pub lights: Option<FeatureFlag>,
    /// has sprung buffers
    pub sprung_buffers: Option<FeatureFlag>,
}

impl TechnicalSpecifications {
    /// the minimum radius
    pub fn minimum_radius(&self) -> Option<Radius> {
        self.minimum_radius
    }

    /// the coupling shaft standard
    pub fn coupling(&self) -> Option<Coupling> {
        self.coupling
    }

    /// with flywheel fitted
    pub fn flywheel_fitted(&self) -> Option<FeatureFlag> {
        self.flywheel_fitted
    }

    /// body shell type
    pub fn body_shell(&self) -> Option<BodyShellType> {
        self.body_shell
    }

    /// chassis type
    pub fn chassis(&self) -> Option<ChassisType> {
        self.chassis
    }

    /// with interior lights
    pub fn interior_lights(&self) -> Option<FeatureFlag> {
        self.interior_lights
    }

    /// with headlights
    pub fn lights(&self) -> Option<FeatureFlag> {
        self.lights
    }

    /// with sprung buffers
    pub fn sprung_buffers(&self) -> Option<FeatureFlag> {
        self.sprung_buffers
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TechnicalSpecificationsBuilder {
    minimum_radius: Option<Radius>,
    coupling: Option<Coupling>,
    flywheel_fitted: Option<FeatureFlag>,
    chassis: Option<ChassisType>,
    body_shell: Option<BodyShellType>,
    interior_lights: Option<FeatureFlag>,
    lights: Option<FeatureFlag>,
    sprung_buffers: Option<FeatureFlag>,
}

impl TechnicalSpecificationsBuilder {
    /// with the minimum radius
    pub fn with_minimum_radius(mut self, radius: Radius) -> Self {
        self.minimum_radius = Some(radius);
        self
    }

    /// with the coupling shaft
    pub fn with_coupling(mut self, coupling: Coupling) -> Self {
        self.coupling = Some(coupling);
        self
    }

    /// with flywheel fitted
    pub fn with_flywheel_fitted(mut self) -> Self {
        self.flywheel_fitted = Some(FeatureFlag::Yes);
        self
    }

    /// with body shell type
    pub fn with_body_shell(mut self, body_shell_types: BodyShellType) -> Self {
        self.body_shell = Some(body_shell_types);
        self
    }

    /// with chassis type
    pub fn with_chassis(mut self, chassis_types: ChassisType) -> Self {
        self.chassis = Some(chassis_types);
        self
    }

    /// with interior lights
    pub fn with_interior_lights(mut self) -> Self {
        self.interior_lights = Some(FeatureFlag::Yes);
        self
    }

    /// with headlights
    pub fn with_lights(mut self) -> Self {
        self.lights = Some(FeatureFlag::Yes);
        self
    }

    /// with sprung buffers
    pub fn with_sprung_buffers(mut self) -> Self {
        self.sprung_buffers = Some(FeatureFlag::Yes);
        self
    }

    /// Build a new technical specifications value
    pub fn build(self) -> TechnicalSpecifications {
        TechnicalSpecifications {
            minimum_radius: self.minimum_radius,
            coupling: self.coupling,
            flywheel_fitted: self.flywheel_fitted,
            body_shell: self.body_shell,
            chassis: self.chassis,
            interior_lights: self.interior_lights,
            lights: self.lights,
            sprung_buffers: self.sprung_buffers,
        }
    }
}

/// A flag to indicate the presence/absence of a given technical specification feature
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type, Default)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "feature_flag", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeatureFlag {
    /// Yes: the feature is present
    Yes,
    /// No: the feature is missing
    No,
    /// The feature is not applicable
    #[default]
    NotApplicable,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "body_shell_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BodyShellType {
    Plastic,
    MetalDieCast,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "chassis_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChassisType {
    Plastic,
    MetalDieCast,
}

/// The minimum drivable radius
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Type)]
#[sqlx(transparent, no_pg_array)]
pub struct Radius(#[serde(with = "common::length::serde::millimeters")] Length);

impl Radius {
    /// Returns a drivable radius expressed in millimeters
    pub fn from_millimeters(value: Decimal) -> Result<Self, RadiusError> {
        if value.is_sign_positive() {
            Ok(Radius(Length::Millimeters(value)))
        } else {
            Err(RadiusError::NegativeRadius)
        }
    }

    /// Returns the value for this radius
    pub fn value(&self) -> Length {
        self.0
    }
}

pub fn validate_radius(input: &Radius) -> Result<(), ValidationError> {
    validate_length_range(&input.0, Some(dec!(0.1)), Some(dec!(9999.0)))
}

impl fmt::Display for Radius {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Error)]
pub enum RadiusError {
    #[error("radius cannot be negative")]
    NegativeRadius,
}

#[cfg(test)]
mod test {
    use super::*;

    mod coupling {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_couplings() {
            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::NotApplicable);

            assert_eq!(coupling.socket(), Some(CouplingSocket::Nem362));
            assert_eq!(coupling.digital_shunting(), Some(FeatureFlag::NotApplicable));
            assert_eq!(coupling.close_couplers(), Some(FeatureFlag::Yes));
        }

        #[test]
        fn it_should_create_close_couplers() {
            let coupling = Coupling::with_close_couplers(CouplingSocket::Nem362);

            assert_eq!(coupling.socket, Some(CouplingSocket::Nem362));
            assert_eq!(coupling.digital_shunting, Some(FeatureFlag::No));
            assert_eq!(coupling.close_couplers, Some(FeatureFlag::Yes));
        }

        #[test]
        fn it_should_create_digital_shunting_couplers() {
            let coupling = Coupling::with_digital_shunting_couplers();

            assert_eq!(coupling.socket, Some(CouplingSocket::None));
            assert_eq!(coupling.digital_shunting, Some(FeatureFlag::Yes));
            assert_eq!(coupling.close_couplers, Some(FeatureFlag::No));
        }
    }

    mod sockets {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("NONE", Ok(CouplingSocket::None))]
        #[case("NEM_355", Ok(CouplingSocket::Nem355))]
        #[case("NEM_356", Ok(CouplingSocket::Nem356))]
        #[case("NEM_357", Ok(CouplingSocket::Nem357))]
        #[case("NEM_359", Ok(CouplingSocket::Nem359))]
        #[case("NEM_360", Ok(CouplingSocket::Nem360))]
        #[case("NEM_362", Ok(CouplingSocket::Nem362))]
        #[case("NEM_365", Ok(CouplingSocket::Nem365))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_couplings(
            #[case] input: &str,
            #[case] expected: Result<CouplingSocket, ParseError>,
        ) {
            let coupling = input.parse::<CouplingSocket>();
            assert_eq!(expected, coupling);
        }

        #[rstest]
        #[case(CouplingSocket::None, "NONE")]
        #[case(CouplingSocket::Nem355, "NEM_355")]
        #[case(CouplingSocket::Nem356, "NEM_356")]
        #[case(CouplingSocket::Nem357, "NEM_357")]
        #[case(CouplingSocket::Nem359, "NEM_359")]
        #[case(CouplingSocket::Nem360, "NEM_360")]
        #[case(CouplingSocket::Nem362, "NEM_362")]
        #[case(CouplingSocket::Nem365, "NEM_365")]
        fn it_should_display_couplings(#[case] input: CouplingSocket, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }

    mod feature_flags {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("YES", Ok(FeatureFlag::Yes))]
        #[case("NO", Ok(FeatureFlag::No))]
        #[case("NOT_APPLICABLE", Ok(FeatureFlag::NotApplicable))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_strings_as_feature_flags(
            #[case] input: &str,
            #[case] expected: Result<FeatureFlag, ParseError>,
        ) {
            let flag = input.parse::<FeatureFlag>();
            assert_eq!(expected, flag);
        }

        #[rstest]
        #[case(FeatureFlag::Yes, "YES")]
        #[case(FeatureFlag::No, "NO")]
        #[case(FeatureFlag::NotApplicable, "NOT_APPLICABLE")]
        fn it_should_display_feature_flags(#[case] input: FeatureFlag, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }

    mod radius {
        use super::*;
        use common::measure_units::MeasureUnit;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_a_new_radius_in_millimeters() {
            let radius = Radius::from_millimeters(dec!(360.0)).expect("unable to create the radius");
            assert_eq!(Length::new(dec!(360), MeasureUnit::Millimeters), radius.value());
        }

        #[test]
        fn it_should_fail_to_create_negative_radius() {
            let result = Radius::from_millimeters(dec!(-1.0));
            assert_eq!(Err(RadiusError::NegativeRadius), result);
        }

        #[test]
        fn it_should_display_a_radius() {
            let radius = Radius::from_millimeters(dec!(360.0)).unwrap();
            assert_eq!("360.0 mm", radius.to_string());
        }

        #[test]
        fn it_should_serialize_radius_as_json() {
            let value = TestStruct {
                radius: Radius::from_millimeters(dec!(360.0)).unwrap(),
            };

            let json = serde_json::to_string(&value).expect("Invalid JSON value");

            assert_eq!(r#"{"radius":360.0}"#, json);
        }

        #[test]
        fn it_should_deserialize_radius_from_json() {
            let json = r#"{"radius":360.0}"#;
            let value = TestStruct {
                radius: Radius::from_millimeters(dec!(360.0)).unwrap(),
            };

            let deserialize_value: TestStruct = serde_json::from_str(json).expect("Invalid JSON value");

            assert_eq!(value, deserialize_value);
        }

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct TestStruct {
            radius: Radius,
        }
    }

    mod technical_specifications {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_tech_specs() {
            let coupling = Coupling::new(CouplingSocket::Nem362, FeatureFlag::Yes, FeatureFlag::No);

            let radius = Radius::from_millimeters(dec!(360)).unwrap();
            let tech_specs = TechnicalSpecificationsBuilder::default()
                .with_coupling(coupling)
                .with_chassis(ChassisType::Plastic)
                .with_body_shell(BodyShellType::MetalDieCast)
                .with_minimum_radius(radius)
                .with_interior_lights()
                .with_lights()
                .with_sprung_buffers()
                .with_flywheel_fitted()
                .build();

            assert_eq!(Some(coupling), tech_specs.coupling());
            assert_eq!(Some(radius), tech_specs.minimum_radius());
            assert_eq!(Some(ChassisType::Plastic), tech_specs.chassis());
            assert_eq!(Some(BodyShellType::MetalDieCast), tech_specs.body_shell());
            assert_eq!(Some(FeatureFlag::Yes), tech_specs.interior_lights());
            assert_eq!(Some(FeatureFlag::Yes), tech_specs.lights());
            assert_eq!(Some(FeatureFlag::Yes), tech_specs.sprung_buffers());
            assert_eq!(Some(FeatureFlag::Yes), tech_specs.flywheel_fitted());
        }
    }

    mod technical_specifications_validation {
        use crate::catalog_items::technical_specifications::{Radius, TechnicalSpecifications};
        use common::length::Length;
        use rust_decimal_macros::dec;
        use validator::Validate;

        #[test]
        fn it_should_validate_technical_specifications() {
            let radius = Radius(Length::Millimeters(dec!(360.0)));
            let tech_specs = TechnicalSpecifications {
                minimum_radius: Some(radius),
                ..TechnicalSpecifications::default()
            };

            let result = tech_specs.validate();
            assert!(result.is_ok());
        }

        #[test]
        fn it_should_failed_to_validate_invalid_radius() {
            let radius = Radius(Length::Millimeters(dec!(-360.0)));
            let tech_specs = TechnicalSpecifications {
                minimum_radius: Some(radius),
                ..TechnicalSpecifications::default()
            };

            let result = tech_specs.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("minimum_radius"));
            assert_eq!(errors["minimum_radius"].len(), 1);
            assert_eq!(errors["minimum_radius"][0].code, "range");
            assert_eq!(errors["minimum_radius"][0].params["value"], -360.0);
            assert_eq!(errors["minimum_radius"][0].params["min"], 0.1);
            assert_eq!(errors["minimum_radius"][0].params["max"], 9999.0);
        }
    }
}
