use crate::catalog::brands::brand_row::BrandRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::brands::brand::Brand;
use catalog::brands::brand_id::BrandId;
use catalog::brands::brand_kind::BrandKind;
use catalog::brands::brand_status::BrandStatus;
use catalog::brands::queries::find_all_brands::FindAllBrandsRepository;
use catalog::brands::queries::find_brand_by_id::FindBrandByIdRepository;
use common::contacts::WebsiteUrl;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::socials::Handler;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct BrandsRepository;

#[async_trait]
impl<'db> FindAllBrandsRepository<'db, PgUnitOfWork<'db>> for BrandsRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork) -> Result<Vec<Brand>, DatabaseError> {
        let results: Vec<BrandRow> = sqlx::query_as!(BrandRow,
                r#"SELECT
                    brand_id as "brand_id!: BrandId", 
                    name, 
                    registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", 
                    group_name, 
                    description_en, description_it, 
                    kind as "kind: BrandKind", 
                    status as "status?: BrandStatus",
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
                FROM brands
                ORDER BY name"#)
            .fetch_all(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch the brands.")?;

        let mut output: Vec<Brand> = Vec::with_capacity(results.len());
        for row in results.into_iter() {
            let brand = row.to_output().map_err(DatabaseError::ConversionError)?;
            output.push(brand);
        }

        Ok(output)
    }
}

#[async_trait]
impl<'db> FindBrandByIdRepository<'db, PgUnitOfWork<'db>> for BrandsRepository {
    async fn find_by_id(
        &self,
        brand_id: &BrandId,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<Option<Brand>, DatabaseError> {
        let result: Option<BrandRow> = sqlx::query_as!(BrandRow,
                r#"SELECT
                    brand_id as "brand_id!: BrandId", 
                    name, registered_company_name, 
                    organization_entity_type as "organization_entity_type: OrganizationEntityType", 
                    group_name, 
                    description_en, description_it, 
                    kind as "kind: BrandKind", 
                    status as "status?: BrandStatus",
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
                FROM brands WHERE brand_id = $1"#, 
                brand_id)
            .fetch_optional(&mut unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to fetch a brand.")?;

        result.map(row_to_brand).transpose()
    }
}

fn row_to_brand(row: BrandRow) -> Result<Brand, DatabaseError> {
    row.to_output().map_err(DatabaseError::ConversionError)
}
