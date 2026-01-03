//! the module includes everything related to lengths

use crate::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, Zero};
use std::borrow::Cow;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::ops;
use thiserror::Error;
use validator::ValidationError;

/// It represents a length.
///
/// Lengths are defined by a non-negative value and a measure unit.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Length {
    /// a length expressed in Inches
    Inches(Decimal),
    /// a length expressed in Kilometers
    Kilometers(Decimal),
    /// a length expressed in Meters
    Meters(Decimal),
    /// a length expressed in Miles
    Miles(Decimal),
    /// a length expressed in Millimeters
    Millimeters(Decimal),
}

pub fn validate_length_range(
    input: &Length,
    min: Option<Decimal>,
    max: Option<Decimal>,
) -> Result<(), ValidationError> {
    let value = input.quantity();
    if min.unwrap_or(Decimal::ZERO) < value && value < max.unwrap_or(Decimal::MAX) {
        Ok(())
    } else {
        let mut error = ValidationError::new("range");
        error.add_param(Cow::from("min"), &min.and_then(|it| it.to_f64()));

        if max.is_some() {
            error.add_param(Cow::from("max"), &max.and_then(|it| it.to_f64()));
        }

        error.add_param(Cow::from("value"), &input.quantity());
        Err(error)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum LengthError {
    #[error("invalid length value")]
    InvalidValue(#[from] rust_decimal::Error),
    #[error("length values cannot be negative")]
    NegativeValue,
}

impl Length {
    /// Returns a `Length` value with a given measure unit  
    ///
    /// # Panics
    ///
    /// This function panics if `value` is < 0.
    pub fn new(value: Decimal, measure_unit: MeasureUnit) -> Self {
        Self::try_new(value, measure_unit).expect("invalid length value")
    }

    /// Checked version of `Length::new`. Will return `Err` instead of panicking at run-time.
    pub fn try_new(value: Decimal, measure_unit: MeasureUnit) -> Result<Self, LengthError> {
        if value.is_sign_negative() {
            Err(LengthError::NegativeValue)
        } else {
            let length = match measure_unit {
                MeasureUnit::Millimeters => Length::Millimeters(value),
                MeasureUnit::Inches => Length::Inches(value),
                MeasureUnit::Meters => Length::Meters(value),
                MeasureUnit::Miles => Length::Miles(value),
                MeasureUnit::Kilometers => Length::Kilometers(value),
            };
            Ok(length)
        }
    }

    /// this `Length` quantity
    pub fn quantity(&self) -> Decimal {
        match self {
            Length::Millimeters(mm) => *mm,
            Length::Inches(ins) => *ins,
            Length::Meters(m) => *m,
            Length::Miles(mi) => *mi,
            Length::Kilometers(km) => *km,
        }
    }

    /// this `Length` measure unit
    pub fn measure_unit(&self) -> MeasureUnit {
        match self {
            Length::Millimeters(_) => MeasureUnit::Millimeters,
            Length::Inches(_) => MeasureUnit::Inches,
            Length::Meters(_) => MeasureUnit::Meters,
            Length::Miles(_) => MeasureUnit::Miles,
            Length::Kilometers(_) => MeasureUnit::Kilometers,
        }
    }

    /// Returns this `Length` expressed in the `measure_unit` converting the value if needed
    pub fn get_value_as(&self, measure_unit: MeasureUnit) -> Decimal {
        if self.measure_unit() == measure_unit {
            self.quantity()
        } else {
            self.measure_unit().to(measure_unit).convert(self.quantity())
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Millimeters(Decimal::zero())
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity(), self.measure_unit().symbol())
    }
}

impl ops::Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        let (val1, mu1) = (self.quantity(), self.measure_unit());
        let (val2, mu2) = (rhs.quantity(), rhs.measure_unit());

        let new_value = val1 + mu2.to(mu1).convert(val2);

        Length::new(new_value, self.measure_unit())
    }
}

impl cmp::PartialEq for Length {
    fn eq(&self, other: &Self) -> bool {
        let value1 = self.quantity();
        let value2 = other.get_value_as(self.measure_unit());
        value1 == value2
    }
}

impl cmp::Eq for Length {}

impl cmp::PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let value1 = self.quantity();
        let value2 = other.get_value_as(self.measure_unit());
        value1.partial_cmp(&value2)
    }
}

pub mod serde {
    use crate::length::Length;
    use crate::measure_units::MeasureUnit;

    fn serialize_length_option<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let quantity = value.map(|len| len.quantity());
        rust_decimal::serde::float_option::serialize(&quantity, serializer)
    }

    fn deserialize_length_option<'de, D>(measure_unit: MeasureUnit, deserializer: D) -> Result<Option<Length>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let quantity = rust_decimal::serde::float_option::deserialize(deserializer)?;
        match quantity {
            None => Ok(None),
            Some(qty) => {
                let length =
                    Length::try_new(qty, measure_unit).map_err(|why| serde::de::Error::custom(why.to_string()))?;
                Ok(Some(length))
            }
        }
    }

    fn serialize_length<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let quantity = value.quantity();
        rust_decimal::serde::float::serialize(&quantity, serializer)
    }

    fn deserialize_length<'de, D>(measure_unit: MeasureUnit, deserializer: D) -> Result<Length, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let quantity = rust_decimal::serde::float::deserialize(deserializer)?;
        Length::try_new(quantity, measure_unit).map_err(|why| serde::de::Error::custom(why.to_string()))
    }

    pub mod kilometers {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length(MeasureUnit::Kilometers, deserializer)
        }

        pub fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length(value, serializer)
        }
    }

    pub mod kilometers_option {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length_option(MeasureUnit::Kilometers, deserializer)
        }

        pub fn serialize<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length_option(value, serializer)
        }
    }

    pub mod inches {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length(MeasureUnit::Inches, deserializer)
        }

        pub fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length(value, serializer)
        }
    }

    pub mod inches_option {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length_option(MeasureUnit::Inches, deserializer)
        }

        pub fn serialize<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length_option(value, serializer)
        }
    }

    pub mod meters {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length(MeasureUnit::Meters, deserializer)
        }

        pub fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length(value, serializer)
        }
    }

    pub mod meters_option {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length_option(MeasureUnit::Meters, deserializer)
        }

        pub fn serialize<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length_option(value, serializer)
        }
    }

    pub mod miles {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length(MeasureUnit::Miles, deserializer)
        }

        pub fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length(value, serializer)
        }
    }

    pub mod miles_option {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length_option(MeasureUnit::Miles, deserializer)
        }

        pub fn serialize<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length_option(value, serializer)
        }
    }

    pub mod millimeters {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Length, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length(MeasureUnit::Millimeters, deserializer)
        }

        pub fn serialize<S>(value: &Length, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length(value, serializer)
        }
    }

    pub mod millimeters_option {
        use super::*;
        use crate::length::Length;

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Length>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            deserialize_length_option(MeasureUnit::Millimeters, deserializer)
        }

        pub fn serialize<S>(value: &Option<Length>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serialize_length_option(value, serializer)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use rust_decimal::prelude::FromPrimitive;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_lengths() {
            let l = Length::new(dec!(42.), MeasureUnit::Millimeters);
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_ensure_lengths_are_non_negative() {
            assert_eq!(
                Err(LengthError::NegativeValue),
                Length::try_new(dec!(-1.), MeasureUnit::Inches)
            );
            assert_eq!(
                Ok(Length::default()),
                Length::try_new(Decimal::ZERO, MeasureUnit::Millimeters)
            );
        }

        #[rstest]
        #[case(42.0f32, MeasureUnit::Inches, "42 in")]
        #[case(42.0f32, MeasureUnit::Meters, "42 m")]
        #[case(42.0f32, MeasureUnit::Millimeters, "42 mm")]
        #[case(42.0f32, MeasureUnit::Miles, "42 mi")]
        #[case(42.0f32, MeasureUnit::Kilometers, "42 km")]
        fn it_should_display_lengths(#[case] input: f32, #[case] measure_unit: MeasureUnit, #[case] expected: &str) {
            let value = Decimal::from_f32(input).unwrap();
            let length = Length::new(value, measure_unit);
            assert_eq!(expected, length.to_string());
        }

        #[test]
        fn it_should_sum_two_lengths() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_sum_two_lengths_converting_measure_units() {
            let l1 = Length::new(dec!(16.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(1.0), MeasureUnit::Inches);

            let l = l1 + l2;
            assert_eq!(dec!(42.0), l.quantity());
            assert_eq!(MeasureUnit::Millimeters, l.measure_unit());
        }

        #[test]
        fn it_should_compare_two_lengths() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);

            assert_eq!(l1, l1);
            assert_ne!(l1, l2);
        }

        #[test]
        fn it_should_sort_length_values() {
            let l1 = Length::new(dec!(20.6), MeasureUnit::Millimeters);
            let l2 = Length::new(dec!(21.4), MeasureUnit::Millimeters);
            let l3 = Length::new(dec!(1.0), MeasureUnit::Meters);

            assert!(l1 < l2);
            assert!(l2 > l1);
            assert!(l3 > l1);
        }
    }

    mod lengths_validation {
        use super::*;
        use rstest::rstest;
        use rust_decimal_macros::dec;

        #[rstest]
        #[case(Length::Inches(dec!(42.0)))]
        #[case(Length::Meters(dec!(42.0)))]
        #[case(Length::Millimeters(dec!(42.0)))]
        #[case(Length::Miles(dec!(42.0)))]
        #[case(Length::Kilometers(dec!(42.0)))]
        fn it_should_validate_lengths(#[case] l1: Length) {
            let result = validate_length_range(&l1, None, None);
            assert!(result.is_ok());
        }

        #[rstest]
        #[case(Length::Inches(dec!(-42.0)))]
        #[case(Length::Meters(dec!(-42.0)))]
        #[case(Length::Millimeters(dec!(-42.0)))]
        #[case(Length::Miles(dec!(-42.0)))]
        #[case(Length::Kilometers(dec!(-42.0)))]
        fn it_should_validate_negative_lengths(#[case] l1: Length) {
            let result = validate_length_range(&l1, Some(dec!(1.0)), Some(dec!(99.9)));

            let error = result.unwrap_err();
            assert_eq!(error.code, "range");
            assert_eq!(error.params["value"], "-42.0");
            assert_eq!(error.params["min"], 1.0);
            assert_eq!(error.params["max"], 99.9);
        }
    }

    mod serde {
        use crate::length::Length;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;
        use serde_derive::Deserialize;
        use serde_derive::Serialize;

        #[test]
        fn it_should_serialize_lengths() {
            let value = TestStruct::new();

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(
                r#"{"inches":1234.56,"kilometers":1234.56,"meters":1234.56,"miles":1234.56,"millimeters":1234.56}"#,
                json
            )
        }

        #[test]
        fn it_should_deserialize_lengths() {
            let json =
                r#"{"inches":1234.56,"kilometers":1234.56,"meters":1234.56,"miles":1234.56,"millimeters":1234.56}"#;

            let value_from_json: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(value_from_json, TestStruct::new());
        }

        #[test]
        fn it_should_fail_to_deserialize_invalid_lengths() {
            let json = r#"{"inches":-1234.56,"kilometers":-1234.56,"meters":-1234.56,"miles":-1234.56,"millimeters":-1234.56}"#;

            let result = serde_json::from_str::<TestStruct>(json);

            assert!(result.is_err());
            assert_eq!(
                "length values cannot be negative at line 1 column 18",
                result.err().unwrap().to_string()
            );
        }

        #[test]
        fn it_should_serialize_optional_lengths() {
            let value = TestStructOptional::new();

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(
                r#"{"inches":1234.56,"kilometers":1234.56,"meters":1234.56,"miles":1234.56,"millimeters":1234.56}"#,
                json
            )
        }

        #[test]
        fn it_should_deserialize_optional_lengths() {
            let json =
                r#"{"inches":1234.56,"kilometers":1234.56,"meters":1234.56,"miles":1234.56,"millimeters":1234.56}"#;

            let value_from_json: TestStructOptional = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(value_from_json, TestStructOptional::new());
        }

        #[test]
        fn it_should_deserialize_empty_values_as_optional_lengths() {
            let json = r#"{"inches":null,"kilometers":null,"meters":null,"miles":null,"millimeters":null}"#;

            let value_from_json: TestStructOptional = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(value_from_json, TestStructOptional::default());
        }

        #[test]
        fn it_should_fail_to_deserialize_invalid_optional_lengths() {
            let json = r#"{"inches":-1234.56,"kilometers":-1234.56,"meters":-1234.56,"miles":-1234.56,"millimeters":-1234.56}"#;

            let result = serde_json::from_str::<TestStructOptional>(json);

            assert!(result.is_err());
            assert_eq!(
                "length values cannot be negative at line 1 column 18",
                result.err().unwrap().to_string()
            );
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct TestStruct {
            #[serde(with = "crate::length::serde::inches")]
            inches: Length,
            #[serde(with = "crate::length::serde::kilometers")]
            kilometers: Length,
            #[serde(with = "crate::length::serde::meters")]
            meters: Length,
            #[serde(with = "crate::length::serde::miles")]
            miles: Length,
            #[serde(with = "crate::length::serde::millimeters")]
            millimeters: Length,
        }

        impl TestStruct {
            fn new() -> Self {
                TestStruct {
                    inches: Length::Inches(dec!(1234.56)),
                    kilometers: Length::Kilometers(dec!(1234.56)),
                    meters: Length::Meters(dec!(1234.56)),
                    miles: Length::Miles(dec!(1234.56)),
                    millimeters: Length::Millimeters(dec!(1234.56)),
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
        struct TestStructOptional {
            #[serde(with = "crate::length::serde::inches_option")]
            inches: Option<Length>,
            #[serde(with = "crate::length::serde::kilometers_option")]
            kilometers: Option<Length>,
            #[serde(with = "crate::length::serde::meters_option")]
            meters: Option<Length>,
            #[serde(with = "crate::length::serde::miles_option")]
            miles: Option<Length>,
            #[serde(with = "crate::length::serde::millimeters_option")]
            millimeters: Option<Length>,
        }

        impl TestStructOptional {
            fn new() -> Self {
                TestStructOptional {
                    inches: Some(Length::Inches(dec!(1234.56))),
                    kilometers: Some(Length::Kilometers(dec!(1234.56))),
                    meters: Some(Length::Meters(dec!(1234.56))),
                    miles: Some(Length::Miles(dec!(1234.56))),
                    millimeters: Some(Length::Millimeters(dec!(1234.56))),
                }
            }
        }
    }
}
