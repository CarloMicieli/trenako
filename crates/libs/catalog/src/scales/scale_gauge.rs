//! the scale gauge

use crate::common::TrackGauge;
use common::length::{Length, validate_length_range};
use common::measure_units::MeasureUnit;
use common::measure_units::MeasureUnit::Millimeters;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::cmp::Ordering;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

/// It represents the track gauge information for a modelling scale
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Gauge {
    /// the distance between the rails in millimeters
    #[serde(with = "common::length::serde::millimeters")]
    pub millimeters: Length,
    /// the distance between the rails in inches
    #[serde(with = "common::length::serde::inches")]
    pub inches: Length,
    /// the track gauge
    pub track_gauge: TrackGauge,
}

impl Validate for Gauge {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if let Err(error) = validate_length_range(&self.millimeters, Some(dec!(6.5)), Some(dec!(200.0))) {
            errors.add("millimeters", error);
        }

        if let Err(error) = validate_length_range(&self.inches, Some(dec!(0.01)), Some(dec!(15.00))) {
            errors.add("inches", error);
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

impl Gauge {
    /// Creates a new scale gauge
    pub fn new(track_gauge: TrackGauge, millimeters: Decimal, inches: Decimal) -> Result<Self, GaugeError> {
        match (millimeters, inches) {
            (mm, _) if mm.is_sign_negative() || mm.is_zero() => {
                Err(GaugeError::NegativeRailsDistance(mm, MeasureUnit::Millimeters))
            }
            (_, inches) if inches.is_sign_negative() || inches.is_zero() => {
                Err(GaugeError::NegativeRailsDistance(inches, MeasureUnit::Inches))
            }
            (mm, inches) if !Millimeters.same_as(mm, MeasureUnit::Inches, inches) => Err(GaugeError::DifferentValues),
            (_, _) => Ok(Gauge {
                millimeters: Length::Millimeters(millimeters),
                inches: Length::Inches(inches),
                track_gauge,
            }),
        }
    }

    /// Creates a new scale gauge from the distance between the rails in inches
    pub fn from_inches(track_gauge: TrackGauge, inches: Decimal) -> Result<Self, GaugeError> {
        let millimeters = MeasureUnit::Inches.to(MeasureUnit::Millimeters).convert(inches);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// Creates a new scale gauge from the distance between the rails in millimeters
    pub fn from_millimeters(track_gauge: TrackGauge, millimeters: Decimal) -> Result<Self, GaugeError> {
        let inches = MeasureUnit::Millimeters.to(MeasureUnit::Inches).convert(millimeters);
        Gauge::new(track_gauge, millimeters, inches)
    }

    /// the distance between the rails in millimeters
    pub fn millimeters(&self) -> Length {
        self.millimeters
    }

    /// the distance between the rails in inches
    pub fn inches(&self) -> Length {
        self.inches
    }

    /// the track gauge
    pub fn track_gauge(&self) -> TrackGauge {
        self.track_gauge
    }

    /// The gauge for the HO scale
    pub const H0: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(16.5)),
        inches: Length::Inches(dec!(0.65)),
    };

    /// The gauge for the N scale
    pub const N: Gauge = Gauge {
        track_gauge: TrackGauge::Standard,
        millimeters: Length::Millimeters(dec!(9.0)),
        inches: Length::Inches(dec!(0.354)),
    };
}

impl cmp::PartialOrd for Gauge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.millimeters.partial_cmp(&other.millimeters)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum GaugeError {
    #[error("the distance between rails must be positive ({0} {1})")]
    NegativeRailsDistance(Decimal, MeasureUnit),
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scale_gauges {
        use super::*;
        use crate::common::TrackGauge;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_scale_gauges() {
            let millimeters = dec!(16.5);
            let inches = dec!(0.65);
            let gauge = Gauge::new(TrackGauge::Standard, millimeters, inches).expect("Invalid scale gauge");

            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.5), gauge.millimeters().quantity());
            assert_eq!(dec!(0.65), gauge.inches().quantity());
        }

        #[rstest]
        #[case(dec!(-16.5), dec!(-0.65), Err(GaugeError::NegativeRailsDistance(dec!(-16.5), MeasureUnit::Millimeters)))]
        #[case(dec!(0.0), dec!(0.0), Err(GaugeError::NegativeRailsDistance(dec!(0.0), MeasureUnit::Millimeters)))]
        fn it_should_validate_the_input_creating_scale_gauge(
            #[case] millimeters: Decimal,
            #[case] inches: Decimal,
            #[case] expected: Result<Gauge, GaugeError>,
        ) {
            let result = Gauge::new(TrackGauge::Standard, millimeters, inches);
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case(dec!(16.5), dec!(0.75), Err(GaugeError::DifferentValues))]
        fn it_should_validate_the_input_creating_scale_gauge_are_the_same_after_conversion(
            #[case] millimeters: Decimal,
            #[case] inches: Decimal,
            #[case] expected: Result<Gauge, GaugeError>,
        ) {
            let result = Gauge::new(TrackGauge::Standard, millimeters, inches);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_scale_gauge_from_millimeters() {
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).expect("invalid scale gauge");
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.5), gauge.millimeters().quantity());
            assert_eq!(dec!(0.64960665), gauge.inches().quantity());
        }

        #[test]
        fn it_should_create_scale_gauge_from_inches() {
            let gauge = Gauge::from_inches(TrackGauge::Standard, dec!(0.65)).expect("invalid scale gauge");
            assert_eq!(TrackGauge::Standard, gauge.track_gauge());
            assert_eq!(dec!(16.510), gauge.millimeters().quantity());
            assert_eq!(dec!(0.65), gauge.inches().quantity());
        }

        #[test]
        fn it_should_compare_two_gauges() {
            let gauge1 = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).expect("invalid scale gauge");
            let gauge2 = Gauge::from_millimeters(TrackGauge::Standard, dec!(9.0)).expect("invalid scale gauge");

            assert!(gauge1 > gauge2);
            assert!(gauge2 < gauge1);
        }

        #[test]
        fn it_should_serialize_scale_gauges_as_json() {
            let value = TestStruct {
                gauge: Gauge::new(TrackGauge::Standard, dec!(16.5), dec!(0.65)).expect("invalid scale gauge"),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(
                r#"{"gauge":{"millimeters":16.5,"inches":0.65,"track_gauge":"STANDARD"}}"#,
                json
            );
        }

        #[test]
        fn it_should_deserialize_scale_gauges_from_json() {
            let json = r#"{"gauge":{"track_gauge":"STANDARD","millimeters":"16.5","inches":"0.65"}}"#;
            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(TrackGauge::Standard, test_struct.gauge.track_gauge);
            assert_eq!(dec!(16.5), test_struct.gauge.millimeters.quantity());
            assert_eq!(dec!(0.65), test_struct.gauge.inches.quantity());
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            gauge: Gauge,
        }
    }

    mod scale_gauge_validation {
        use super::*;

        #[test]
        fn it_should_validate_gauges() {
            let input = Gauge {
                millimeters: Length::Millimeters(dec!(16.5)),
                inches: Length::Millimeters(dec!(0.65)),
                track_gauge: TrackGauge::Standard,
            };

            let result = input.validate();

            assert!(result.is_ok());
        }

        #[test]
        fn it_should_validate_inches() {
            let input = Gauge {
                millimeters: Length::Millimeters(dec!(1.0)),
                inches: Length::Millimeters(dec!(-1.0)),
                track_gauge: TrackGauge::Standard,
            };

            let result = input.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("inches"));
            assert_eq!(errors["inches"].len(), 1);
            assert_eq!(errors["inches"][0].code, "range");
            assert_eq!(errors["inches"][0].params["value"], "-1.0");
            assert_eq!(errors["inches"][0].params["min"], 0.01);
            assert_eq!(errors["inches"][0].params["max"], 15.00);
        }

        #[test]
        fn it_should_validate_millimeters() {
            let input = Gauge {
                millimeters: Length::Millimeters(dec!(-1.0)),
                inches: Length::Millimeters(dec!(1.0)),
                track_gauge: TrackGauge::Standard,
            };

            let result = input.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("millimeters"));
            assert_eq!(errors["millimeters"].len(), 1);
            assert_eq!(errors["millimeters"][0].code, "range");
            assert_eq!(errors["millimeters"][0].params["value"], "-1.0");
            assert_eq!(errors["millimeters"][0].params["min"], 6.5);
            assert_eq!(errors["millimeters"][0].params["max"], 200.0);
        }
    }
}
