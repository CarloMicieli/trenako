//! the rolling stock length over buffers

use common::length::{validate_length_range, Length};
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::{Validate, ValidationErrors};

/// The rail vehicle measurement method expressed as the length over buffers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LengthOverBuffers {
    /// the overall length in inches
    #[serde(with = "common::length::serde::inches_option")]
    pub inches: Option<Length>,
    /// the overall length in millimeters
    #[serde(with = "common::length::serde::millimeters_option")]
    pub millimeters: Option<Length>,
}

impl LengthOverBuffers {
    /// Creates a new length over buffers value
    pub fn new(inches: Option<Decimal>, millimeters: Option<Decimal>) -> Result<Self, LengthOverBuffersError> {
        match (inches, millimeters) {
            (Some(inches), _) if inches.is_sign_negative() || inches.is_zero() => {
                Err(LengthOverBuffersError::NonPositiveValue)
            }
            (_, Some(mm)) if mm.is_sign_negative() || mm.is_zero() => Err(LengthOverBuffersError::NonPositiveValue),
            (Some(inches), Some(mm)) if !MeasureUnit::Millimeters.same_as(mm, MeasureUnit::Inches, inches) => {
                Err(LengthOverBuffersError::DifferentValues)
            }
            _ => {
                let inches = inches.map(Length::Inches);
                let millimeters = millimeters.map(Length::Millimeters);
                Ok(LengthOverBuffers { inches, millimeters })
            }
        }
    }

    /// Creates a new length over buffers value in millimeters
    pub fn from_millimeters(millimeters: Length) -> Self {
        let inches = MeasureUnit::Millimeters
            .to(MeasureUnit::Inches)
            .convert(millimeters.quantity());
        LengthOverBuffers {
            inches: Some(Length::Inches(inches)),
            millimeters: Some(millimeters),
        }
    }

    /// Creates a new length over buffers value in inches
    pub fn from_inches(inches: Length) -> Self {
        let millimeters = MeasureUnit::Inches
            .to(MeasureUnit::Millimeters)
            .convert(inches.quantity());
        LengthOverBuffers {
            inches: Some(inches),
            millimeters: Some(Length::Millimeters(millimeters)),
        }
    }

    /// the length over buffers value in inches
    pub fn inches(&self) -> Option<&Length> {
        self.inches.as_ref()
    }

    /// the length over buffers value in millimeters
    pub fn millimeters(&self) -> Option<&Length> {
        self.millimeters.as_ref()
    }
}

impl Validate for LengthOverBuffers {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if let Some(inches) = self.inches && let Err(error) = validate_length_range(&inches, Some(dec!(0.1)), Some(dec!(999.0))) {
            errors.add("inches", error);
        }

        if let Some(millimeters) = self.millimeters && let Err(error) = validate_length_range(&millimeters, Some(dec!(0.1)), Some(dec!(9999.0))) {
            errors.add("millimeters", error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// The length over buffers error enum
#[derive(Debug, PartialEq, Error)]
pub enum LengthOverBuffersError {
    #[error("the value in millimeters is not matching the one in inches")]
    DifferentValues,
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod length_over_buffer_tests {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(None, None, Ok(LengthOverBuffers { inches: None, millimeters: None}))]
        #[case(Some(dec!(0.0)), Some(dec!(0.0)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(-0.65)), Some(dec!(-16.5)), Err(LengthOverBuffersError::NonPositiveValue))]
        #[case(Some(dec!(0.65)), Some(dec!(16.2)), Err(LengthOverBuffersError::DifferentValues))]
        fn it_should_create_new_length_over_buffers_values(
            #[case] inches: Option<Decimal>,
            #[case] millimeters: Option<Decimal>,
            #[case] expected: Result<LengthOverBuffers, LengthOverBuffersError>,
        ) {
            let result = LengthOverBuffers::new(inches, millimeters);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_inches() {
            let inches = Length::Inches(dec!(42));
            let lob = LengthOverBuffers::from_inches(inches);
            assert_eq!(Some(&inches), lob.inches());
            assert_eq!(Some(&Length::Millimeters(dec!(1066.8))), lob.millimeters());
        }

        #[test]
        fn it_should_create_new_length_over_buffer_from_millimeters() {
            let millimeters = Length::Millimeters(dec!(42));
            let lob = LengthOverBuffers::from_millimeters(millimeters);
            assert_eq!(Some(&millimeters), lob.millimeters());
            assert_eq!(Some(&Length::Inches(dec!(1.6535442))), lob.inches());
        }

        #[test]
        fn it_should_serialize_as_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);
            let value = TestStruct {
                length_over_buffers: LengthOverBuffers::new(Some(inches), Some(millimeters))
                    .expect("invalid length over buffers"),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            let expected = r#"{"length_over_buffers":{"inches":0.65,"millimeters":16.5}}"#;
            assert_eq!(expected, json);
        }

        #[test]
        fn it_should_deserialize_from_json() {
            let inches = dec!(0.65);
            let millimeters = dec!(16.5);

            let json = r#"{"length_over_buffers":{"inches":0.65,"millimeters":16.5}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(
                Some(inches),
                test_struct.length_over_buffers.inches.map(|l| l.quantity())
            );
            assert_eq!(
                Some(millimeters),
                test_struct.length_over_buffers.millimeters.map(|l| l.quantity())
            );
        }

        #[test]
        fn it_should_deserialize_from_json_length_over_buffers_with_only_the_inches_value() {
            let inches = dec!(0.65);

            let json = r#"{"length_over_buffers":{"inches":0.65}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(
                Some(inches),
                test_struct.length_over_buffers.inches.map(|l| l.quantity())
            );
            assert_eq!(None, test_struct.length_over_buffers.millimeters);
        }

        #[test]
        fn it_should_deserialize_from_json_length_over_buffers_with_only_the_millimeters_value() {
            let millimeters = dec!(16.5);

            let json = r#"{"length_over_buffers":{"millimeters":16.5}}"#;

            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(None, test_struct.length_over_buffers.inches);
            assert_eq!(
                Some(millimeters),
                test_struct.length_over_buffers.millimeters.map(|l| l.quantity())
            );
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            length_over_buffers: LengthOverBuffers,
        }
    }

    mod length_over_buffer_validation {
        use crate::catalog_items::length_over_buffers::LengthOverBuffers;
        use common::length::Length;
        use rust_decimal_macros::dec;
        use validator::Validate;

        #[test]
        fn it_should_validate_length_over_buffers() {
            let length_over_buffer = LengthOverBuffers::new(Some(dec!(0.65)), Some(dec!(16.5))).unwrap();
            let result = length_over_buffer.validate();
            assert!(result.is_ok());
        }

        #[test]
        fn it_should_fail_to_validate_invalid_length_over_buffers() {
            let millimeters = Length::Millimeters(dec!(-16.5));
            let inches = Length::Inches(dec!(-0.65));
            let length_over_buffer = LengthOverBuffers {
                millimeters: Some(millimeters),
                inches: Some(inches),
            };
            let result = length_over_buffer.validate();
            assert!(result.is_err());
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert!(errors.contains_key("inches"));
            assert_eq!(errors["inches"].len(), 1);
            assert_eq!(errors["inches"][0].code, "range");
            assert_eq!(errors["inches"][0].params["value"], "-0.65");
            assert_eq!(errors["inches"][0].params["min"], 0.1);
            assert_eq!(errors["inches"][0].params["max"], 999.0);

            assert!(errors.contains_key("millimeters"));
            assert_eq!(errors["millimeters"].len(), 1);
            assert_eq!(errors["millimeters"][0].code, "range");
            assert_eq!(errors["millimeters"][0].params["value"], "-16.5");
            assert_eq!(errors["millimeters"][0].params["min"], 0.1);
            assert_eq!(errors["millimeters"][0].params["max"], 9999.0);
        }
    }
}
