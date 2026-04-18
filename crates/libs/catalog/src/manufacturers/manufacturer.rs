//! the manufacturer view models

use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::manufacturers::manufacturer_kind::ManufacturerKind;
use crate::manufacturers::manufacturer_status::ManufacturerStatus;
use common::address::Address;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use std::{cmp, fmt};

/// It represents a model railways manufacturer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manufacturer {
    /// the manufacturer unique identifier (an url encoded string)
    pub manufacturer_id: ManufacturerId,
    /// the name
    pub name: String,
    /// the registered company name
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the group name in case the manufacturer is part of a group
    pub group_name: Option<String>,
    /// the description
    pub description: LocalizedText,
    /// the manufacturer main address
    pub address: Option<Address>,
    /// the contact information
    pub contact_info: Option<ContactInformation>,
    /// the manufacturer kind
    pub kind: ManufacturerKind,
    /// the manufacturer status
    pub status: Option<ManufacturerStatus>,
    /// the manufacturer social profiles
    pub socials: Option<Socials>,
    /// the manufacturer metadata
    pub metadata: Metadata,
}

impl Manufacturer {
    /// Creates a new modelling rail manufacturer
    pub fn new(
        manufacturer_id: ManufacturerId,
        name: &str,
        registered_company_name: Option<&str>,
        organization_entity_type: Option<OrganizationEntityType>,
        group_name: Option<&str>,
        description: Option<&str>,
        address: Option<Address>,
        contact_info: Option<ContactInformation>,
        kind: ManufacturerKind,
        status: Option<ManufacturerStatus>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        Manufacturer {
            manufacturer_id,
            name: String::from(name),
            registered_company_name: registered_company_name.map(String::from),
            organization_entity_type,
            group_name: group_name.map(String::from),
            description: description.map(LocalizedText::with_italian).unwrap_or_default(),
            address,
            contact_info,
            kind,
            status,
            socials,
            metadata,
        }
    }

    /// this manufacturer unique identifier (an url encoded string)
    pub fn manufacturer_id(&self) -> &ManufacturerId {
        &self.manufacturer_id
    }

    /// this manufacturer name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// this manufacturer description
    pub fn description(&self) -> Option<&String> {
        self.description.italian()
    }

    /// this manufacturer registered company name
    pub fn registered_company_name(&self) -> Option<&String> {
        self.registered_company_name.as_ref()
    }

    /// the organization entity type
    pub fn organization_entity_type(&self) -> Option<OrganizationEntityType> {
        self.organization_entity_type
    }

    /// this manufacturer group name (if any)
    pub fn group_name(&self) -> Option<&String> {
        self.group_name.as_ref()
    }

    /// the contact information (email, phone, website url)
    pub fn contact_info(&self) -> Option<&ContactInformation> {
        self.contact_info.as_ref()
    }

    /// the postal address
    pub fn address(&self) -> Option<&Address> {
        self.address.as_ref()
    }

    /// this manufacturer status
    pub fn status(&self) -> Option<&ManufacturerStatus> {
        self.status.as_ref()
    }

    /// this manufacturer kind
    pub fn kind(&self) -> ManufacturerKind {
        self.kind
    }

    /// the social profiles
    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// the metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl cmp::PartialEq for Manufacturer {
    fn eq(&self, other: &Self) -> bool {
        self.manufacturer_id.eq(&other.manufacturer_id)
    }
}

impl cmp::Eq for Manufacturer {}

#[cfg(test)]
mod tests {
    use super::*;

    mod manufacturers {
        use super::*;
        use crate::manufacturers::test_data::{acme, roco};
        use chrono::{DateTime, Utc};
        use common::contacts::{MailAddress, WebsiteUrl};
        use isocountry::CountryCode;
        use pretty_assertions::{assert_eq, assert_ne};

        #[test]
        fn it_should_create_manufacturers() {
            let now: DateTime<Utc> = Utc::now();
            let address = Address::builder()
                .street_address("Viale Lombardia, 27")
                .postal_code("20131")
                .city("Milano")
                .region("MI")
                .country(CountryCode::ITA)
                .build()
                .unwrap();

            let contact_info = ContactInformation::new(
                Some(MailAddress::new("mail@acmetreni.com")),
                Some(WebsiteUrl::new("http://www.acmetreni.com")),
                None,
            );

            let socials = Socials::builder().facebook("Acmetreni").build().unwrap();

            let manufacturer = Manufacturer::new(
                ManufacturerId::new("ACME"),
                "ACME",
                Some("Associazione Costruzioni Modellistiche Esatte"),
                Some(OrganizationEntityType::LimitedCompany),
                None,
                None,
                Some(address.clone()),
                Some(contact_info.clone()),
                ManufacturerKind::Industrial,
                Some(ManufacturerStatus::Active),
                Some(socials.clone()),
                Metadata::created_at(now),
            );

            assert_eq!("ACME", manufacturer.to_string());

            assert_eq!(&ManufacturerId::new("ACME"), manufacturer.manufacturer_id());
            assert_eq!("ACME", manufacturer.name());
            assert_eq!(
                Some(&"Associazione Costruzioni Modellistiche Esatte".to_string()),
                manufacturer.registered_company_name()
            );
            assert_eq!(
                Some(OrganizationEntityType::LimitedCompany),
                manufacturer.organization_entity_type()
            );
            assert_eq!(None, manufacturer.group_name());
            assert_eq!(None, manufacturer.description());
            assert_eq!(ManufacturerKind::Industrial, manufacturer.kind());
            assert_eq!(Some(&ManufacturerStatus::Active), manufacturer.status());
            assert_eq!(Some(&address), manufacturer.address());
            assert_eq!(Some(&contact_info), manufacturer.contact_info());
            assert_eq!(Some(&socials), manufacturer.socials());
            assert_eq!(Metadata::created_at(now), manufacturer.metadata);
        }

        #[test]
        fn it_should_compare_two_manufacturers() {
            let acme = acme();
            let roco = roco();

            assert_eq!(acme, acme);
            assert_ne!(acme, roco);
        }

        #[test]
        fn it_should_display_manufacturers() {
            assert_eq!("ACME", acme().to_string());
        }
    }
}
