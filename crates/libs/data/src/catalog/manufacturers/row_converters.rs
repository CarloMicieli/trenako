use crate::catalog::manufacturers::manufacturer_row::ManufacturerRow;
use catalog::manufacturers::manufacturer::Manufacturer;
use common::address::{Address, AddressBuilder};
use common::contacts::ContactInformation;
use common::localized_text::{Language, LocalizedText};
use common::metadata::Metadata;
use common::queries::converters::{ConversionErrors, Converter, OptionConverter, ToOutputConverter};
use common::socials::Socials;

impl ToOutputConverter<Manufacturer> for ManufacturerRow {
    fn to_output(self) -> Result<Manufacturer, ConversionErrors> {
        let row = self;

        let description = LocalizedText::try_convert(&row)?;
        let address = Address::try_convert(&row)?;
        let socials = Socials::try_convert(&row)?;
        let contact_info = ContactInformation::try_convert(&row)?;
        let metadata = Metadata::try_convert(&row)?;

        Ok(Manufacturer {
            manufacturer_id: row.manufacturer_id,
            name: row.name,
            registered_company_name: row.registered_company_name,
            organization_entity_type: row.organization_entity_type,
            group_name: row.group_name,
            description,
            address,
            contact_info,
            kind: row.kind,
            status: row.status,
            socials,
            metadata,
        })
    }
}

impl Converter<ManufacturerRow> for LocalizedText {
    fn try_convert(value: &ManufacturerRow) -> Result<Self, ConversionErrors> {
        let mut localized_text = LocalizedText::default();

        localized_text.insert(Language::English, value.description_en.as_ref());
        localized_text.insert(Language::French, value.description_fr.as_ref());
        localized_text.insert(Language::German, value.description_de.as_ref());
        localized_text.insert(Language::Italian, value.description_it.as_ref());

        Ok(localized_text)
    }
}

impl OptionConverter<ManufacturerRow> for Socials {
    fn try_convert(value: &ManufacturerRow) -> Result<Option<Self>, ConversionErrors> {
        match (
            &value.socials_facebook,
            &value.socials_instagram,
            &value.socials_linkedin,
            &value.socials_youtube,
            &value.socials_twitter,
        ) {
            (None, None, None, None, None) => Ok(None),
            (facebook, instagram, linkedin, youtube, twitter) => Ok(Some(Socials {
                facebook: facebook.clone(),
                instagram: instagram.clone(),
                linkedin: linkedin.clone(),
                twitter: twitter.clone(),
                youtube: youtube.clone(),
            })),
        }
    }
}

impl Converter<ManufacturerRow> for Metadata {
    fn try_convert(value: &ManufacturerRow) -> Result<Self, ConversionErrors> {
        Ok(Metadata::new(
            value.version as u8,
            value.created_at,
            value.last_modified_at,
        ))
    }
}

impl OptionConverter<ManufacturerRow> for ContactInformation {
    fn try_convert(row: &ManufacturerRow) -> Result<Option<Self>, ConversionErrors> {
        match (&row.contact_email, &row.contact_phone, &row.contact_website_url) {
            (None, None, None) => Ok(None),
            (email, phone, website_url) => Ok(Some(ContactInformation {
                email: email.clone(),
                phone: phone.clone(),
                website_url: website_url.clone(),
            })),
        }
    }
}

impl OptionConverter<ManufacturerRow> for Address {
    fn try_convert(row: &ManufacturerRow) -> Result<Option<Self>, ConversionErrors> {
        match (
            &row.address_street_address,
            &row.address_city,
            &row.address_postal_code,
            &row.address_country,
        ) {
            (Some(street_address), Some(city), Some(postal_code), Some(country)) => {
                let mut builder = AddressBuilder::default()
                    .street_address(street_address)
                    .country_code(country)
                    .postal_code(postal_code)
                    .city(city);

                if let Some(extended_address) = &row.address_extended_address {
                    builder = builder.extended_address(extended_address);
                }

                if let Some(region) = &row.address_region {
                    builder = builder.region(region);
                }

                Ok(Some(builder.build().unwrap()))
            }
            (None, None, None, None) => Ok(None),
            _ => Err(ConversionErrors::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::catalog::manufacturers::manufacturer_row::test::new_manufacturer_row;
    use chrono::Utc;

    fn default_row() -> ManufacturerRow {
        new_manufacturer_row("ACME", Utc::now())
    }

    mod manufacturer_row_converter {
        use super::*;
        use catalog::manufacturers::manufacturer_id::ManufacturerId;
        use catalog::manufacturers::manufacturer_kind::ManufacturerKind;
        use catalog::manufacturers::manufacturer_status::ManufacturerStatus;
        use common::organizations::OrganizationEntityType;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_manufacturer_rows() {
            let row = ManufacturerRow {
                manufacturer_id: ManufacturerId::new("ACME"),
                name: String::from("ACME"),
                registered_company_name: Some(String::from("Company Ltd")),
                organization_entity_type: Some(OrganizationEntityType::LimitedCompany),
                kind: ManufacturerKind::Industrial,
                status: Some(ManufacturerStatus::Active),
                ..default_row()
            };

            let result = row.to_output();
            assert!(result.is_ok());

            let manufacturer = result.unwrap();
            assert_eq!(manufacturer.manufacturer_id, ManufacturerId::new("ACME"));
            assert_eq!(manufacturer.name, String::from("ACME"));
            assert_eq!(manufacturer.registered_company_name, Some(String::from("Company Ltd")));
            assert_eq!(
                manufacturer.organization_entity_type,
                Some(OrganizationEntityType::LimitedCompany)
            );
            assert_eq!(manufacturer.kind, ManufacturerKind::Industrial);
            assert_eq!(manufacturer.status, Some(ManufacturerStatus::Active));
        }
    }

    mod address_converter {
        use super::*;
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_address() {
            let row = ManufacturerRow {
                address_city: Some(String::from("City")),
                address_postal_code: Some(String::from("1234")),
                address_street_address: Some(String::from("street address")),
                address_extended_address: Some(String::from("extended address")),
                address_country: Some(String::from("IT")),
                ..default_row()
            };

            let result = Address::try_convert(&row);
            assert!(result.is_ok());

            let address = result.unwrap().unwrap();
            assert_eq!(address.street_address, "street address");
            assert_eq!(address.extended_address, Some(String::from("extended address")));
            assert_eq!(address.country, CountryCode::ITA);
            assert_eq!(address.postal_code, "1234");
            assert_eq!(address.city, "City");
        }
    }

    mod socials_converter {
        use super::*;
        use common::socials::Handler;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_return_a_none_when_there_are_no_socials_handler_in_the_row() {
            let row = ManufacturerRow { ..default_row() };

            let result = Socials::try_convert(&row);

            assert!(result.is_ok());

            let socials = result.unwrap();
            assert!(socials.is_none());
        }

        #[test]
        fn it_should_convert_socials() {
            let row = ManufacturerRow {
                socials_facebook: Some(Handler::new("facebook")),
                socials_instagram: Some(Handler::new("instagram")),
                socials_linkedin: Some(Handler::new("linkedin")),
                socials_youtube: Some(Handler::new("youtube")),
                socials_twitter: Some(Handler::new("twitter")),
                ..default_row()
            };

            let result = Socials::try_convert(&row);

            assert!(result.is_ok());

            let socials = result.unwrap();
            assert!(socials.is_some());

            let socials = socials.unwrap();
            assert_eq!(Some(Handler::new("facebook")), socials.facebook);
            assert_eq!(Some(Handler::new("instagram")), socials.instagram);
            assert_eq!(Some(Handler::new("linkedin")), socials.linkedin);
            assert_eq!(Some(Handler::new("youtube")), socials.youtube);
            assert_eq!(Some(Handler::new("twitter")), socials.twitter);
        }
    }

    mod description_converter {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_description() {
            let row = ManufacturerRow {
                description_de: Some(String::from("de")),
                description_en: Some(String::from("en")),
                description_fr: Some(String::from("fr")),
                description_it: Some(String::from("it")),
                ..default_row()
            };

            let result = LocalizedText::try_convert(&row);

            assert!(result.is_ok());

            let description = result.unwrap();
            assert_eq!(Some(&String::from("en")), description.english());
            assert_eq!(Some(&String::from("it")), description.italian());
            assert_eq!(Some(&String::from("fr")), description.french());
            assert_eq!(Some(&String::from("de")), description.german());
        }
    }

    mod metadata_converter {
        use super::*;
        use chrono::Utc;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_convert_metadata() {
            let now = Utc::now();
            let row = ManufacturerRow {
                created_at: now,
                version: 42,
                ..default_row()
            };

            let result = Metadata::try_convert(&row);

            assert!(result.is_ok());

            let metadata = result.unwrap();
            assert_eq!(&now, metadata.created());
            assert_eq!(None, metadata.last_modified());
            assert_eq!(42, metadata.version());
        }
    }

    mod contact_information_converter {
        use super::*;
        use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};

        #[test]
        fn it_should_return_a_none_when_the_contact_information_are_missing() {
            let row = ManufacturerRow { ..default_row() };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            assert!(result.is_none());
        }

        #[test]
        fn it_should_convert_contact_information_email() {
            let contact_email = Some(MailAddress::new("mail@mail.com"));
            let row = ManufacturerRow {
                contact_email: contact_email.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(contact_email, contact_information.email);
            assert_eq!(None, contact_information.phone);
            assert_eq!(None, contact_information.website_url);
        }

        #[test]
        fn it_should_convert_contact_information_phone() {
            let contact_phone = Some(PhoneNumber::new("+39029566789"));
            let row = ManufacturerRow {
                contact_phone: contact_phone.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(None, contact_information.email);
            assert_eq!(contact_phone, contact_information.phone);
            assert_eq!(None, contact_information.website_url);
        }

        #[test]
        fn it_should_convert_contact_information_website_url() {
            let contact_website_url = Some(WebsiteUrl::new("http://localhost"));
            let row = ManufacturerRow {
                contact_website_url: contact_website_url.clone(),
                ..default_row()
            };

            let result = ContactInformation::try_convert(&row).expect("the contact information are invalid");
            let contact_information = result.expect("the contact information are missing");

            assert_eq!(None, contact_information.email);
            assert_eq!(None, contact_information.phone);
            assert_eq!(contact_website_url, contact_information.website_url);
        }
    }
}
