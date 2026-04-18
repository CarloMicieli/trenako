use crate::csv_record::CsvRecord;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;

pub fn write_catalog_items(file_path: &str, items: Vec<CatalogItemRequest>) -> Result<(), anyhow::Error> {
    let mut wtr = csv::Writer::from_path(file_path)?;

    for item in items {
        let rec = csv_record_from_catalog_item(item);
        wtr.serialize(&rec)?;
    }

    wtr.flush()?;
    Ok(())
}

fn csv_record_from_catalog_item(item: CatalogItemRequest) -> Vec<CsvRecord> {
    let main_record = CsvRecord {
        manufacturer: item.manufacturer,
        item_number: Some(item.item_number.clone()),
        scale: item.scale,
        power_method: Some(item.power_method),
        description: item.description.italian().unwrap_or(&String::from("")).clone(),
        details: item.details.italian().unwrap_or(&String::from("")).clone(),
        delivery_date: item.delivery_date.clone(),
        availability: item.availability_status,
        count: Some(item.count),
        ..CsvRecord::default()
    };

    vec![main_record]
}
