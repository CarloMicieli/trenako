use crate::dataset::{Dataset, Resource, ResourceType};
use crate::schemas;
use jsonschema::{Draft, Validator};
use serde_derive::Serialize;
use serde_json::Value;
use std::str::FromStr;
use thiserror::Error;

/// Validate a resources dataset
pub fn validate_dataset(dataset: Dataset) -> Result<Vec<Validated>, ValidatorError> {
    let validators = Validators::new()?;

    let it: Vec<Validated> = dataset
        .manufacturers
        .iter()
        .map(|it| validators.validate(it))
        .chain(dataset.catalog_items.iter().map(|it| validators.validate(it)))
        .chain(dataset.railways.iter().map(|it| validators.validate(it)))
        .chain(dataset.scales.iter().map(|it| validators.validate(it)))
        .collect();

    Ok(it)
}

/// It represents a resource validator, against a JSON schema.
pub struct JsonSchemaValidator(Validator);

impl JsonSchemaValidator {
    /// Creates a new schema validator for the dataset resources
    pub fn new(schema: &str) -> Result<Self, ValidatorError> {
        let json_schema = json_schema_from_str(schema)?;
        Ok(JsonSchemaValidator(json_schema))
    }

    /// Validate the resource with the current validator schema
    pub fn validate(&self, input: &Resource) -> Result<Validated, ValidatorError> {
        let input_json = Value::from_str(&input.content)?;
        let result = self.0.validate(&input_json);
        let result = if let Err(validation_errors) = result {
            let mut errors = Vec::new();
            // `jsonschema::ValidationError` changed shape in newer versions
            // and is not directly iterable. Aggregate the validation
            // information into a single error message to keep behavior
            // deterministic across versions.
            let error = Error::new("", &validation_errors.to_string());
            errors.push(error);

            let Resource {
                file_name,
                resource_type,
                content: _,
            } = input;

            Validated::Invalid {
                file_name: file_name.to_string(),
                resource_type: *resource_type,
                errors,
            }
        } else {
            Validated::Valid
        };
        Ok(result)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum Validated {
    Valid,
    Invalid {
        file_name: String,
        resource_type: ResourceType,
        errors: Vec<Error>,
    },
}

impl Validated {
    pub fn is_valid(&self) -> bool {
        matches!(self, Validated::Valid)
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Error {
    pub path: String,
    pub error_message: String,
}

impl Error {
    pub fn new(path: &str, error_message: &str) -> Self {
        Error {
            path: path.to_string(),
            error_message: error_message.to_string(),
        }
    }
}

fn json_schema_from_str(input: &str) -> Result<Validator, ValidatorError> {
    let schema = serde_json::from_str(input)?;
    Validator::options()
        .with_draft(Draft::Draft7)
        .build(&schema)
        .map_err(|_| ValidatorError::InvalidSchema)
}

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error("invalid json value")]
    InvalidJson(#[from] serde_json::Error),

    #[error("invalid json schema")]
    InvalidSchema,
}

pub struct Validators {
    manufacturers: JsonSchemaValidator,
    catalog_items: JsonSchemaValidator,
    railways: JsonSchemaValidator,
    scales: JsonSchemaValidator,
}

impl Validators {
    /// Creates a new schema validator for the dataset resources
    pub fn new() -> Result<Self, ValidatorError> {
        let manufacturers = JsonSchemaValidator::new(schemas::BRANDS_SCHEMA)?;
        let catalog_items = JsonSchemaValidator::new(schemas::CATALOG_ITEMS_SCHEMA)?;
        let railways = JsonSchemaValidator::new(schemas::RAILWAYS_SCHEMA)?;
        let scales = JsonSchemaValidator::new(schemas::SCALES_SCHEMA)?;

        Ok(Self {
            manufacturers,
            catalog_items,
            railways,
            scales,
        })
    }

    pub fn validate(&self, input: &Resource) -> Validated {
        let result = match input.resource_type {
            ResourceType::Manufacturers => self.manufacturers.validate(input),
            ResourceType::CatalogItems => self.catalog_items.validate(input),
            ResourceType::Railways => self.railways.validate(input),
            ResourceType::Scales => self.scales.validate(input),
        };
        result.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod validator {
        use super::*;
        use crate::dataset::ResourceType;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_parse_the_json_schemas() {
            assert!(json_schema_from_str(schemas::BRANDS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::CATALOG_ITEMS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::RAILWAYS_SCHEMA).is_ok());
            assert!(json_schema_from_str(schemas::SCALES_SCHEMA).is_ok());
        }

        fn resource_from_json(value: &str, resource_type: ResourceType) -> Resource {
            Resource {
                file_name: "test_resource".to_string(),
                resource_type,
                content: value.to_string(),
            }
        }

        #[test]
        fn it_should_validate_a_valid_manufacturer() {
            let manufacturer_value = resource_from_json(
                r#"
                {
                  "name" : "ACME",
                  "registered_company_name" : "Associazione Costruzioni Modellistiche Esatte",
                  "organization_entity_type" : "OTHER",
                  "group_name" : null,
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "address" : {
                    "street_address" : "Viale Lombardia, 27",
                    "extended_address" : null,
                    "postal_code" : "20131",
                    "city" : "Milano",
                    "region" : "MI",
                    "country" : "IT"
                  },
                  "contact_info" : {
                    "email" : "mail@acmetreni.com",
                    "phone" : null,
                    "website_url" : "http://www.acmetreni.com"
                  },
                  "socials" : {
                    "facebook" : null,
                    "instagram" : null,
                    "linkedin" : null,
                    "twitter" : null,
                    "youtube" : null
                  },
                  "kind" : "INDUSTRIAL",
                  "status" : "ACTIVE"
                }"#,
                ResourceType::Manufacturers,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate(&manufacturer_value);
            assert_eq!(Validated::Valid, result);
        }

        #[test]
        fn it_should_validate_a_valid_catalog_item() {
            let catalog_item_value = resource_from_json(
                r#"
                {
                  "manufacturer" : "ACME",
                  "item_number" : "60023",
                  "scale" : "H0",
                  "category" : "LOCOMOTIVES",
                  "description" : {
                    "it" : "XMPR FS Trenitalia logo verde/rosso, corrimani e prese frontali, pantografi 52/92",
                    "en" : null
                  },
                  "details" : {
                    "it" : null,
                    "en" : null
                  },
                  "power_method" : "DC",
                  "epoch" : "VI",
                  "delivery_date" : "2022",
                  "availability_status" : "AVAILABLE",
                  "rolling_stocks" : [ {
                    "category" : "LOCOMOTIVES",
                    "class_name" : "E402 A",
                    "road_number" : "E402 031",
                    "series" : "",
                    "locomotive_type" : "ELECTRIC_LOCOMOTIVE",
                    "railway" : "FS",
                    "depot" : "",
                    "dcc_interface" : "MTC_21",
                    "control" : "DCC_READY",
                    "livery" : "",
                    "length_over_buffer" : {
                      "inches" : null,
                      "millimeters" : 210.0
                    },
                    "technical_specifications" : {
                      "minimum_radius" : 360.0,
                      "coupling" : {
                        "socket" : "NEM_362",
                        "close_couplers" : "NO",
                        "digital_shunting" : "NO"
                      },
                      "flywheel_fitted" : "NO",
                      "body_shell" : "PLASTIC",
                      "chassis" : "METAL_DIE_CAST",
                      "interior_lights" : "NO",
                      "lights" : "YES",
                      "sprung_buffers" : "NO"
                    },
                    "is_dummy" : false
                  } ],
                  "count" : 1
                }"#,
                ResourceType::CatalogItems,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate(&catalog_item_value);
            assert_eq!(Validated::Valid, result);
        }

        #[test]
        fn it_should_validate_a_valid_railway() {
            let railway_value = resource_from_json(
                r#"
                {
                  "name" : "FS",
                  "abbreviation" : "FS",
                  "registered_company_name" : "Ferrovie dello Stato Italiane S.p.A.",
                  "organization_entity_type" : "STATE_OWNED_ENTERPRISE",
                  "country" : "IT",
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "period_of_activity" : {
                    "operating_since" : "1905-07-01",
                    "operating_until" : null,
                    "status" : "ACTIVE"
                  },
                  "gauge" : {
                    "track_gauge" : "STANDARD",
                    "meters" : 1.435
                  },
                  "total_length" : {
                    "kilometers" : 24564.0,
                    "miles" : null
                  },
                  "contact_info" : {
                    "email" : null,
                    "website_url" : "https://www.fsitaliane.it",
                    "phone" : null
                  },
                  "social" : {
                    "facebook" : null,
                    "instagram" : "fsitaliane",
                    "linkedin" : "ferrovie-dello-stato-s-p-a-",
                    "twitter" : "FSitaliane",
                    "youtube" : "fsitaliane"
                  },
                  "headquarters" : [ "Roma" ]
                }"#,
                ResourceType::Railways,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate(&railway_value);
            assert_eq!(Validated::Valid, result);
        }

        #[test]
        fn it_should_validate_a_valid_scale() {
            let scale_value = resource_from_json(
                r#"
                {
                  "name" : "H0",
                  "description" : {
                    "it" : null,
                    "en" : null
                  },
                  "ratio" : 87.0,
                  "gauge" : {
                    "millimeters" : 16.5,
                    "inches" : 0.65,
                    "track_gauge" : "STANDARD"
                  },
                  "standards" : [ "NEM" ]
                }
            "#,
                ResourceType::Scales,
            );

            let validator = Validators::new().unwrap();
            let result = validator.validate(&scale_value);
            assert_eq!(Validated::Valid, result);
        }
    }
}
