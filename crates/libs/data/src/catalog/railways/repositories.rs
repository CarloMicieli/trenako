use crate::catalog::railways::railway_row::RailwayRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::common::TrackGauge;
use catalog::railways::commands::new_railways::NewRailwayCommand;
use catalog::railways::commands::repositories::NewRailwayRepository;
use catalog::railways::period_of_activity::RailwayStatus;
use catalog::railways::queries::find_all_railways::FindAllRailwaysRepository;
use catalog::railways::queries::find_railway_by_id::FindRailwayByIdRepository;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use common::contacts::WebsiteUrl;
use common::contacts::{MailAddress, PhoneNumber};
use common::organizations::OrganizationEntityType;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::socials::Handler;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct RailwaysRepository;

#[async_trait]
impl<'db> NewRailwayRepository<'db, PgUnitOfWork<'db>> for RailwaysRepository {
    async fn exists(&self, railway_id: &RailwayId, unit_of_work: &mut PgUnitOfWork) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT railway_id FROM railways WHERE railway_id = $1 LIMIT 1",
            railway_id
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for railway existence.")?;

        Ok(result.is_some())
    }

    async fn insert(
        &self,
        new_railway: &NewRailwayCommand,
        unit_of_work: &mut PgUnitOfWork,
    ) -> Result<(), anyhow::Error> {
        let railway_id = &new_railway.railway_id;
        let request = &new_railway.payload;
        let metadata = &new_railway.metadata;

        sqlx::query!(
            r#"INSERT INTO railways (
                railway_id,
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type,
                description_de, 
                description_en,
                description_fr, 
                description_it,
                country,
                operating_since,
                operating_until,
                status,
                gauge_meters,
                track_gauge,
                headquarters,
                total_length_mi,
                total_length_km,
                contact_email,
                contact_website_url,
                contact_phone,
                socials_facebook,
                socials_instagram,
                socials_linkedin,
                socials_twitter,
                socials_youtube,
                created_at,
                version
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, 
                $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23, $24, 
                $25, $26, $27, $28
            )"#,
            railway_id as &RailwayId,
            request.name,
            request.abbreviation,
            request.registered_company_name,
            request.organization_entity_type.as_ref() as Option<&OrganizationEntityType>,
            request.description.german(),
            request.description.english(),
            request.description.french(),
            request.description.italian(),
            request.country,
            request.operating_since,
            request.operating_until,
            request.status.as_ref() as Option<&RailwayStatus>,
            request.gauge_meters,
            request.track_gauge.as_ref() as Option<&TrackGauge>,
            &request.headquarters,
            request.total_length_mi,
            request.total_length_km,
            request.contact_email.as_ref() as Option<&MailAddress>,
            request.contact_website_url.as_ref().map(|x| x.to_string()),
            request.contact_phone.as_ref() as Option<&PhoneNumber>,
            request.socials_facebook.as_ref() as Option<&Handler>,
            request.socials_instagram.as_ref() as Option<&Handler>,
            request.socials_linkedin.as_ref() as Option<&Handler>,
            request.socials_twitter.as_ref() as Option<&Handler>,
            request.socials_youtube.as_ref() as Option<&Handler>,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a railway.")?;

        Ok(())
    }
}

#[async_trait]
impl<'db> FindAllRailwaysRepository<'db, PgUnitOfWork<'db>> for RailwaysRepository {
    async fn find_all(&self, unit_of_work: &mut PgUnitOfWork<'db>) -> Result<Vec<Railway>, DatabaseError> {
        let results = sqlx::query_as!(
            RailwayRow,
            r#"SELECT
                railway_id as "railway_id: RailwayId",
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type as "organization_entity_type?: OrganizationEntityType",
                description_de, 
                description_en,
                description_fr, 
                description_it,
                country,
                operating_since,
                operating_until,
                status as "status?: RailwayStatus",
                gauge_meters,
                track_gauge as "track_gauge?: TrackGauge",
                headquarters as "headquarters!: Vec<String>",
                total_length_mi,
                total_length_km,
                contact_email as "contact_email?: MailAddress",
                contact_website_url as "contact_website_url?: WebsiteUrl",
                contact_phone as "contact_phone?: PhoneNumber",
                socials_facebook as "socials_facebook?: Handler",
                socials_instagram as "socials_instagram?: Handler",
                socials_linkedin as "socials_linkedin?: Handler",
                socials_twitter as "socials_twitter?: Handler",
                socials_youtube as "socials_youtube?: Handler",
                created_at,
                last_modified_at,
                version
            FROM railways
            ORDER BY name"#,
        )
        .fetch_all(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch railways.")?;

        results.to_output().map_err(DatabaseError::ConversionError)
    }
}

#[async_trait]
impl<'db> FindRailwayByIdRepository<'db, PgUnitOfWork<'db>> for RailwaysRepository {
    async fn find_by_id(
        &self,
        railway_id: &RailwayId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Option<Railway>, DatabaseError> {
        let result = sqlx::query_as!(
            RailwayRow,
            r#"SELECT
                railway_id as "railway_id: RailwayId",
                name,
                abbreviation,
                registered_company_name,
                organization_entity_type as "organization_entity_type?: OrganizationEntityType",
                description_de, 
                description_en,
                description_fr, 
                description_it,
                country,
                operating_since,
                operating_until,
                status as "status?: RailwayStatus",
                gauge_meters,
                track_gauge as "track_gauge?: TrackGauge",
                headquarters as "headquarters!: Vec<String>",
                total_length_mi,
                total_length_km,
                contact_email as "contact_email?: MailAddress",
                contact_website_url as "contact_website_url?: WebsiteUrl",
                contact_phone as "contact_phone?: PhoneNumber",
                socials_facebook as "socials_facebook?: Handler",
                socials_instagram as "socials_instagram?: Handler",
                socials_linkedin as "socials_linkedin?: Handler",
                socials_twitter as "socials_twitter?: Handler",
                socials_youtube as "socials_youtube?: Handler",
                created_at,
                last_modified_at,
                version
            FROM railways 
            WHERE railway_id = $1"#,
            railway_id
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a railway.")?;

        result.to_output().map_err(DatabaseError::ConversionError)
    }
}
