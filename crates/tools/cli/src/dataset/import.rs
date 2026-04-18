use crate::Mode;
use anyhow::Context;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use common::slug::Slug;
use std::fs;

pub fn import_catalog_items(
    catalog_items: Vec<CatalogItemRequest>,
    mode: Mode,
    output: &str,
    dry_run: bool,
) -> Result<(), anyhow::Error> {
    match mode {
        Mode::Json => {
            fs::create_dir_all(output).with_context(|| format!("unable to create the {} directory", output))?;

            for catalog_item in catalog_items {
                let json = serde_json::to_string_pretty(&catalog_item).expect("Invalid json value");

                if dry_run {
                    println!("{}", json);
                } else {
                    let manufacturer = Slug::new(&catalog_item.manufacturer.replace('/', "_"));
                    let item_number = &catalog_item.item_number;
                    let category = &catalog_item.category.to_string().to_lowercase();

                    let dir = format!("{}/{}/{}", output, manufacturer, category);
                    fs::create_dir_all(&dir).with_context(|| format!("unable to create the {} directory", &dir))?;
                    let filename = format!("{}/{}.json", &dir, item_number);
                    println!("Writing {}...", filename);

                    fs::write(filename, json).unwrap(); //.with_context(|| format!("unable to write file"))?
                }
            }
        }
    }

    Ok(())
}
