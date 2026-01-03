use cli::cvs_files::read_catalog_items;
use cli::cvs_files::write_catalog_items;
use cli::dataset::Dataset;
use cli::dataset::import::import_catalog_items;
use cli::validator::validate_dataset;
use cli::{Command, cli_parser};
use serde_json::json;

fn main() {
    let command = cli_parser::parse();
    match command {
        Some(Command::CsvExport {
            source,
            output,
            dry_run: _,
        }) => {
            let result = Dataset::from_path(&source);
            match result {
                Ok(_dataset) => {
                    let catalog_items = Vec::new();
                    write_catalog_items(&output, catalog_items).expect("unable to export to the csv file")
                }
                Err(why) => why.display(),
            }
        }
        Some(Command::CsvImport {
            file,
            mode,
            output,
            dry_run,
        }) => {
            let catalog_items = read_catalog_items(&file).expect("unable to parse the csv file");
            import_catalog_items(catalog_items, mode, &output, dry_run).expect("unable to import the csv file");
        }
        Some(Command::Seed) => {}
        Some(Command::Validate(s)) => {
            let result = Dataset::from_path(&s);
            match result {
                Ok(dataset) => {
                    println!("{}", dataset);
                    let result = validate_dataset(dataset).unwrap();

                    for v in result.iter().filter(|x| !x.is_valid()) {
                        println!("{:#?}", json!(v));
                    }
                }
                Err(why) => why.display(),
            }
        }
        _ => {}
    }
}
