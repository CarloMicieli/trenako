use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

/// A Trenako Resource Name (TRN)
///
/// # Details
///
/// A Trenako Resource Name (TRN) is a Uniform Resource Identifier (URI) that uses the urn scheme.
/// TRNs are globally unique persistent identifiers assigned within defined namespaces so they will
/// be available for a long period of time, even after the resource which they identify ceases to
/// exist or becomes unavailable.
///
/// TRNs cannot be used to directly locate an item and need not be resolvable, as they are
/// simply templates that another parser may use to find an item.
///
/// # Description
///
/// A Trenako Resource Name (TRN) is a Uniform Resource Identifier (URI)
/// that is assigned under the "trn" URI scheme and a particular URN
/// namespace, with the intent that the URN will be a persistent,
/// location-independent resource identifier.
///
/// TRNs have two main parts:
///
/// - _NID_: The identifier associated with a TRN namespace.
/// - _NSS_: The TRN-namespace-specific part of a TRN.
///
/// ## Namespace Identifier (NID)
///
/// NIDs are case insensitive (e.g., "ISBN" and "isbn" are equivalent).
///
/// Characters outside the ASCII range are not permitted in NIDs,
/// and no encoding mechanism for such characters is supported.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Trn {
    nid: String,
    nss: String,
}

static PREFIX: &str = "trn";
static SEP: &str = ":";

impl Trn {
    /// Creates a new `Trn` value
    pub fn new(nid: &str, nss: &str) -> Result<Self, TrnError> {
        if nid.is_empty() {
            return Err(TrnError::EmptyNid);
        }

        if nss.is_empty() {
            return Err(TrnError::EmptyNss);
        }

        if nid == PREFIX {
            return Err(TrnError::InvalidNid);
        }

        if !nid_is_valid(nid) {
            return Err(TrnError::InvalidNid);
        }

        if !nss_is_valid(nss) {
            return Err(TrnError::InvalidNss);
        }

        Ok(Trn {
            nid: String::from(nid),
            nss: String::from(nss),
        })
    }

    /// Returns the namespace ID
    pub fn nid(&self) -> &str {
        self.nid.as_ref()
    }

    /// Returns the namespace specific string
    pub fn nss(&self) -> &str {
        self.nss.as_ref()
    }

    /// Creates a new Trenako Resource Name (TRN) for an instance
    pub fn instance(id: &Uuid) -> Self {
        Trn::new("instance", &id.to_string()).unwrap()
    }
}

fn nid_is_valid(nid: &str) -> bool {
    lazy_static! {
        static ref RE_NID: Regex = Regex::new("^[0-9a-z][0-9a-z-]{0,30}[0-9a-z]$").unwrap();
    }
    RE_NID.is_match(nid)
}

fn nss_is_valid(nss: &str) -> bool {
    lazy_static! {
        static ref RE_NSS: Regex = Regex::new("^(([\\-a-zA-Z0-9/]|%[0-9a-fA-F]{2})*)+(\\?\\w+(=([\\-a-zA-Z0-9/]|%[0-9a-fA-F]{2})*)?(&\\w+(=([\\-a-zA-Z0-9/]|%[0-9a-fA-F]{2})*)?)*)?\\*?$").unwrap();
    }
    RE_NSS.is_match(nss)
}

impl FromStr for Trn {
    type Err = TrnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(PREFIX) {
            return Err(TrnError::WrongTrnPrefix);
        }

        let remaining: &str = &s[PREFIX.len() + 1..];

        let tokens: Vec<&str> = remaining.split(SEP).collect();
        if tokens.len() != 2 {
            return Err(TrnError::InvalidTrn);
        }

        Trn::new(tokens[0], tokens[1])
    }
}

impl PartialOrd for Trn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let compare_nid = self.nid.partial_cmp(&other.nid);
        if let Some(Ordering::Equal) = compare_nid {
            self.nss.partial_cmp(&other.nss)
        } else {
            compare_nid
        }
    }
}

impl fmt::Display for Trn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", PREFIX, self.nid, self.nss)
    }
}

impl Serialize for Trn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct TrnVisitor;

impl<'de> Visitor<'de> for TrnVisitor {
    type Value = Trn;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the input is not a valid trn")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(trn) = Trn::from_str(s) {
            Ok(trn)
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Trn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TrnVisitor)
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum TrnError {
    #[error("invalid trn")]
    InvalidTrn,
    #[error("invalid trn: the identifier is empty")]
    EmptyNid,
    #[error("invalid trn: the namespace is empty")]
    EmptyNss,
    #[error("invalid trn: it must start with 'trn:'")]
    WrongTrnPrefix,
    #[error("invalid trn: the identifier is not valid")]
    InvalidNid,
    #[error("invalid trn: the namespace is not valid")]
    InvalidNss,
}

#[cfg(test)]
mod test {
    use super::*;

    mod serde_tests {
        use super::*;
        use serde::Serialize;

        #[test]
        fn it_should_serialize_trn_values() {
            let value = TestStruct {
                trn: Trn::new("first", "second").unwrap(),
            };

            let json_value = serde_json::to_string(&value);

            assert_eq!(r#"{"trn":"trn:first:second"}"#, json_value.unwrap());
        }

        #[test]
        fn it_should_deserialize_trn_values() {
            let result: serde_json::Result<TestStruct> = serde_json::from_str(r#"{"trn":"trn:first:second"}"#);
            let value = TestStruct {
                trn: Trn::new("first", "second").unwrap(),
            };
            assert_eq!(value, result.unwrap());
        }

        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct TestStruct {
            pub trn: Trn,
        }
    }

    mod trn_values {
        use super::*;
        use rstest::rstest;

        #[test]
        fn it_should_create_a_trn_for_instances() {
            let id = Uuid::new_v4();

            let trn = Trn::instance(&id);

            assert_eq!(format!("trn:instance:{}", id), trn.to_string());
        }

        #[test]
        fn it_should_start_with_the_trn_prefix() {
            let result = Trn::from_str("invalid:first:second");

            assert!(result.is_err());
            let error = result.expect_err("the trn should not be valid");
            assert_eq!(error, TrnError::WrongTrnPrefix);
        }

        #[rstest]
        #[case("trn")]
        #[case("@@@@@@")]
        #[case("first name")]
        #[case("-invalid")]
        #[case("123456789012345678901234567890123")]
        fn it_should_validate_the_trn_identifier(#[case] nid: &str) {
            let result = Trn::new(nid, "second-item");

            assert!(result.is_err());
            let error = result.expect_err("the trn should not be valid");
            assert_eq!(error, TrnError::InvalidNid);
        }

        #[rstest]
        #[case("@@@@@@")]
        #[case("first name")]
        fn it_should_validate_the_trn_namespace(#[case] nss: &str) {
            let result = Trn::new("my-trn", nss);

            assert!(result.is_err());
            let error = result.expect_err("the trn should not be valid");
            assert_eq!(error, TrnError::InvalidNss);
        }

        #[test]
        fn it_should_create_new_trn_values() {
            let v = Trn::from_str("trn:first:second-item");
            let trn = v.expect("invalid trn value");

            assert_eq!(trn.nid(), "first");
            assert_eq!(trn.nss(), "second-item");

            assert_eq!(trn.to_string(), "trn:first:second-item");
        }

        #[test]
        fn it_should_compare_two_trn_values() {
            let trn1 = Trn::new("id1", "nss1").unwrap();
            let trn2 = Trn::new("id1", "nss2").unwrap();
            let trn3 = Trn::new("id2", "nss1").unwrap();

            assert_eq!(trn1.partial_cmp(&trn2), Some(Ordering::Less));
            assert_eq!(trn1.partial_cmp(&trn3), Some(Ordering::Less));
            assert_eq!(trn2.partial_cmp(&trn3), Some(Ordering::Less));
            assert_eq!(trn2.partial_cmp(&trn1), Some(Ordering::Greater));
            assert_eq!(trn3.partial_cmp(&trn1), Some(Ordering::Greater));
            assert_eq!(trn3.partial_cmp(&trn2), Some(Ordering::Greater));
        }
    }
}
