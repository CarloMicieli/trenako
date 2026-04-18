pub mod import;

use crate::{CliError, Result};
use serde_derive::Serialize;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{fmt, result};
use std::{fs, str};
use thiserror::Error;
use walkdir::WalkDir;

/// It represent a dataset
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Dataset {
    /// the manufacturers resources (path contains `manufacturers`)
    pub manufacturers: Vec<Resource>,
    /// the catalog items resources (path contains `catalog_items`)
    pub catalog_items: Vec<Resource>,
    /// the railways resources (path contains `railways`)
    pub railways: Vec<Resource>,
    /// the scale resources (path contains `scales`)
    pub scales: Vec<Resource>,
}

impl Dataset {
    /// Creates a new dataset from a resources list
    pub fn new(resources: Vec<Resource>) -> Self {
        let mut dataset = Dataset::default();
        for resource in resources {
            match resource.resource_type {
                ResourceType::Manufacturers => dataset.manufacturers.push(resource),
                ResourceType::CatalogItems => dataset.catalog_items.push(resource),
                ResourceType::Railways => dataset.railways.push(resource),
                ResourceType::Scales => dataset.scales.push(resource),
            }
        }

        dataset.sort();
        dataset
    }

    /// Creates a new Dataset from a filesystem path
    pub fn from_path(root: &str) -> Result<Self> {
        let resources = read_files(root)?;
        Ok(Dataset::new(resources))
    }

    fn sort(&mut self) {
        self.manufacturers.sort();
        self.catalog_items.sort();
        self.railways.sort();
        self.scales.sort();
    }
}

impl Display for Dataset {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} manufacturer(s)\n{} catalog item(s)\n{} railway(s)\n{} scale(s)",
            self.manufacturers.len(),
            self.catalog_items.len(),
            self.railways.len(),
            self.scales.len()
        )
    }
}

fn read_files(root: &str) -> Result<Vec<Resource>> {
    let root_path = Path::new(root);

    if !root_path.exists() {
        return Err(CliError::PathNotFound(String::from(root)));
    }

    let mut results: Vec<Resource> = vec![];
    for entry in WalkDir::new(root_path).min_depth(0).max_depth(4) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file_name = filename_from_path(entry.path())?;
            let content = fs::read_to_string(entry.path()).unwrap();
            let resource_type = resource_type_from_path(entry.path())?;

            results.push(Resource {
                file_name,
                content,
                resource_type,
            });
        }
    }
    Ok(results)
}

fn filename_from_path(path: &Path) -> Result<String> {
    let file_name = path.file_name().and_then(|x| x.to_str()).map(|x| x.to_string());
    if let Some(file_name) = file_name {
        Ok(file_name)
    } else {
        Err(CliError::InvalidFileName)
    }
}

fn resource_type_from_path(path: &Path) -> result::Result<ResourceType, ResourceTypeError> {
    for component in path.components() {
        if let Some(resource_type) = ResourceType::from_os_str(component.as_os_str()) {
            return Ok(resource_type);
        }
    }

    Err(ResourceTypeError::ResourceTypeNotFound(String::from("")))
}

/// A dataset resource element
#[derive(Debug, PartialEq, Eq)]
pub struct Resource {
    pub file_name: String,
    pub resource_type: ResourceType,
    pub content: String,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] file_name: {}", self.resource_type, self.file_name)
    }
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Resource {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_name.cmp(&other.file_name)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub enum ResourceType {
    Manufacturers,
    CatalogItems,
    Railways,
    Scales,
}

impl ResourceType {
    fn from_os_str(input: &OsStr) -> Option<ResourceType> {
        if let Some(s) = input.to_str() {
            match s {
                "manufacturers" => Some(ResourceType::Manufacturers),
                "catalog_items" => Some(ResourceType::CatalogItems),
                "railways" => Some(ResourceType::Railways),
                "scales" => Some(ResourceType::Scales),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ResourceTypeError {
    #[error("unable to determine the resource type (path: {0})")]
    ResourceTypeNotFound(String),
}

#[cfg(test)]
mod test {
    use super::*;

    mod datasets {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_should_create_new_datasets() {
            let resources = vec![
                manufacturer("manufacturer2.json"),
                scale("scale2.json"),
                catalog_item("catalog-item.json"),
                scale("scale1.json"),
                railway("railway1.json"),
                manufacturer("manufacturer1.json"),
            ];

            let dataset = Dataset::new(resources);

            assert_eq!(2, dataset.manufacturers.len());
            assert_eq!(1, dataset.catalog_items.len());
            assert_eq!(1, dataset.railways.len());
            assert_eq!(2, dataset.scales.len());

            assert_eq!("manufacturer1.json", dataset.manufacturers[0].file_name);
            assert_eq!("manufacturer2.json", dataset.manufacturers[1].file_name);
            assert_eq!("catalog-item.json", dataset.catalog_items[0].file_name);
            assert_eq!("railway1.json", dataset.railways[0].file_name);
            assert_eq!("scale1.json", dataset.scales[0].file_name);
            assert_eq!("scale2.json", dataset.scales[1].file_name);
        }

        #[test]
        fn it_should_display_datasets() {
            let resources = vec![
                manufacturer("manufacturer2.json"),
                scale("scale2.json"),
                catalog_item("catalog-item.json"),
                scale("scale1.json"),
                railway("railway1.json"),
                manufacturer("manufacturer1.json"),
            ];

            let dataset = Dataset::new(resources);

            let expected = r#"2 manufacturer(s)
1 catalog item(s)
1 railway(s)
2 scale(s)"#;
            assert_eq!(expected, dataset.to_string());
        }
    }

    mod resources {
        use super::*;
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        #[rstest]
        #[case(resource("foo.json"), resource("bar.json"), true)]
        #[case(resource("bar.json"), resource("foo.json"), false)]
        #[case(resource("foo.json"), resource("Bar.json"), true)]
        #[case(resource("Bar.json"), resource("foo.json"), false)]
        fn it_should_compare_two_resources(#[case] lhs: Resource, #[case] rhs: Resource, #[case] expected: bool) {
            assert_eq!(expected, lhs > rhs);
        }

        #[test]
        fn it_should_display_resources() {
            let resource = resource("foo.json");
            assert_eq!("[Manufacturers] file_name: foo.json", resource.to_string());
        }
    }

    fn resource(file_name: &str) -> Resource {
        Resource {
            file_name: file_name.to_owned(),
            resource_type: ResourceType::Manufacturers,
            content: "".to_owned(),
        }
    }

    fn manufacturer(file_name: &str) -> Resource {
        Resource {
            file_name: file_name.to_owned(),
            resource_type: ResourceType::Manufacturers,
            content: "".to_owned(),
        }
    }

    fn catalog_item(file_name: &str) -> Resource {
        Resource {
            file_name: file_name.to_owned(),
            resource_type: ResourceType::CatalogItems,
            content: "".to_owned(),
        }
    }

    fn railway(file_name: &str) -> Resource {
        Resource {
            file_name: file_name.to_owned(),
            resource_type: ResourceType::Railways,
            content: "".to_owned(),
        }
    }

    fn scale(file_name: &str) -> Resource {
        Resource {
            file_name: file_name.to_owned(),
            resource_type: ResourceType::Scales,
            content: "".to_owned(),
        }
    }
}
