//! the scale view models

use super::ratio::Ratio;
use super::scale_gauge::Gauge;
use super::scale_id::ScaleId;
use super::standard::Standard;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::{cmp, fmt};

/// Rail transport modelling uses a variety of scales (ratio between the real world and the model)
/// to ensure scale models look correct when placed next to each other.
///
/// Model railway scales are standardized worldwide by many organizations and hobbyist groups.
/// Some of the scales are recognized globally, while others are less widespread and, in many cases,
/// virtually unknown outside their circle of origin. Scales may be expressed as a numeric ratio
/// (e.g. 1/87 or 1:87) or as letters defined in rail transport modelling standards
/// (e.g. HO, OO, N, O, G, TT and Z.) The majority of commercial model railway equipment manufacturers
/// base their offerings on Normen Europäischer Modellbahnen (NEM) or
/// National Model Railroad Association (NMRA) standards in most popular scales.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    /// the unique identifier for a scale
    pub scale_id: ScaleId,
    /// the scale name
    pub name: String,
    /// the ratio between the real world and the model (e.g. 1/87 or 1:87)
    pub ratio: Ratio,
    /// the scale gauge
    pub gauge: Gauge,
    /// the modelling scale description
    pub description: LocalizedText,
    /// the list of standards   
    pub standards: HashSet<Standard>,
    /// the metadata
    pub metadata: Metadata,
}

impl Scale {
    /// Create a new Scale
    pub fn new(
        scale_id: ScaleId,
        name: &str,
        description: Option<&str>,
        ratio: Ratio,
        gauge: Gauge,
        standards: HashSet<Standard>,
        metadata: Metadata,
    ) -> Self {
        Scale {
            scale_id,
            name: String::from(name),
            ratio,
            gauge,
            description: description.map(LocalizedText::with_italian).unwrap_or_default(),
            standards,
            metadata,
        }
    }

    /// The unique identifier for this Scale
    pub fn scale_id(&self) -> &ScaleId {
        &self.scale_id
    }

    /// The scale name, typically letters defined in rail transport modelling standards
    /// (e.g. HO, OO, N, O, G, TT and Z.)
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The (optional) Scale description
    pub fn description(&self) -> Option<&String> {
        self.description.italian()
    }

    /// This scale ratio between the real world and the model
    /// (e.g. 1/87 or 1:87)
    pub fn ratio(&self) -> &Ratio {
        &self.ratio
    }

    /// The scale gauge
    pub fn gauge(&self) -> &Gauge {
        &self.gauge
    }

    /// The standards set for this scale
    pub fn standards(&self) -> &HashSet<Standard> {
        &self.standards
    }

    /// The scale metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.ratio)
    }
}

impl cmp::PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.scale_id == other.scale_id
    }
}

impl cmp::Eq for Scale {}

impl cmp::PartialOrd for Scale {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Scale {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ratio.cmp(other.ratio())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod scales {
        use super::*;
        use crate::common::TrackGauge;
        use crate::scales::test_data::{h0, n};
        use chrono::{DateTime, Utc};
        use pretty_assertions::{assert_eq, assert_ne};
        use rust_decimal::Decimal;
        use rust_decimal_macros::dec;

        #[test]
        fn it_should_create_new_scales() {
            let now: DateTime<Utc> = Utc::now();
            let id = ScaleId::new("H0");
            let ratio = Ratio::try_from(Decimal::from(87)).unwrap();
            let gauge = Gauge::from_millimeters(TrackGauge::Standard, dec!(16.5)).unwrap();

            let standard = HashSet::from([Standard::NMRA]);

            let scale = Scale::new(
                id.clone(),
                "H0",
                Some("Scale H0"),
                ratio.clone(),
                gauge.clone(),
                standard.clone(),
                Metadata::created_at(now),
            );
            assert_eq!(&id, scale.scale_id());
            assert_eq!("H0", scale.name());
            assert_eq!(Some(&String::from("Scale H0")), scale.description());
            assert_eq!(&ratio, scale.ratio());
            assert_eq!(&gauge, scale.gauge());
            assert_eq!(&standard, scale.standards());
            assert_eq!(&Metadata::created_at(now), scale.metadata());
        }

        #[test]
        fn it_should_display_scales() {
            let scale = h0();
            assert_eq!("H0 (1:87)", scale.to_string());
        }

        #[test]
        fn it_should_compare_two_scales() {
            let n = n();
            let h0 = h0();

            assert_eq!(n, n);
            assert_ne!(n, h0);
        }

        #[test]
        fn it_sort_scales_by_their_ratios() {
            let n = n();
            let h0 = h0();

            assert!(n < h0, "n < h0");
            assert!(h0 > n, "h0 > n");
        }
    }
}
