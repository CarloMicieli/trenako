use crate::csv_record::CsvRecord;
use anyhow::Context;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::rolling_stock_request::RollingStockRequest;
use catalog::manufacturers::manufacturer_id::ManufacturerId;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;

pub fn read_catalog_items(file_path: &str) -> Result<Vec<CatalogItemRequest>, anyhow::Error> {
    let file = File::open(file_path)?;

    let mut catalog_items: HashMap<CatalogItemId, CatalogItemRequest> = HashMap::new();

    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut line = 2;

    for result in rdr.deserialize() {
        let result: csv::Result<CsvRecord> = result;
        match result {
            Ok(record) => {
                let manufacturer_id = ManufacturerId::new(&record.manufacturer);
                let catalog_item_id = CatalogItemId::of(&manufacturer_id, record.item_number.as_ref().unwrap());

                let catalog_item = catalog_items.entry(catalog_item_id).or_insert_with(|| {
                    record
                        .clone()
                        .try_into()
                        .with_context(|| format!("(at line {})", line))
                        .unwrap()
                });

                let rolling_stock: RollingStockRequest =
                    record.try_into().with_context(|| format!("(at line {})", line))?;
                catalog_item.rolling_stocks.push(rolling_stock);
            }
            Err(why) => {
                eprintln!("(at line {}) {:?}", line, why);
            }
        }
        line += 1;
    }

    let output: Vec<CatalogItemRequest> = catalog_items
        .into_values()
        .sorted_by(|a, b| a.manufacturer.cmp(&b.manufacturer))
        .sorted_by(|a, b| a.item_number.cmp(&b.item_number))
        .collect();
    Ok(output)
}
