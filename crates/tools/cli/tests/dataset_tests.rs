use cli::CliError;
use cli::dataset::Dataset;
use pretty_assertions::assert_eq;

const DATASET_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/dataset");

#[test]
fn it_should_load_a_dataset() {
    let dataset = Dataset::from_path(DATASET_ROOT).expect("a valid dataset");
    assert_eq!(dataset.brands.len(), 2);
    assert_eq!(dataset.catalog_items.len(), 2);
    assert_eq!(dataset.railways.len(), 2);
    assert_eq!(dataset.scales.len(), 2);
    assert_eq!(dataset.brands[0].file_name, String::from("acme.json"));
    assert_eq!(dataset.brands[1].file_name, String::from("piko.json"));
    assert_eq!(dataset.catalog_items[0].file_name, String::from("43277.3.json"));
    assert_eq!(dataset.catalog_items[1].file_name, String::from("60030.json"));
    assert_eq!(dataset.railways[0].file_name, String::from("db.json"));
    assert_eq!(dataset.railways[1].file_name, String::from("fs.json"));
    assert_eq!(dataset.scales[0].file_name, String::from("h0.json"));
    assert_eq!(dataset.scales[1].file_name, String::from("n.json"));
}

#[test]
fn it_should_return_an_error_when_the_path_is_not_found() {
    let result = Dataset::from_path("not-found");
    assert_eq!(Err(CliError::PathNotFound(String::from("not-found"))), result);
}
