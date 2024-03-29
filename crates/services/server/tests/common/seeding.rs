use ::data::catalog::brands::repositories::BrandsRepository;
use ::data::catalog::catalog_item::repositories::{CatalogItemsRepository, RollingStocksRepository};
use ::data::catalog::railways::repositories::RailwaysRepository;
use ::data::catalog::scales::repositories::ScalesRepository;
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::commands::new_brand::NewBrandCommand;
use catalog::brands::commands::repositories::NewBrandRepository;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::commands::new_catalog_item::NewCatalogItemCommand;
use catalog::catalog_items::commands::repositories::{NewCatalogItemRepository, NewRollingStockRepository};
use catalog::railways::commands::new_railways::NewRailwayCommand;
use catalog::railways::commands::repositories::NewRailwayRepository;
use catalog::railways::railway_request::RailwayRequest;
use catalog::scales::commands::new_scales::NewScaleCommand;
use catalog::scales::commands::repositories::NewScaleRepository;
use catalog::scales::scale_request::ScaleRequest;
use common::unit_of_work::postgres::PgDatabase;
use common::unit_of_work::{Database, UnitOfWork};
use serde_derive::Deserialize;
use sqlx::PgPool;

pub async fn seed_brands(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = BrandsRepository;

    let brands = brands();
    let brands: Vec<NewBrandCommand> = brands
        .items
        .into_iter()
        .map(|it| NewBrandCommand::try_from(it).expect("invalid brand request"))
        .collect();

    for brand_cmd in brands {
        repo.insert(&brand_cmd, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_catalog_items(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = CatalogItemsRepository;
    let rs_repo = RollingStocksRepository;

    let catalog_items: Vec<NewCatalogItemCommand> = catalog_items()
        .items
        .into_iter()
        .map(|it| NewCatalogItemCommand::try_from(it).expect("invalid catalog item request"))
        .collect();

    for catalog_item_cmd in catalog_items {
        repo.insert(&catalog_item_cmd, &mut unit_of_work).await.unwrap();

        for rs_cmd in catalog_item_cmd.rolling_stocks {
            rs_repo.insert(&rs_cmd, &mut unit_of_work).await.unwrap();
        }
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_railways(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = RailwaysRepository;

    let railways: Vec<NewRailwayCommand> = railways()
        .items
        .into_iter()
        .map(|it| NewRailwayCommand::try_from(it).expect("invalid railway request"))
        .collect();

    for railway_cmd in railways {
        repo.insert(&railway_cmd, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

pub async fn seed_scales(pg_pool: &PgPool) {
    let db = PgDatabase::new(pg_pool);
    let mut unit_of_work = db.begin().await.unwrap();

    let repo = ScalesRepository;

    let scales: Vec<NewScaleCommand> = scales()
        .items
        .into_iter()
        .map(|it| NewScaleCommand::try_from(it).expect("invalid scale request"))
        .collect();

    for scale_cmd in scales {
        repo.insert(&scale_cmd, &mut unit_of_work).await.unwrap();
    }

    unit_of_work.commit().await.unwrap();
}

fn brands() -> Brands {
    serde_json::from_str::<Brands>(data::BRANDS).expect("Invalid BRANDS data for seeding")
}

fn catalog_items() -> CatalogItems {
    serde_json::from_str::<CatalogItems>(data::CATALOG_ITEMS).expect("Invalid CATALOG_ITEMS data for seeding")
}

fn railways() -> Railways {
    serde_json::from_str::<Railways>(data::RAILWAYS).expect("Invalid RAILWAYS data for seeding")
}

fn scales() -> Scales {
    serde_json::from_str::<Scales>(data::SCALES).expect("Invalid SCALES data for seeding")
}

#[cfg(not(windows))]
mod data {
    pub const BRANDS: &str = include_str!("../resources/brands.json");
    pub const CATALOG_ITEMS: &str = include_str!("../resources/catalog_items.json");
    pub const RAILWAYS: &str = include_str!("../resources/railways.json");
    pub const SCALES: &str = include_str!("../resources/scales.json");
}

#[cfg(windows)]
mod data {
    pub const BRANDS: &str = include_str!("..\\resources\\brands.json");
    pub const CATALOG_ITEMS: &str = include_str!("..\\resources\\catalog_items.json");
    pub const RAILWAYS: &str = include_str!("..\\resources\\railways.json");
    pub const SCALES: &str = include_str!("..\\resources\\scales.json");
}

#[derive(Deserialize)]
struct Brands {
    items: Vec<BrandRequest>,
}

#[derive(Deserialize)]
struct CatalogItems {
    items: Vec<CatalogItemRequest>,
}

#[derive(Deserialize)]
struct Railways {
    items: Vec<RailwayRequest>,
}

#[derive(Deserialize)]
struct Scales {
    items: Vec<ScaleRequest>,
}
