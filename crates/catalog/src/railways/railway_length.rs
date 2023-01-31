use common::length::Length;
use common::measure_units::MeasureUnit;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// The overall length of tracks (in km and miles) operated by a railway company
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct RailwayLength {
    /// the total railway network in kilometers
    #[serde(with = "common::length::serde::kilometers")]
    pub kilometers: Length,
    /// the total railway network in miles
    #[serde(with = "common::length::serde::miles")]
    pub miles: Length,
}

impl RailwayLength {
    /// Creates a new railway length
    pub fn new(kilometers: Decimal, miles: Decimal) -> Self {
        RailwayLength {
            kilometers: Length::Kilometers(kilometers),
            miles: Length::Miles(miles),
        }
    }

    /// Creates a new railway length from the kilometers value
    pub fn of_kilometers(kilometers: Decimal) -> Self {
        let miles = MeasureUnit::Kilometers.to(MeasureUnit::Miles).convert(kilometers);
        RailwayLength::new(kilometers, miles)
    }

    /// Creates a new railway length from the miles value
    pub fn of_miles(miles: Decimal) -> Self {
        let kilometers = MeasureUnit::Miles.to(MeasureUnit::Kilometers).convert(miles);
        RailwayLength::new(kilometers, miles)
    }

    /// Returns the length of track in Kilometers
    pub fn kilometers(&self) -> Length {
        self.kilometers
    }

    /// Returns the length of track in Miles
    pub fn miles(&self) -> Length {
        self.miles
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum RailwayLengthError {
    #[error("the value in kilometers is not matching the one in miles")]
    DifferentValues,
    #[error("The length over buffers must be positive")]
    NonPositiveValue,
}

impl Display for RailwayLength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "kilometers: {}, miles: {}", self.kilometers, self.miles)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_lengths {
        use super::*;
        use pretty_assertions::assert_eq;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railway_lengths() {
            let miles = Length::Miles(dec!(100));
            let kilometers = Length::Kilometers(dec!(100));
            let len = RailwayLength::new(dec!(100), dec!(100));
            assert_eq!(miles, len.miles());
            assert_eq!(kilometers, len.kilometers());
        }

        #[test]
        fn it_should_display_a_railway_length_value() {
            let miles = dec!(100);
            let kilometers = dec!(100);
            let len = RailwayLength::new(kilometers, miles);
            assert_eq!("kilometers: 100 km, miles: 100 mi", len.to_string());
        }

        #[test]
        fn it_should_create_a_railway_length_from_kilometers() {
            let kilometers = dec!(100);
            let length = RailwayLength::of_kilometers(kilometers);

            assert_eq!(Length::Kilometers(kilometers), length.kilometers());
            assert_eq!(Length::Miles(dec!(62.137100)), length.miles());
        }

        #[test]
        fn it_should_create_a_railway_length_from_miles() {
            let miles = dec!(100);
            let length = RailwayLength::of_miles(miles);

            assert_eq!(Length::Kilometers(dec!(160.93400)), length.kilometers());
            assert_eq!(Length::Miles(miles), length.miles());
        }

        #[test]
        fn it_should_serialize_railway_lengths_as_json() {
            let value = TestStruct {
                railway_length: RailwayLength::new(dec!(100), dec!(62.1)),
            };

            let json = serde_json::to_string(&value).expect("invalid JSON value");

            assert_eq!(r#"{"railway_length":{"kilometers":100.0,"miles":62.1}}"#, json);
        }

        #[test]
        fn it_should_deserialize_railway_lengths_from_json() {
            let json = r#"{"railway_length":{"kilometers":100,"miles":62.1}}"#;
            let test_struct: TestStruct = serde_json::from_str(json).expect("Invalid test struct");

            assert_eq!(Length::Kilometers(dec!(100)), test_struct.railway_length.kilometers());
            assert_eq!(Length::Miles(dec!(62.1)), test_struct.railway_length.miles());
        }

        #[derive(Serialize, Deserialize)]
        struct TestStruct {
            railway_length: RailwayLength,
        }
    }
}
