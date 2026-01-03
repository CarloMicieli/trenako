//! the catalog item delivery date

use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;
use std::fmt::Formatter;
use std::str;
use std::str::FromStr;
use thiserror::Error;

/// The delivery date quarter number
pub type Quarter = u8;
/// The delivery date month number
pub type Month = u8;
/// The delivery date year
pub type Year = i32;

/// A delivery date for a catalog item (either by quarter or just year).
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DeliveryDate {
    /// A delivery date with just a year (ie, _"2022"_)
    ByYear(Year),
    /// A delivery date with year and quarter (ie, _"2022/Q1"_)
    ByQuarter(Year, Quarter),
    /// A delivery date with year and month (ie, "2022/12")
    ByYearMonth(Year, Month),
}

impl DeliveryDate {
    /// Creates a new delivery date without the quarter nor month
    pub fn by_year(year: Year) -> Self {
        DeliveryDate::ByYear(year)
    }

    /// Creates a new delivery date with the quarter/year information
    pub fn by_quarter(year: Year, quarter: Quarter) -> Self {
        DeliveryDate::ByQuarter(year, quarter)
    }

    /// Creates a new delivery date with the month/year information
    pub fn by_month(year: Year, month: Month) -> Self {
        DeliveryDate::ByQuarter(year, month)
    }

    /// Returns the year component from this delivery date
    pub fn year(&self) -> Year {
        match self {
            DeliveryDate::ByQuarter(y, _) => *y,
            DeliveryDate::ByYear(y) => *y,
            DeliveryDate::ByYearMonth(y, _) => *y,
        }
    }

    /// Returns the (optional) quarter component from this delivery date
    pub fn quarter(&self) -> Option<Quarter> {
        match self {
            DeliveryDate::ByQuarter(_, q) => Some(*q),
            DeliveryDate::ByYear(_) => None,
            DeliveryDate::ByYearMonth(_, _) => None,
        }
    }

    /// Returns the (optional) month component from this delivery date
    pub fn month(&self) -> Option<Month> {
        match self {
            DeliveryDate::ByQuarter(_, _) => None,
            DeliveryDate::ByYear(_) => None,
            DeliveryDate::ByYearMonth(_, m) => Some(*m),
        }
    }

    fn parse_year(s: &str) -> Result<Year, DeliveryDateParseError> {
        let year = s
            .parse::<Year>()
            .map_err(|_| DeliveryDateParseError::InvalidYearValue)?;
        if !(1900..=2999).contains(&year) {
            return Err(DeliveryDateParseError::InvalidYearValue);
        }

        Ok(year)
    }

    fn parse_quarter(s: &str) -> Result<Quarter, DeliveryDateParseError> {
        if s.len() != 2 {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        let quarter = s[1..]
            .parse::<Quarter>()
            .map_err(|_| DeliveryDateParseError::InvalidQuarterValue)?;
        if !(1..=4).contains(&quarter) {
            return Err(DeliveryDateParseError::InvalidQuarterValue);
        }

        Ok(quarter)
    }

    fn parse_month(s: &str) -> Result<Quarter, DeliveryDateParseError> {
        if s.len() != 1 && s.len() != 2 {
            return Err(DeliveryDateParseError::InvalidMonthValue);
        }

        let month = s
            .parse::<Month>()
            .map_err(|_| DeliveryDateParseError::InvalidMonthValue)?;
        if !(1..=12).contains(&month) {
            return Err(DeliveryDateParseError::InvalidMonthValue);
        }

        Ok(month)
    }
}

impl str::FromStr for DeliveryDate {
    type Err = DeliveryDateParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(DeliveryDateParseError::EmptyValue);
        }

        if s.contains("/Q") {
            let tokens: Vec<&str> = s.split_terminator('/').collect();
            if tokens.len() != 2 {
                return Err(DeliveryDateParseError::InvalidDeliveryDateFormat);
            }

            let year = DeliveryDate::parse_year(tokens[0])?;
            let quarter = DeliveryDate::parse_quarter(tokens[1])?;
            Ok(DeliveryDate::ByQuarter(year, quarter))
        } else if s.contains('/') {
            let tokens: Vec<&str> = s.split_terminator('/').collect();
            if tokens.len() != 2 {
                return Err(DeliveryDateParseError::InvalidDeliveryDateFormat);
            }

            let year = DeliveryDate::parse_year(tokens[0])?;
            let month = DeliveryDate::parse_month(tokens[1])?;
            Ok(DeliveryDate::ByYearMonth(year, month))
        } else {
            let year = DeliveryDate::parse_year(s)?;
            Ok(DeliveryDate::ByYear(year))
        }
    }
}

impl fmt::Display for DeliveryDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DeliveryDate::ByQuarter(y, q) => write!(f, "{y}/Q{q}"),
            DeliveryDate::ByYear(y) => y.fmt(f),
            DeliveryDate::ByYearMonth(y, m) => write!(f, "{y}/{m:0>2}"),
        }
    }
}

impl Serialize for DeliveryDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct DeliveryDateVisitor;

impl<'de> Visitor<'de> for DeliveryDateVisitor {
    type Value = DeliveryDate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the input is not a valid delivery date")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(dd) = DeliveryDate::from_str(s) {
            Ok(dd)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for DeliveryDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeliveryDateVisitor)
    }
}

/// The delivery date parsing errors enum
#[derive(Debug, Error, PartialEq)]
pub enum DeliveryDateParseError {
    #[error("Delivery date cannot be empty")]
    EmptyValue,
    #[error("Invalid format for a delivery date")]
    InvalidDeliveryDateFormat,
    #[error("Delivery date year component is not valid")]
    InvalidYearValue,
    #[error("Delivery date quarter component is not valid")]
    InvalidQuarterValue,
    #[error("Delivery date month component is not valid")]
    InvalidMonthValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod delivery_dates {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[test]
        fn it_should_parse_year_string_as_delivery_dates() {
            let dd = "2020".parse::<DeliveryDate>();
            assert!(dd.is_ok());

            let dd = dd.unwrap();
            assert_eq!(2020, dd.year());
            assert_eq!(None, dd.quarter());
            assert_eq!(None, dd.month());
        }

        #[test]
        fn it_should_parse_year_quarter_string_as_delivery_dates() {
            let dd = "2020/Q1".parse::<DeliveryDate>();
            assert!(dd.is_ok());

            let dd = dd.unwrap();
            assert_eq!(2020, dd.year());
            assert_eq!(Some(1), dd.quarter());
            assert_eq!(None, dd.month());
        }

        #[rstest]
        #[case("2020/1", 1)]
        #[case("2020/11", 11)]
        fn it_should_parse_year_month_string_as_delivery_dates(#[case] input: &str, #[case] expected: u8) {
            let dd = input.parse::<DeliveryDate>();
            assert!(dd.is_ok());

            let dd = dd.unwrap();
            assert_eq!(2020, dd.year());
            assert_eq!(None, dd.quarter());
            assert_eq!(Some(expected), dd.month());
        }

        #[test]
        fn it_should_produce_string_representations_from_delivery_dates() {
            let dd1 = "2020/Q1".parse::<DeliveryDate>().unwrap();
            let dd2 = "2020".parse::<DeliveryDate>().unwrap();
            let dd3 = "2020/11".parse::<DeliveryDate>().unwrap();
            let dd4 = "2020/2".parse::<DeliveryDate>().unwrap();

            assert_eq!("2020/Q1", dd1.to_string());
            assert_eq!("2020", dd2.to_string());
            assert_eq!("2020/11", dd3.to_string());
            assert_eq!("2020/02", dd4.to_string());
        }

        #[rstest]
        #[case("2020/Q1", r#""2020/Q1""#)]
        #[case("2020", r#""2020""#)]
        #[case("2020/11", r#""2020/11""#)]
        fn it_should_serialize_delivery_dates(#[case] input: &str, #[case] expected: &str) {
            let dd1 = input.parse::<DeliveryDate>().unwrap();
            let result = serde_json::to_string(&dd1).unwrap();
            assert_eq!(expected, result);
        }

        #[rstest]
        #[case("2020/Q11", DeliveryDateParseError::InvalidQuarterValue)]
        #[case("2020/Q0", DeliveryDateParseError::InvalidQuarterValue)]
        #[case("2020/Q5", DeliveryDateParseError::InvalidQuarterValue)]
        #[case("2020/QA", DeliveryDateParseError::InvalidQuarterValue)]
        #[case("202/Q1", DeliveryDateParseError::InvalidYearValue)]
        #[case("1899/Q1", DeliveryDateParseError::InvalidYearValue)]
        #[case("3000/Q1", DeliveryDateParseError::InvalidYearValue)]
        #[case("3000/Q1/?", DeliveryDateParseError::InvalidDeliveryDateFormat)]
        #[case("2022/13", DeliveryDateParseError::InvalidMonthValue)]
        #[case("2022/0", DeliveryDateParseError::InvalidMonthValue)]
        fn it_should_fail_to_parse_invalid_delivery_dates(
            #[case] input: &str,
            #[case] expected: DeliveryDateParseError,
        ) {
            let result = input.parse::<DeliveryDate>();
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), expected);
        }

        #[test]
        fn it_should_deserialize_delivery_dates() {
            let test_struct = TestStruct {
                just_year: DeliveryDate::ByYear(2022),
                with_quarter: DeliveryDate::ByQuarter(2022, 3),
                with_month: DeliveryDate::ByYearMonth(2022, 11),
            };

            let json = serde_json::json!(test_struct);

            let result: serde_json::Result<TestStruct> = serde_json::from_str(&json.to_string());
            assert_eq!(test_struct, result.unwrap());
        }

        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
        pub struct TestStruct {
            pub just_year: DeliveryDate,
            pub with_quarter: DeliveryDate,
            pub with_month: DeliveryDate,
        }
    }
}
