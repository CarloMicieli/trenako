//! the manufacturer kinds

use sqlx::Type;
use strum_macros;
use strum_macros::{Display, EnumString};

/// The different kinds for railway models manufacturers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Type, Default)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[strum(ascii_case_insensitive)]
#[sqlx(type_name = "manufacturer_kind", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ManufacturerKind {
    /// These manufacturers produce models which are made of brass or similar alloys.
    ///
    /// They are usually more expensive than the industrial series due to the limited
    /// production quantities and the "hand made" nature of the production
    BrassModels,

    /// These manufactures produce models using the die casting method
    #[default]
    Industrial,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_kinds {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use strum::ParseError;

        #[rstest]
        #[case("BRASS_MODELS", Ok(ManufacturerKind::BrassModels))]
        #[case("INDUSTRIAL", Ok(ManufacturerKind::Industrial))]
        #[case("invalid", Err(ParseError::VariantNotFound))]
        fn it_should_parse_manufacturer_kinds(
            #[case] input: &str,
            #[case] expected: Result<ManufacturerKind, ParseError>,
        ) {
            let manufacturer_kind = input.parse::<ManufacturerKind>();
            assert_eq!(expected, manufacturer_kind);
        }

        #[rstest]
        #[case(ManufacturerKind::BrassModels, "BRASS_MODELS")]
        #[case(ManufacturerKind::Industrial, "INDUSTRIAL")]
        fn it_should_display_manufacturer_kinds(#[case] input: ManufacturerKind, #[case] expected: &str) {
            assert_eq!(expected, input.to_string());
        }

        #[test]
        fn it_should_define_a_default_manufacturer_kind() {
            let kind = ManufacturerKind::default();
            assert_eq!(ManufacturerKind::Industrial, kind);
        }
    }
}
