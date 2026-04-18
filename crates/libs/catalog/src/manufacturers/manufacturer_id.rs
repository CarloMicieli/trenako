//! the manufacturer identifier

use common::slug::{Slug, SlugParserError};
use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::ops;
use std::str;
use std::str::FromStr;

/// It represents the unique identifier for a manufacturer.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize, Type)]
#[sqlx(transparent)]
pub struct ManufacturerId(Slug);

impl ManufacturerId {
    /// Creates a new manufacturer unique identifier
    ///
    /// # Panics
    /// Panics if `id` is not a valid value (ie, blank string)
    pub fn new(id: &str) -> Self {
        ManufacturerId::from_str(id).expect("invalid manufacturer id")
    }
}

impl fmt::Display for ManufacturerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl str::FromStr for ManufacturerId {
    type Err = SlugParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::from_str(s).map(ManufacturerId)
    }
}

impl ops::Deref for ManufacturerId {
    type Target = Slug;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturer_ids {
        use super::*;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn it_should_create_new_manufacturer_ids() {
            let manufacturer_id = ManufacturerId::new("manufacturer name");
            assert_eq!("manufacturer-name", manufacturer_id.to_string());
        }

        #[test]
        fn it_should_return_an_error_when_the_manufacturer_id_is_empty() {
            let result = ManufacturerId::from_str("");
            assert!(result.is_err());
        }

        #[test]
        fn it_should_create_new_manufacturer_ids_from_str() {
            let manufacturer_id = ManufacturerId::from_str("manufacturer name").unwrap();
            assert_eq!("manufacturer-name", manufacturer_id.to_string());
        }

        #[test]
        fn it_should_check_whether_two_manufacturer_ids_are_equal() {
            let id1 = ManufacturerId::new("manufacturer name");
            let id2 = ManufacturerId::new("manufacturer name");
            let id3 = ManufacturerId::new("another manufacturer name");

            assert_eq!(id1, id1);
            assert_eq!(id1, id2);
            assert_ne!(id1, id3);
        }

        #[test]
        fn it_should_compare_two_manufacturer_ids() {
            let id1 = ManufacturerId::new("manufacturer 1");
            let id2 = ManufacturerId::new("manufacturer 2");

            assert!(id1 < id2);
            assert!(id2 > id1);
        }
    }
}
