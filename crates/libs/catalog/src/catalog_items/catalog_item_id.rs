//! the catalog item identifier

use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::catalog_items::catalog_item::CatalogItemManufacturer;
use crate::catalog_items::item_number::ItemNumber;
use common::slug::{Slug, SlugParserError};
use sqlx::Type;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

/// A unique identifier for a catalog item
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash, Type)]
#[sqlx(transparent)]
pub struct CatalogItemId(Slug);

impl CatalogItemId {
    /// Creates a new catalog item id from its manufacturer and item number
    pub fn new(manufacturer: CatalogItemManufacturer, item_number: ItemNumber) -> Self {
        let slug = manufacturer.manufacturer_id().combine(item_number);
        CatalogItemId(slug)
    }

    /// Creates a new catalog item id from its manufacturer and item number
    pub fn of(manufacturer_id: &ManufacturerId, item_number: &ItemNumber) -> Self {
        let slug = Slug::new(&format!("{manufacturer_id}-{item_number}"));
        CatalogItemId(slug)
    }

    /// Returns the value for this catalog item id
    pub fn value(&self) -> &str {
        self.0.deref()
    }
}

impl fmt::Display for CatalogItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl FromStr for CatalogItemId {
    type Err = SlugParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Slug::from_str(s).map(CatalogItemId)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_ids {
        use super::*;
        use crate::manufacturers::manufacturer_id::ManufacturerId;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_catalog_items() {
            let manufacturer_id = ManufacturerId::new("ACME");
            let item_number = ItemNumber::new("12345");

            let id = CatalogItemId::of(&manufacturer_id, &item_number);

            assert_eq!("acme-12345", id.value());
        }

        #[test]
        fn it_should_create_new_catalog_item_ids() {
            let manufacturer = CatalogItemManufacturer::new(ManufacturerId::new("acme"), "ACME");
            let item_number = ItemNumber::new("12345");

            let id = CatalogItemId::new(manufacturer, item_number);

            assert_eq!("acme-12345", id.value());
        }

        #[test]
        fn it_should_display_catalog_item_ids() {
            let id: CatalogItemId = CatalogItemId::from_str("acme-60000").unwrap();
            assert_eq!("acme-60000", id.value());
            assert_eq!("acme-60000", id.to_string());
        }
    }
}
