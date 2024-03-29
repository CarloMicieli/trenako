//! the modelling scale ratio

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Serialize, Serializer};
use sqlx::Type;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Formatter;
use std::{cmp, convert, fmt, ops};
use thiserror::Error;
use validator::ValidationError;

/// It represents the {@code Ratio} between a model railway size
/// and the size of an actual train.
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Type)]
#[sqlx(transparent)]
pub struct Ratio(Decimal);

pub fn validate_ratio(input: &Ratio) -> Result<(), ValidationError> {
    let value = input.0;

    if dec!(1.0) <= value && value <= dec!(220.0) {
        Ok(())
    } else {
        let mut error = ValidationError::new("ratio");
        error.add_param(Cow::from("min"), &Some(1));
        error.add_param(Cow::from("max"), &Some(220));
        error.add_param(Cow::from("value"), &value);
        Err(error)
    }
}

impl convert::TryFrom<Decimal> for Ratio {
    type Error = RatioError;

    fn try_from(value: Decimal) -> Result<Self, Self::Error> {
        match value {
            _ if value.is_sign_negative() => Err(RatioError::NonPositiveValue(value)),
            _ if value.is_zero() => Err(RatioError::NonPositiveValue(value)),
            _ if value > dec!(220) => Err(RatioError::OutsideAllowedRange),
            _ if value < Decimal::ONE => Err(RatioError::OutsideAllowedRange),
            _ => Ok(Ratio(value)),
        }
    }
}

impl Serialize for Ratio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        rust_decimal::serde::float::serialize(&self.0, serializer)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum RatioError {
    #[error("scale ratios must be positive (value: {0})")]
    NonPositiveValue(Decimal),
    #[error("scale ratios must be included in the 1-220 range")]
    OutsideAllowedRange,
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "1:{}", self.0)
    }
}

impl ops::Deref for Ratio {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl convert::AsRef<Decimal> for Ratio {
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

impl cmp::PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ratios {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_ratios() {
            let value = dec!(87);
            assert_eq!(Ok(Ratio(value)), Ratio::try_from(value));
        }

        #[test]
        fn it_should_dereference_ratios() {
            let value = dec!(87);
            let ratio = Ratio(value);

            assert_eq!(&value, ratio.as_ref());
            assert_eq!(value, *ratio);
        }

        #[rstest]
        #[case(dec!(0))]
        #[case(dec!(-1))]
        fn it_should_only_allow_non_negative_ratios(#[case] input: Decimal) {
            let result = Ratio::try_from(input);
            assert_eq!(Err(RatioError::NonPositiveValue(input)), result);
        }

        #[rstest]
        #[case(dec!(0.9), Err(RatioError::OutsideAllowedRange))]
        #[case(dec!(1.0), Ok(Ratio(dec!(1.0))))]
        #[case(dec!(220.0), Ok(Ratio(dec!(220.0))))]
        #[case(dec!(221.0), Err(RatioError::OutsideAllowedRange))]
        fn it_should_check_if_the_input_is_inside_the_allowed_range(
            #[case] input: Decimal,
            #[case] expected: Result<Ratio, RatioError>,
        ) {
            let result = Ratio::try_from(input);
            assert_eq!(expected, result);
        }

        #[test]
        fn it_should_display_ratios() {
            let ratio1 = Ratio::try_from(dec!(87));
            assert_eq!("1:87", ratio1.unwrap().to_string());
        }

        #[test]
        fn it_should_compare_two_ratios() {
            let ratio1 = Ratio::try_from(dec!(87)).unwrap();
            let ratio2 = Ratio::try_from(dec!(160)).unwrap();

            assert!(ratio1 > ratio2, "1:87 > 1:160 must hold true");
            assert!(ratio2 < ratio1, "1:160 < 1:87 must hold true");
        }
    }

    mod ratio_validation {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_validate_ratios() {
            let ratio = Ratio(dec!(-1.0));
            let result = validate_ratio(&ratio);
            let error = result.unwrap_err();
            assert_eq!(error.code, "ratio");
            assert_eq!(error.params["value"], "-1.0");
            assert_eq!(error.params["min"], 1);
            assert_eq!(error.params["max"], 220);
        }
    }

    mod ratio_serialization {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_serialize_ratios() {
            let value = TestStruct {
                ratio: Ratio::try_from(dec!(43.5)).unwrap(),
            };

            let json = serde_json::to_string(&value).expect("invalid json value");

            assert_eq!(r#"{"ratio":43.5}"#, json);
        }

        #[derive(Debug, Serialize)]
        struct TestStruct {
            ratio: Ratio,
        }
    }
}
