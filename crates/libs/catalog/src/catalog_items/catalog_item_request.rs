//! the catalog item command request

use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::category::Category;
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock_request::RollingStockRequest;
use common::localized_text::LocalizedText;
use validator::Validate;

/// A request to create/update catalog items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct CatalogItemRequest {
    /// the brand
    #[validate(length(min = 3, max = 50))]
    pub brand: String,
    /// the item number
    #[validate(custom(function = "crate::catalog_items::item_number::validate_item_number"))]
    pub item_number: ItemNumber,
    /// the scale
    #[validate(length(min = 1, max = 50))]
    pub scale: String,
    /// the category
    pub category: Category,
    /// the power method
    pub power_method: PowerMethod,
    /// the epoch
    pub epoch: Epoch,
    /// the catalog item description
    #[validate(nested)]
    pub description: LocalizedText,
    /// the catalog item details
    #[validate(nested)]
    pub details: LocalizedText,
    /// the delivery date
    pub delivery_date: Option<DeliveryDate>,
    /// the availability status
    pub availability_status: Option<AvailabilityStatus>,
    /// the rolling stocks included in this catalog item
    #[validate(nested)]
    pub rolling_stocks: Vec<RollingStockRequest>,
    /// the number of rolling stocks for this catalog item
    #[validate(range(min = 1, max = 99))]
    pub count: i32,
}

#[cfg(test)]
mod test {
    mod catalog_item_request_validation {
        use crate::catalog_items::catalog_item_request::CatalogItemRequest;
        use crate::catalog_items::category::Category;
        use crate::catalog_items::epoch::Epoch;
        use crate::catalog_items::item_number::{ItemNumber, invalid_item_number};
        use crate::catalog_items::power_method::PowerMethod;
        use crate::catalog_items::rolling_stock_request::data::{
            freight_car_request, locomotive_request, passenger_car_request,
        };
        use crate::test_helpers::{random_str, unwrap_map};
        use common::localized_text::LocalizedText;
        use pretty_assertions::assert_eq;
        use rstest::rstest;
        use validator::{Validate, ValidationErrorsKind};

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(1))]
        #[case(random_str(2))]
        #[case(random_str(51))]
        fn it_should_validate_the_brand_name(#[case] input: String) {
            let request = CatalogItemRequest {
                brand: input.clone(),
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("brand"));
            assert_eq!(errors["brand"].len(), 1);
            assert_eq!(errors["brand"][0].code, "length");
            assert_eq!(errors["brand"][0].params["value"], input);
            assert_eq!(errors["brand"][0].params["min"], 3);
            assert_eq!(errors["brand"][0].params["max"], 50);
        }

        #[test]
        fn it_should_validate_the_item_number() {
            let request = CatalogItemRequest {
                item_number: invalid_item_number(),
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("item_number"));
            assert_eq!(errors["item_number"].len(), 1);
            assert_eq!(errors["item_number"][0].code, "length");
            assert_eq!(errors["item_number"][0].params["value"], "");
            assert_eq!(errors["item_number"][0].params["min"], 1);
            assert_eq!(errors["item_number"][0].params["max"], 25);
        }

        #[rstest]
        #[case(random_str(0))]
        #[case(random_str(51))]
        fn it_should_validate_the_scale_name(#[case] input: String) {
            let request = CatalogItemRequest {
                scale: input.clone(),
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("scale"));
            assert_eq!(errors["scale"].len(), 1);
            assert_eq!(errors["scale"][0].code, "length");
            assert_eq!(errors["scale"][0].params["value"], input);
            assert_eq!(errors["scale"][0].params["min"], 1);
            assert_eq!(errors["scale"][0].params["max"], 50);
        }

        #[rstest]
        #[case(0)]
        #[case(100)]
        fn it_should_validate_the_count(#[case] count: i32) {
            let request = CatalogItemRequest {
                count,
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errors = err.field_errors();
            assert_eq!(1, errors.len());
            assert!(errors.contains_key("count"));
            assert_eq!(errors["count"].len(), 1);
            assert_eq!(errors["count"][0].code, "range");
            assert_eq!(errors["count"][0].params["value"], count);
            assert_eq!(errors["count"][0].params["min"], 1.0);
            assert_eq!(errors["count"][0].params["max"], 99.0);
        }

        #[test]
        fn it_should_validate_the_description() {
            let request = CatalogItemRequest {
                description: LocalizedText::with_italian(&random_str(2501)),
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errs = err.errors();
            assert_eq!(1, errs.len());
            assert!(errs.contains_key("description"));
            if let ValidationErrorsKind::Struct(ref errs) = errs["description"] {
                unwrap_map(errs, |errs| {
                    assert_eq!(errs.len(), 1);
                    assert!(errs.contains_key("it"));
                    if let ValidationErrorsKind::Field(ref errs) = errs["it"] {
                        assert_eq!(errs.len(), 1);
                        assert_eq!(errs[0].code, "length");
                    } else {
                        panic!("Expected field validation errors");
                    }
                });
            } else {
                panic!("Expected struct validation errors");
            }
        }

        #[test]
        fn it_should_validate_the_details() {
            let request = CatalogItemRequest {
                details: LocalizedText::with_italian(&random_str(2501)),
                ..catalog_item_request()
            };

            let result = request.validate();
            let err = result.unwrap_err();
            let errs = err.errors();
            assert_eq!(1, errs.len());
            assert!(errs.contains_key("details"));
            if let ValidationErrorsKind::Struct(ref errs) = errs["details"] {
                unwrap_map(errs, |errs| {
                    assert_eq!(errs.len(), 1);
                    assert!(errs.contains_key("it"));
                    if let ValidationErrorsKind::Field(ref errs) = errs["it"] {
                        assert_eq!(errs.len(), 1);
                        assert_eq!(errs[0].code, "length");
                    } else {
                        panic!("Expected field validation errors");
                    }
                });
            } else {
                panic!("Expected struct validation errors");
            }
        }

        fn catalog_item_request() -> CatalogItemRequest {
            CatalogItemRequest {
                brand: "ACME".to_string(),
                item_number: ItemNumber::new("123456"),
                scale: "H0".to_string(),
                category: Category::TrainSets,
                power_method: PowerMethod::DC,
                epoch: Epoch::II,
                description: LocalizedText::with_italian("label"),
                details: LocalizedText::with_italian("label"),
                delivery_date: None,
                availability_status: None,
                rolling_stocks: vec![locomotive_request(), passenger_car_request(), freight_car_request()],
                count: 3,
            }
        }
    }
}
