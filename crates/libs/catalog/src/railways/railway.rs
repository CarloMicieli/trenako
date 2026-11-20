//! the railway view models

use crate::railways::period_of_activity::PeriodOfActivity;
use crate::railways::railway_gauge::RailwayGauge;
use crate::railways::railway_id::RailwayId;
use crate::railways::railway_length::RailwayLength;
use common::contacts::ContactInformation;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::organizations::OrganizationEntityType;
use common::socials::Socials;
use isocountry::CountryCode;
use std::fmt::Formatter;
use std::{cmp, fmt};

/// A railway company is a company within the rail industry.
///
/// It can be a manufacturing firm or an operator. Some railway companies operate both the trains
/// and the track, while, particularly in the European Union, operation of the track is undertaken
/// by infrastructure operators and trains are run by different companies.
///
/// Railway companies can be private or public.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Railway {
    /// the unique railway identifier (an URL encoded string)
    pub railway_id: RailwayId,
    /// the railway name
    pub name: String,
    /// the railway abbreviated name
    pub abbreviation: Option<String>,
    /// the registered company name
    pub registered_company_name: Option<String>,
    /// the organization entity type
    pub organization_entity_type: Option<OrganizationEntityType>,
    /// the railway description
    pub description: LocalizedText,
    /// the registration country
    pub country: CountryCode,
    /// the period of activity
    pub period_of_activity: Option<PeriodOfActivity>,
    /// the track gauge
    pub gauge: Option<RailwayGauge>,
    /// the railway headquarter
    pub headquarters: Vec<String>,
    /// the railway total length
    pub total_length: Option<RailwayLength>,
    /// the contacts information
    pub contact_info: Option<ContactInformation>,
    /// the social profiles
    pub socials: Option<Socials>,
    /// the metadata
    pub metadata: Metadata,
}

impl Railway {
    /// Create new railway
    pub fn new(
        railway_id: RailwayId,
        name: &str,
        abbreviation: Option<&str>,
        registered_company_name: &str,
        organization_entity_type: Option<OrganizationEntityType>,
        description: Option<&str>,
        period_of_activity: Option<PeriodOfActivity>,
        total_length: Option<RailwayLength>,
        gauge: Option<RailwayGauge>,
        country: CountryCode,
        headquarters: Vec<&str>,
        contact_info: Option<ContactInformation>,
        socials: Option<Socials>,
        metadata: Metadata,
    ) -> Self {
        let headquarters: Vec<String> = headquarters.into_iter().map(str::to_string).collect();

        Railway {
            railway_id,
            name: String::from(name),
            abbreviation: abbreviation.map(str::to_string),
            registered_company_name: Some(String::from(registered_company_name)),
            organization_entity_type,
            description: description.map(LocalizedText::with_italian).unwrap_or_default(),
            country,
            period_of_activity,
            gauge,
            headquarters,
            total_length,
            contact_info,
            socials,
            metadata,
        }
    }

    /// The unique identifier for this railway company
    pub fn railway_id(&self) -> &RailwayId {
        &self.railway_id
    }

    /// The name for this railway company
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The abbreviated name for this railway company
    pub fn abbreviation(&self) -> Option<&str> {
        self.abbreviation.as_deref()
    }

    /// The registered company name (the more formal denomination)
    /// for this Railway company
    pub fn registered_company_name(&self) -> Option<&str> {
        self.registered_company_name.as_deref()
    }

    /// The organization entity type for this railway company
    pub fn organization_entity_type(&self) -> Option<OrganizationEntityType> {
        self.organization_entity_type
    }

    /// The description for this railway company
    pub fn description(&self) -> Option<&String> {
        self.description.italian()
    }

    /// The period of activity (active/inactive) for this railway company
    pub fn period_of_activity(&self) -> Option<&PeriodOfActivity> {
        self.period_of_activity.as_ref()
    }

    /// The total railway network length controlled by this railway company
    pub fn total_length(&self) -> Option<&RailwayLength> {
        self.total_length.as_ref()
    }

    /// The track gauge for this railway
    pub fn gauge(&self) -> Option<&RailwayGauge> {
        self.gauge.as_ref()
    }

    /// The registration country
    pub fn country(&self) -> CountryCode {
        self.country
    }

    /// The railway headquarter
    pub fn headquarters(&self) -> &Vec<String> {
        &self.headquarters
    }

    /// The contact railway info
    pub fn contact_info(&self) -> Option<&ContactInformation> {
        self.contact_info.as_ref()
    }

    /// The social profiles for this railway
    pub fn socials(&self) -> Option<&Socials> {
        self.socials.as_ref()
    }

    /// The metadata for this railway company
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Railway {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.abbreviation {
            Some(abbr) => write!(f, "{} ({})", abbr, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl cmp::PartialEq for Railway {
    fn eq(&self, other: &Self) -> bool {
        self.railway_id == other.railway_id
    }
}

impl cmp::Eq for Railway {}

#[cfg(test)]
mod test {
    use super::*;

    mod railways {
        use super::*;
        use crate::railways::test_data::{die_bahn, fs};
        use chrono::Utc;
        use common::socials::SocialsBuilder;
        use pretty_assertions::{assert_eq, assert_ne};
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let socials = SocialsBuilder::default()
                .instagram("fsitaliane")
                .linkedin("ferrovie-dello-stato-s-p-a-")
                .twitter("FSitaliane")
                .youtube("fsitaliane")
                .build()
                .unwrap();
            let length = RailwayLength::of_kilometers(dec!(24564.0));
            let gauge = RailwayGauge::standard();
            let contact_info = ContactInformation::builder()
                .website_url("https://www.fsitaliane.it")
                .build()
                .unwrap();

            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                Some("FS"),
                "Ferrovie dello stato italiane",
                Some(OrganizationEntityType::StateOwnedEnterprise),
                Some("Description text"),
                None,
                Some(length),
                Some(gauge.clone()),
                CountryCode::ITA,
                vec!["Rome"],
                Some(contact_info.clone()),
                Some(socials.clone()),
                metadata.clone(),
            );

            assert_eq!(&RailwayId::new("FS"), railway.railway_id());
            assert_eq!("FS", railway.name());
            assert_eq!(Some("FS"), railway.abbreviation());
            assert_eq!(Some("Ferrovie dello stato italiane"), railway.registered_company_name());
            assert_eq!(
                Some(OrganizationEntityType::StateOwnedEnterprise),
                railway.organization_entity_type()
            );
            assert_eq!(&vec!["Rome"], railway.headquarters());
            assert_eq!(Some(&length), railway.total_length());
            assert_eq!(Some(&String::from("Description text")), railway.description());
            assert_eq!(Some(&gauge), railway.gauge());
            assert_eq!(Some(&contact_info), railway.contact_info());
            assert_eq!(Some(&socials), railway.socials());
            assert_eq!(&metadata, railway.metadata());
        }

        #[test]
        fn it_should_display_railways() {
            let metadata = Metadata::created_at(Utc::now());
            let railway = Railway::new(
                RailwayId::new("FS"),
                "FS",
                Some("FS"),
                "Ferrovie dello stato italiane",
                Some(OrganizationEntityType::StateOwnedEnterprise),
                None,
                None,
                None,
                None,
                CountryCode::ITA,
                vec!["Rome"],
                None,
                None,
                metadata,
            );

            assert_eq!("FS (FS)", railway.to_string());
        }

        #[test]
        fn it_should_compare_railways() {
            let db = die_bahn();
            let fs = fs();

            assert_eq!(fs, fs);
            assert_ne!(fs, db);
        }
    }
}
