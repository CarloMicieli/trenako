use crate::catalog::manufacturers::manufacturer_row::ManufacturerRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::manufacturers::commands::new_manufacturer::NewManufacturerCommand;
use catalog::manufacturers::commands::repositories::NewManufacturerRepository;
use catalog::manufacturers::manufacturer::Manufacturer;
use catalog::manufacturers::manufacturer_id::ManufacturerId;
use catalog::manufacturers::manufacturer_kind::ManufacturerKind;
use catalog::manufacturers::manufacturer_status::ManufacturerStatus;
use catalog::manufacturers::queries::find_all_manufacturers::FindAllManufacturersRepository;
use catalog::manufacturers::queries::find_manufacturer_by_id::FindManufacturerByIdRepository;
use common::contacts::WebsiteUrl;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::socials::Handler;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct ManufacturersRepository;

#[async_trait]
impl<'db> NewManufacturerRepository<'db, PgUnitOfWork<'db>> for ManufacturersRepository {
    async fn exists(
        &self,
        manufacturer_id: &ManufacturerId,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT manufacturer_id FROM manufacturers WHERE manufacturer_id = $1 LIMIT 1",
            manufacturer_id
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for a manufacturer existence.")?;

        Ok(result.is_some())
    }

    async fn insert(
        &self,
        new_manufacturer: &NewManufacturerCommand,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<(), anyhow::Error> {
        let manufacturer_id = &new_manufacturer.manufacturer_id;
        let request = &new_manufacturer.payload;
        let metadata = &new_manufacturer.metadata;

        sqlx::query!(
                r#"INSERT INTO manufacturers (
                    manufacturer_id,
                    name,
                    registered_company_name,
                    organization_entity_type,
                    group_name,
                    description_de,
                    description_en,
                    description_fr,
                    description_it,
                    kind,
                    status,
                    contact_email, contact_website_url, contact_phone,
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube,
                    created_at,
                    version
                )
                VALUES (
                    $1, $2, $3, $4, $5, $6,
                    $7, $8, $9, $10, $11, $12, 
                    $13, $14, $15, $16, $17, $18,
                    $19, $20, $21, $22, $23, $24, 
                    $25, $26, $27
                )"#,
                manufacturer_id as &ManufacturerId,
                request.name,
                request.registered_company_name,
                request.organization_entity_type as Option<OrganizationEntityType>,
                request.group_name,
                request.description.german(),
                request.description.english(),
                request.description.french(),
                request.description.italian(),
                request.kind as ManufacturerKind,
                request.status as Option<ManufacturerStatus>,
                request.contact_email.as_ref() as Option<&MailAddress>,
                request.contact_website_url.as_ref().map(|x| x.to_string()),
                request.contact_phone.as_ref() as Option<&PhoneNumber>,
                request.address_street_address,
                request.address_extended_address,
                request.address_city,
                request.address_region,
                request.address_postal_code,
                request.address_country,
                request.facebook_handler.as_ref() as Option<&Handler>,
                request.instagram_handler.as_ref() as Option<&Handler>,
                request.linkedin_handler.as_ref() as Option<&Handler>,
                request.twitter_handler.as_ref() as Option<&Handler>,
                request.youtube_handler.as_ref() as Option<&Handler>,
                metadata.created(),
                metadata.version() as i32
            )
            .execute(&mut *unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to store a manufacturer.")?;

        Ok(())
    }
}

#[async_trait]
impl<'db> FindAllManufacturersRepository<'db, PgUnitOfWork<'db>> for ManufacturersRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork) -> Result<Vec<Manufacturer>, DatabaseError> {
        let results: Vec<ManufacturerRow> = sqlx::query_as!(ManufacturerRow,
                r#"SELECT
                    manufacturer_id as "manufacturer_id!: ManufacturerId", 
                    name, 
                    registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", 
                    group_name,
                    description_de, 
                    description_en,
                    description_fr, 
                    description_it,
                    kind as "kind: ManufacturerKind", 
                    status as "status?: ManufacturerStatus",
                    contact_email as "contact_email?: MailAddress", 
                    contact_website_url as "contact_website_url?: WebsiteUrl", 
                    contact_phone as "contact_phone?: PhoneNumber",
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook as "socials_facebook?: Handler", 
                    socials_instagram as "socials_instagram?: Handler",     
                    socials_linkedin as "socials_linkedin?: Handler",    
                    socials_twitter as "socials_twitter?: Handler",    
                    socials_youtube as "socials_youtube?: Handler",
                    created_at,
                    last_modified_at,
                    version
                FROM manufacturers
                ORDER BY name"#)
            .fetch_all(&mut *unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch the manufacturers.")?;

        let mut output: Vec<Manufacturer> = Vec::with_capacity(results.len());
        for row in results.into_iter() {
            let manufacturer = row.to_output().map_err(DatabaseError::ConversionError)?;
            output.push(manufacturer);
        }

        Ok(output)
    }
}

#[async_trait]
impl<'db> FindManufacturerByIdRepository<'db, PgUnitOfWork<'db>> for ManufacturersRepository {
    async fn find_by_id(
        &self,
        manufacturer_id: &ManufacturerId,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<Option<Manufacturer>, DatabaseError> {
        let result: Option<ManufacturerRow> = sqlx::query_as!(ManufacturerRow,
                r#"SELECT
                    manufacturer_id as "manufacturer_id!: ManufacturerId", 
                    name, registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", 
                    group_name, 
                    description_de, 
                    description_en,
                    description_fr, 
                    description_it,
                    kind as "kind: ManufacturerKind", 
                    status as "status?: ManufacturerStatus",
                    contact_email as "contact_email?: MailAddress", 
                    contact_website_url as "contact_website_url?: WebsiteUrl", 
                    contact_phone as "contact_phone?: PhoneNumber",
                    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
                    socials_facebook as "socials_facebook?: Handler", 
                    socials_instagram as "socials_instagram?: Handler",     
                    socials_linkedin as "socials_linkedin?: Handler",    
                    socials_twitter as "socials_twitter?: Handler",    
                    socials_youtube as "socials_youtube?: Handler",
                    created_at,
                    last_modified_at,
                    version
                FROM manufacturers WHERE manufacturer_id = $1"#, 
                manufacturer_id)
            .fetch_optional(&mut *unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch a manufacturer.")?;

        result.to_output().map_err(DatabaseError::ConversionError)
    }
}
