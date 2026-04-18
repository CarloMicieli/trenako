//! the manufacturer status

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The current status for a model railway company
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, EnumString, Display, Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "manufacturer_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManufacturerStatus {
    /// the manufacturer is active
    Active,

    /// the manufacturer is out of business
    OutOfBusiness,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_statuses {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("ACTIVE", Ok(ManufacturerStatus::Active))]
        #[case("OUT_OF_BUSINESS", Ok(ManufacturerStatus::OutOfBusiness))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_manufacturer_statuses(
            #[case] input: &str,
            #[case] expected: Result<ManufacturerStatus, ParseError>,
        ) {
            let status = input.parse::<ManufacturerStatus>();
            assert_eq!(expected, status);
        }

        #[rstest]
        #[case(ManufacturerStatus::Active, "ACTIVE")]
        #[case(ManufacturerStatus::OutOfBusiness, "OUT_OF_BUSINESS")]
        fn it_should_display_manufacturer_status(#[case] input: ManufacturerStatus, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }
    }
}
