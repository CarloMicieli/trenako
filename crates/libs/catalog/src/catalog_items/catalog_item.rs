//! the catalog item view model

use crate::manufacturers::manufacturer::Manufacturer;
use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::category::Category;
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock::RollingStock;
use crate::scales::scale::Scale;
use crate::scales::scale_id::ScaleId;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use std::fmt::Formatter;
use std::{cmp, convert, fmt};

/// A catalog item, it can contain one or more rolling stock.
///
/// A catalog item is identified by its catalog item number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    /// the unique identifier for this catalog item
    pub catalog_item_id: CatalogItemId,
    /// the manufacturer
    pub manufacturer: CatalogItemManufacturer,
    /// the item number
    pub item_number: ItemNumber,
    /// the scale
    pub scale: CatalogItemScale,
    /// the category
    pub category: Category,
    /// the power method
    pub power_method: PowerMethod,
    /// the epoch for this catalog item
    pub epoch: Epoch,
    /// the catalog item description
    pub description: LocalizedText,
    /// the catalog item details
    pub details: LocalizedText,
    /// the delivery date
    pub delivery_date: Option<DeliveryDate>,
    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,
    /// the rolling stocks included in this catalog item
    pub rolling_stocks: Vec<RollingStock>,
    /// the number of rolling stocks for this catalog item
    pub count: u8,
    /// the metadata
    pub metadata: Metadata,
}

impl cmp::PartialEq for CatalogItem {
    fn eq(&self, other: &Self) -> bool {
        self.manufacturer == other.manufacturer && self.item_number == other.item_number
    }
}

impl cmp::Eq for CatalogItem {}

impl cmp::PartialOrd for CatalogItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for CatalogItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let cmp1 = self.manufacturer().cmp(other.manufacturer());
        if cmp1 == cmp::Ordering::Equal {
            return self.item_number.cmp(&other.item_number);
        }

        cmp1
    }
}

impl CatalogItem {
    /// Creates a new catalog item
    pub fn new(
        catalog_item_id: CatalogItemId,
        manufacturer: CatalogItemManufacturer,
        item_number: ItemNumber,
        category: Category,
        scale: CatalogItemScale,
        description: Option<&str>,
        details: Option<&str>,
        rolling_stocks: Vec<RollingStock>,
        power_method: PowerMethod,
        epoch: Epoch,
        delivery_date: Option<DeliveryDate>,
        availability_status: Option<AvailabilityStatus>,
        count: u8,
        metadata: Metadata,
    ) -> Self {
        CatalogItem {
            catalog_item_id,
            manufacturer,
            item_number,
            category,
            description: description.map(LocalizedText::with_italian).unwrap_or_default(),
            details: details.map(LocalizedText::with_italian).unwrap_or_default(),
            scale,
            power_method,
            epoch,
            rolling_stocks,
            delivery_date,
            availability_status,
            count,
            metadata,
        }
    }

    /// the unique catalog item id
    pub fn id(&self) -> &CatalogItemId {
        &self.catalog_item_id
    }

    /// the manufacturer for this catalog item.
    pub fn manufacturer(&self) -> &CatalogItemManufacturer {
        &self.manufacturer
    }

    /// the item number as in the corresponding manufacturer catalog.
    pub fn item_number(&self) -> &ItemNumber {
        &self.item_number
    }

    /// the category for this catalog item
    pub fn category(&self) -> Category {
        self.category
    }

    /// the rolling stocks count for this catalog item
    pub fn count(&self) -> u8 {
        self.count
    }

    /// the list of rolling stocks for this catalog item
    pub fn rolling_stocks(&self) -> &Vec<RollingStock> {
        &self.rolling_stocks
    }

    /// the description for this catalog item
    pub fn description(&self) -> Option<&String> {
        self.description.italian()
    }

    /// the details for this catalog item
    pub fn details(&self) -> Option<&String> {
        self.details.italian()
    }

    /// the scale for this catalog item
    pub fn scale(&self) -> &CatalogItemScale {
        &self.scale
    }

    /// the power method for this catalog item
    pub fn power_method(&self) -> PowerMethod {
        self.power_method
    }

    /// the catalog item epoch
    pub fn epoch(&self) -> &Epoch {
        &self.epoch
    }

    /// the delivery date for this catalog item
    pub fn delivery_date(&self) -> Option<&DeliveryDate> {
        self.delivery_date.as_ref()
    }

    /// the availability status for this catalog item
    pub fn availability_status(&self) -> Option<&AvailabilityStatus> {
        self.availability_status.as_ref()
    }

    /// the metadata for this catalog item
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// The model railways manufacturer for a catalog item
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct CatalogItemManufacturer {
    /// the manufacturer unique identifier
    pub manufacturer_id: ManufacturerId,
    /// the manufacturer display text
    pub display: String,
}

impl CatalogItemManufacturer {
    /// Creates a new manufacturer with the given display text.
    pub fn new(manufacturer_id: ManufacturerId, display: &str) -> Self {
        CatalogItemManufacturer {
            manufacturer_id,
            display: display.to_owned(),
        }
    }

    /// this manufacturer unique identifier
    pub fn manufacturer_id(&self) -> &ManufacturerId {
        &self.manufacturer_id
    }

    /// this manufacturer display text
    pub fn display(&self) -> &str {
        &self.display
    }
}

impl fmt::Display for CatalogItemManufacturer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display)
    }
}

impl convert::From<Manufacturer> for CatalogItemManufacturer {
    fn from(value: Manufacturer) -> Self {
        CatalogItemManufacturer {
            manufacturer_id: value.manufacturer_id().clone(),
            display: value.to_string(),
        }
    }
}

/// The modelling scale for a catalog item
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct CatalogItemScale {
    /// the scale unique identifier
    pub scale_id: ScaleId,
    /// the scale display text
    pub display: String,
}

impl CatalogItemScale {
    /// Creates a new scale with the given display text.
    pub fn new(scale_id: ScaleId, display: &str) -> Self {
        CatalogItemScale {
            scale_id,
            display: display.to_owned(),
        }
    }

    /// this scale unique identifier
    pub fn scale_id(&self) -> &ScaleId {
        &self.scale_id
    }

    /// this scale display text
    pub fn display(&self) -> &str {
        &self.display
    }
}

impl fmt::Display for CatalogItemScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.display)
    }
}

impl convert::From<Scale> for CatalogItemScale {
    fn from(value: Scale) -> Self {
        CatalogItemScale {
            scale_id: value.scale_id().clone(),
            display: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod catalog_item_manufacturers {
        use super::*;
        use crate::manufacturers::test_data::acme;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_manufacturers() {
            let b = CatalogItemManufacturer::new(ManufacturerId::new("ACME"), "ACME");
            assert_eq!(&ManufacturerId::new("ACME"), b.manufacturer_id());
            assert_eq!("ACME", b.display());
        }

        #[test]
        fn it_should_display_manufacturer_as_string() {
            let b = CatalogItemManufacturer::new(ManufacturerId::new("ACME"), "ACME");
            assert_eq!("ACME", b.to_string());
        }

        #[test]
        fn it_should_convert_from_manufacturers() {
            let b: CatalogItemManufacturer = acme().into();
            assert_eq!(&ManufacturerId::new("ACME"), b.manufacturer_id());
            assert_eq!("ACME", b.display());
        }
    }

    mod catalog_item_scales {
        use super::*;
        use crate::scales::test_data::h0;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_scales() {
            let s = CatalogItemScale::new(ScaleId::new("H0"), "H0 (1:87)");
            assert_eq!(&ScaleId::new("H0"), s.scale_id());
            assert_eq!("H0 (1:87)", s.display());
        }

        #[test]
        fn it_should_display_scale_as_string() {
            let s = CatalogItemScale::new(ScaleId::new("H0"), "H0");
            assert_eq!("H0", s.to_string());
        }

        #[test]
        fn it_should_convert_from_scales() {
            let s: CatalogItemScale = h0().into();
            assert_eq!(&ScaleId::new("H0"), s.scale_id());
            assert_eq!("H0 (1:87)", s.display());
        }
    }

    mod catalog_items {
        use super::*;
        use chrono::{DateTime, Utc};
        use pretty_assertions::assert_eq;
        use std::str::FromStr;

        #[test]
        fn it_should_create_new_catalog_items() {
            let id = CatalogItemId::from_str("acme_123456").unwrap();
            let acme = CatalogItemManufacturer::new(ManufacturerId::new("ACME"), "ACME");
            let half_zero = CatalogItemScale::new(ScaleId::new("H0"), "H0");
            let item_number = ItemNumber::new("123456");
            let now: DateTime<Utc> = Utc::now();

            let catalog_item = CatalogItem::new(
                id.clone(),
                acme.clone(),
                item_number.clone(),
                Category::Locomotives,
                half_zero.clone(),
                Some("test description"),
                Some("test details"),
                Vec::new(),
                PowerMethod::DC,
                Epoch::IV,
                Some(DeliveryDate::ByYear(2000)),
                Some(AvailabilityStatus::Available),
                1,
                Metadata::created_at(now),
            );

            assert_eq!(&id, catalog_item.id());
            assert_eq!(&acme, catalog_item.manufacturer());
            assert_eq!(&item_number, catalog_item.item_number());
            assert_eq!(Category::Locomotives, catalog_item.category());
            assert_eq!(Some(&String::from("test description")), catalog_item.description());
            assert_eq!(Some(&String::from("test details")), catalog_item.details());
            assert_eq!(PowerMethod::DC, catalog_item.power_method());
            assert_eq!(&half_zero, catalog_item.scale());
            assert_eq!(Some(&DeliveryDate::ByYear(2000)), catalog_item.delivery_date());
            assert_eq!(Some(&AvailabilityStatus::Available), catalog_item.availability_status());
            assert_eq!(1, catalog_item.count());
            assert_eq!(&Metadata::created_at(now), catalog_item.metadata());
        }
    }
}
