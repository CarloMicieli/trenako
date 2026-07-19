use crate::hateoas::links::{Link, LinkRelation};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// A trait for representation models to collect links.
pub trait RepresentationModel {
    /// Adds the given link to the resource.
    fn add_link(&mut self, link: Link);

    /// Adds all given Links to the resource.
    fn add_links(&mut self, links: &mut Vec<Link>);

    /// Returns all Links contained in this resource.
    fn get_links(&self) -> &[Link];

    /// Returns whether the resource contains a Link with the given rel.
    fn has_link(&self, rel: LinkRelation) -> bool {
        self.get_links().iter().any(|it| it.rel == rel)
    }

    /// Returns whether the resource contains Links at all.
    fn has_links(&self) -> bool {
        !self.get_links().is_empty()
    }
}

/// A simple EntityModel wrapping a domain object and adding links to it.
#[derive(Debug, PartialEq, Clone, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    #[serde(rename = "_links")]
    pub links: Vec<Link>,

    #[serde(flatten)]
    pub content: T,
}

impl<T> EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    /// Creates a new EntityModel for the given content and links.
    pub fn of(content: T, links: Vec<Link>) -> Self {
        EntityModel { content, links }
    }
}

impl<T> RepresentationModel for EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    fn add_links(&mut self, links: &mut Vec<Link>) {
        self.links.append(links);
    }

    fn get_links(&self) -> &[Link] {
        &self.links
    }
}

impl<T> IntoResponse for EntityModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    #[serde(rename = "_links")]
    pub links: Vec<Link>,

    pub items: Vec<T>,
}

impl<T> CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    /// Creates a new collection model with links
    pub fn of(items: Vec<T>, links: Vec<Link>) -> Self {
        CollectionModel { items, links }
    }
}

impl<T> RepresentationModel for CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    fn add_links(&mut self, links: &mut Vec<Link>) {
        self.links.append(links);
    }

    fn get_links(&self) -> &[Link] {
        &self.links
    }
}

impl<T> IntoResponse for CollectionModel<T>
where
    T: Serialize + PartialEq + Clone,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::Deserialize;

    mod entity_model_tests {
        use super::*;
        use crate::testing::extract_body;
        use axum::http::StatusCode;

        #[test]
        fn it_should_serialize_entity_models_without_links() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            let entity_model = EntityModel::of(content, Vec::new());

            let result = serde_json::json!(entity_model);
            let json = result.to_string();

            assert_eq!(r#"{"_links":[],"label":"label","value":42}"#, json);
        }

        #[test]
        fn it_should_serialize_entity_models_with_links() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };

            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the self link is invalid");
            let mut entity_model = EntityModel::of(content, Vec::new());
            entity_model.add_link(link);

            let result = serde_json::json!(entity_model);
            let json = result.to_string();

            assert_eq!(
                r#"{"_links":[{"href":"http://localhost/","rel":"self"}],"label":"label","value":42}"#,
                json
            );
        }

        #[test]
        fn it_should_add_links_to_entity_models() {
            let mut entity_model = new_entity_model(Vec::new());

            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the self link is invalid");
            entity_model.add_link(link);

            let links = entity_model.get_links();

            assert_eq!(1, links.len());
            assert!(entity_model.has_links());
            assert!(entity_model.has_link(LinkRelation::SelfLink));
            assert!(!entity_model.has_link(LinkRelation::Item));
        }

        #[tokio::test]
        async fn it_should_produce_responses() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };

            let link = Link::of("http://localhost", LinkRelation::SelfLink).expect("the self link is invalid");
            let mut entity_model = EntityModel::of(content, Vec::new());
            entity_model.add_link(link);

            let response = entity_model.into_response();
            assert_eq!(StatusCode::OK, response.status());

            let body = extract_body(response)
                .await
                .expect("unable to extract the request body");
            let body = serde_json::from_slice::<EntityModel<TestStruct>>(&body).unwrap();
            assert_eq!("label", body.content.label);
            assert_eq!(42, body.content.value);
        }

        fn new_entity_model(links: Vec<Link>) -> EntityModel<TestStruct> {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            EntityModel::of(content, links)
        }
    }

    mod collection_model_tests {
        use super::*;
        use crate::testing::extract_body;
        use axum::http::StatusCode;

        #[test]
        fn it_should_serialize_collection_models_without_links() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            let collection = vec![content];
            let links = Vec::new();
            let collection_model = CollectionModel::of(collection, links);

            let result = serde_json::json!(collection_model);
            let json = result.to_string();

            assert_eq!(r#"{"_links":[],"items":[{"label":"label","value":42}]}"#, json);
        }

        #[tokio::test]
        async fn it_should_produce_responses() {
            let content = TestStruct {
                label: "label".to_string(),
                value: 42,
            };
            let collection = vec![content];
            let links = Vec::new();
            let collection_model = CollectionModel::of(collection, links);

            let response = collection_model.into_response();
            assert_eq!(StatusCode::OK, response.status());

            let body = extract_body(response)
                .await
                .expect("unable to extract the request body");
            let body = serde_json::from_slice::<CollectionModel<TestStruct>>(&body).unwrap();

            assert_eq!(1, body.items.len());
            assert_eq!("label", body.items[0].label);
            assert_eq!(42, body.items[0].value);
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct TestStruct {
        pub label: String,
        pub value: i32,
    }
}
