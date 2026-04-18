//! the new manufacturer creation command

use crate::manufacturers::commands::repositories::NewManufacturerRepository;
use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::manufacturers::manufacturer_kind::ManufacturerKind;
use crate::manufacturers::manufacturer_request::ManufacturerRequest;
use crate::manufacturers::manufacturer_response::ManufacturerCreated;
use crate::manufacturers::manufacturer_status::ManufacturerStatus;
use chrono::Utc;
use common::address::Address;
use common::contacts::{ContactInformation, MailAddress, PhoneNumber, WebsiteUrl};
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::queries::errors::DatabaseError;
use common::socials::{Handler, Socials};
use common::unit_of_work::{Database, UnitOfWork};
use std::result;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

pub type Result<R> = result::Result<R, ManufacturerCreationError>;

pub async fn create_new_manufacturer<'db, U, Repo, DB>(
    request: ManufacturerRequest,
    repo: Repo,
    db: DB,
) -> Result<ManufacturerCreated>
where
    U: UnitOfWork<'db>,
    Repo: NewManufacturerRepository<'db, U>,
    DB: Database<'db, U>,
{
    let manufacturer_id = ManufacturerId::new(&request.name);

    let mut unit_of_work = db.begin().await?;

    if repo.exists(&manufacturer_id, &mut unit_of_work).await? {
        return Err(ManufacturerCreationError::ManufacturerAlreadyExists(manufacturer_id));
    }

    let command = NewManufacturerCommand::try_from(request)?;
    repo.insert(&command, &mut unit_of_work).await?;

    unit_of_work.commit().await?;

    Ok(ManufacturerCreated {
        manufacturer_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum ManufacturerCreationError {
    #[error("The manufacturer request is not valid")]
    InvalidRequest(ValidationErrors),

    #[error("The manufacturer already exists (id: {0})")]
    ManufacturerAlreadyExists(ManufacturerId),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

/// It represents the command to create a new model railway manufacturer
#[derive(Debug, Clone)]
pub struct NewManufacturerCommand {
    pub manufacturer_id: ManufacturerId,
    pub payload: ManufacturerCommandPayload,
    pub metadata: Metadata,
}

impl TryFrom<ManufacturerRequest> for NewManufacturerCommand {
    type Error = ManufacturerCreationError;

    fn try_from(value: ManufacturerRequest) -> result::Result<Self, Self::Error> {
        validate_request(&value)?;
        let manufacturer_id = ManufacturerId::new(&value.name);
        let payload = ManufacturerCommandPayload::try_from(value)?;
        let metadata = Metadata::created_at(Utc::now());
        Ok(NewManufacturerCommand {
            manufacturer_id,
            payload,
            metadata,
        })
    }
}

fn validate_request(request: &ManufacturerRequest) -> result::Result<(), ManufacturerCreationError> {
    request.validate().map_err(ManufacturerCreationError::InvalidRequest)
}

#[derive(Debug, Clone, Default)]
pub struct ManufacturerCommandPayload {
    pub name: String,
    pub registered_company_name: Option<String>,
    pub organization_entity_type: Option<OrganizationEntityType>,
    pub group_name: Option<String>,
    pub description: LocalizedText,
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
    pub facebook_handler: Option<Handler>,
    pub instagram_handler: Option<Handler>,
    pub linkedin_handler: Option<Handler>,
    pub twitter_handler: Option<Handler>,
    pub youtube_handler: Option<Handler>,
}

impl TryFrom<ManufacturerRequest> for ManufacturerCommandPayload {
    type Error = ManufacturerCreationError;

    fn try_from(request: ManufacturerRequest) -> result::Result<Self, Self::Error> {
        let contacts = request.contact_info.unwrap_or_default();
        let ContactInformation {
            email,
            website_url,
            phone,
        } = contacts;

        let socials = request.socials.unwrap_or_default();
        let Socials {
            facebook,
            instagram,
            linkedin,
            twitter,
            youtube,
        } = socials;

        let (
            address_street_address,
            address_extended_address,
            address_city,
            address_region,
            address_postal_code,
            address_country,
        ) = if let Some(Address {
            street_address,
            extended_address,
            city,
            region,
            postal_code,
            country,
        }) = request.address
        {
            (
                Some(street_address),
                extended_address,
                Some(city),
                region,
                Some(postal_code),
                Some(country.alpha2().to_string()),
            )
        } else {
            (None, None, None, None, None, None)
        };

        let value = ManufacturerCommandPayload {
            name: request.name,
            registered_company_name: request.registered_company_name,
            organization_entity_type: request.organization_entity_type,
            group_name: request.group_name,
            description: request.description,
            kind: request.kind,
            status: request.status,
            contact_email: email,
            contact_website_url: website_url,
            contact_phone: phone,
            address_street_address,
            address_extended_address,
            address_city,
            address_region,
            address_postal_code,
            address_country,
            facebook_handler: facebook,
            instagram_handler: instagram,
            linkedin_handler: linkedin,
            twitter_handler: twitter,
            youtube_handler: youtube,
        };

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod new_manufacturer_command {
        use super::*;
        use crate::manufacturers::commands::repositories::in_memory::InMemoryManufacturerRepository;
        use chrono::TimeZone;
        use common::localized_text::LocalizedText;
        use common::unit_of_work::noop::NoOpDatabase;
        use isocountry::CountryCode;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_create_a_new_manufacturer() {
            let repo = InMemoryManufacturerRepository::empty();

            let request = new_manufacturer("ACME");
            let db = NoOpDatabase;
            let result = create_new_manufacturer(request, repo, db).await;

            let created = result.expect("result is an error");

            assert_eq!(ManufacturerId::new("ACME"), created.manufacturer_id);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_manufacturer_already_exists() {
            let new_manufacturer_cmd = new_manufacturer_cmd_with_name("ACME");
            let repo = InMemoryManufacturerRepository::with(new_manufacturer_cmd);

            let request = new_manufacturer("ACME");
            let db = NoOpDatabase;
            let result = create_new_manufacturer(request, repo, db).await;

            match result {
                Err(ManufacturerCreationError::ManufacturerAlreadyExists(id)) => {
                    assert_eq!(ManufacturerId::new("ACME"), id)
                }
                _ => panic!("ManufacturerAlreadyExists is expected (found: {:?})", result),
            }
        }

        fn new_manufacturer(name: &str) -> ManufacturerRequest {
            let address = Address::builder()
                .street_address("Rue Morgue 22")
                .city("London")
                .postal_code("1H2 4BB")
                .country(CountryCode::GBR)
                .build()
                .unwrap();

            let contact_info = ContactInformation::builder()
                .email("mail@mail.com")
                .phone("+14152370800")
                .website_url("https://www.site.com")
                .build()
                .unwrap();

            let socials = Socials::builder()
                .instagram("instagram_handler")
                .linkedin("linkedin_handler")
                .facebook("facebook_handler")
                .twitter("twitter_handler")
                .youtube("youtube_handler")
                .build()
                .unwrap();

            ManufacturerRequest {
                name: String::from(name),
                registered_company_name: Some(String::from("A cool manufacturer ltd.")),
                organization_entity_type: Some(OrganizationEntityType::LimitedCompany),
                group_name: Some(String::from("Group Corp.")),
                description: LocalizedText::with_italian("La descrizione va qui"),
                address: Some(address),
                contact_info: Some(contact_info),
                kind: ManufacturerKind::Industrial,
                status: Some(ManufacturerStatus::Active),
                socials: Some(socials),
            }
        }

        fn new_manufacturer_cmd_with_name(name: &str) -> NewManufacturerCommand {
            let manufacturer_id = ManufacturerId::new(name);
            let payload = ManufacturerCommandPayload {
                name: String::from(name),
                ..ManufacturerCommandPayload::default()
            };
            let metadata = Metadata::created_at(Utc.with_ymd_and_hms(1988, 11, 25, 0, 0, 0).unwrap());

            NewManufacturerCommand {
                manufacturer_id,
                payload,
                metadata,
            }
        }
    }
}
