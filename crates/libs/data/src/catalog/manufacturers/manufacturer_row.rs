use catalog::manufacturers::manufacturer_id::ManufacturerId;
use catalog::manufacturers::manufacturer_kind::ManufacturerKind;
use catalog::manufacturers::manufacturer_status::ManufacturerStatus;
use chrono::{DateTime, Utc};
use common::contacts::{MailAddress, PhoneNumber, WebsiteUrl};
use common::organizations::OrganizationEntityType;
use common::socials::Handler;

#[derive(Debug)]
pub struct ManufacturerRow {
    pub manufacturer_id: ManufacturerId,
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description_de: Option<String>,
    pub description_en: Option<String>,
    pub description_fr: Option<String>,
    pub description_it: Option<String>,
    pub kind: ManufacturerKind,
    pub status: Option<ManufacturerStatus>,
    pub contact_email: Option<MailAddress>,
    pub contact_website_url: Option<WebsiteUrl>,
    pub contact_phone: Option<PhoneNumber>,
    pub address_street_address: Option<String>,
    pub address_extended_address: Option<String>,
    pub address_city: Option<String>,
    pub address_region: Option<String>,
    pub address_postal_code: Option<String>,
    pub address_country: Option<String>,
    pub socials_facebook: Option<Handler>,
    pub socials_instagram: Option<Handler>,
    pub socials_linkedin: Option<Handler>,
    pub socials_twitter: Option<Handler>,
    pub socials_youtube: Option<Handler>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[allow(dead_code)]
    pub fn new_manufacturer_row(name: &str, created_at: DateTime<Utc>) -> ManufacturerRow {
        ManufacturerRow {
            manufacturer_id: ManufacturerId::new(name),
            name: String::from(name),
            registered_company_name: None,
            organization_entity_type: None,
            group_name: None,
            description_de: None,
            description_en: None,
            description_fr: None,
            description_it: None,
            kind: Default::default(),
            status: None,
            contact_email: None,
            contact_website_url: None,
            contact_phone: None,
            address_street_address: None,
            address_extended_address: None,
            address_city: None,
            address_region: None,
            address_postal_code: None,
            address_country: None,
            socials_facebook: None,
            socials_instagram: None,
            socials_linkedin: None,
            socials_twitter: None,
            socials_youtube: None,
            version: 0,
            created_at,
            last_modified_at: None,
        }
    }
}
