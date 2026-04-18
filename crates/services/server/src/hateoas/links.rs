use serde::Serialize;
use thiserror::Error;
use url::{ParseError, Url};

/// It represents a resource Link.
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Link {
    pub href: Url,
    pub rel: LinkRelation,
}

impl Link {
    /// Creates a new Link to the given URI with the given rel.
    pub fn of(href: &str, rel: LinkRelation) -> Result<Self, LinkError> {
        if href.is_empty() {
            Err(LinkError::EmptyHref)
        } else {
            Ok(Link {
                href: Url::parse(href)?,
                rel,
            })
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum LinkError {
    #[error("Invalid href: {0}")]
    InvalidHref(#[from] ParseError),

    #[error("Invalid href: it cannot be empty")]
    EmptyHref,
}

/// The enumeration of link relation types
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[serde(rename_all = "lowercase")]
pub enum LinkRelation {
    /// An IRI that refers to the furthest preceding resource in a series of resources.
    First,

    /// The target IRI points to a resource that is a member of the collection represented
    /// by the context IRI.
    Item,

    /// An IRI that refers to the furthest following resource in a series of resources.
    Last,

    /// Indicates that the link's context is a part of a series, and that the next in the series
    /// is the link target.
    Next,

    /// Indicates that the link's context is a part of a series, and that the previous in the
    /// series is the link target.
    Previous,

    /// Conveys an identifier for the link's context.
    #[serde(rename = "self")]
    SelfLink,
}

/// A Links builder
#[derive(Debug)]
pub struct LinkBuilder {
    base_url: String,
}

impl LinkBuilder {
    /// Creates a new Links builder
    pub fn new(base_url: &str) -> Self {
        LinkBuilder {
            base_url: base_url.to_owned(),
        }
    }

    /// Start the creation of a self Link.
    pub fn link_to<'a>(&'a self, resource: &'a str) -> SelfLinkBuilder<'a> {
        SelfLinkBuilder {
            base_url: &self.base_url,
            resource,
        }
    }
}

#[derive(Debug)]
pub struct SelfLinkBuilder<'a> {
    base_url: &'a str,
    resource: &'a str,
}

impl<'a> SelfLinkBuilder<'a> {
    /// Adds the given Id representation as sub-resource to the current Link.
    pub fn slash<Id: AsRef<str>>(&self, id: Id) -> Result<Link, LinkError> {
        let href = format!("{}{}/{}", self.base_url, self.resource, id.as_ref());
        Link::of(&href, LinkRelation::SelfLink)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod links {
        use super::*;

        #[test]
        fn it_should_create_new_links() {
            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the link is not valid");
            assert_eq!("http://localhost/", link.href.to_string());
            assert_eq!(LinkRelation::SelfLink, link.rel);
        }

        #[test]
        fn it_should_return_an_error_creating_link_when_the_href_is_empty() {
            let result = Link::of("", LinkRelation::SelfLink);
            let error = result.unwrap_err();
            assert_eq!(LinkError::EmptyHref, error);
        }

        #[test]
        fn it_should_return_an_error_creating_link_when_the_href_is_not_a_valid_url() {
            let result = Link::of("invalid url", LinkRelation::SelfLink);
            let error = result.unwrap_err();

            assert_eq!(LinkError::InvalidHref(ParseError::RelativeUrlWithoutBase), error);
        }
    }

    mod links_builder {
        use super::*;

        #[test]
        fn it_should_create_self_links() {
            let builder = LinkBuilder::new("http://localhost:8000");
            let link = builder.link_to("/manufacturers").slash("my-id").expect("invalid self link");

            assert_eq!(LinkRelation::SelfLink, link.rel);
            assert_eq!("http://localhost:8000/manufacturers/my-id", link.href.to_string());
        }
    }
}
