use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::commands::new_catalog_item::{NewCatalogItemCommand, NewRollingStockCommand};
use crate::railways::railway_id::RailwayId;
use crate::scales::scale_id::ScaleId;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;

/// The persistence related functionality for the catalog item commands
#[async_trait]
pub trait NewCatalogItemRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a catalog with the input id already exists
    async fn exists(&self, catalog_item_id: &CatalogItemId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Inserts a new catalog item
    async fn insert(&self, new_item: &NewCatalogItemCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;

    /// Checks if the manufacturer exists
    async fn manufacturer_exists(&self, manufacturer_id: &ManufacturerId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Checks if the scale exists
    async fn scale_exists(&self, scale_id: &ScaleId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;
}

/// The persistence related functionality for the rolling stock commands
#[async_trait]
pub trait NewRollingStockRepository<'db, U: UnitOfWork<'db>> {
    /// Inserts a new catalog item
    async fn insert(&self, new_item: &NewRollingStockCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;

    /// Checks if the railway exists
    async fn railway_exists(&self, railway_id: &RailwayId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;
}

#[cfg(test)]
pub mod in_memory {
    use crate::manufacturers::manufacturer_id::ManufacturerId;
    use crate::catalog_items::catalog_item_id::CatalogItemId;
    use crate::catalog_items::commands::new_catalog_item::{NewCatalogItemCommand, NewRollingStockCommand};
    use crate::catalog_items::commands::repositories::{NewCatalogItemRepository, NewRollingStockRepository};
    use crate::catalog_items::rolling_stock_id::RollingStockId;
    use crate::railways::railway_id::RailwayId;
    use crate::scales::scale_id::ScaleId;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use std::str::FromStr;

    /// An in-memory catalog item repository
    pub struct InMemoryCatalogItemRepository {
        catalog_items: InMemoryRepository<CatalogItemId, NewCatalogItemCommand>,
        manufacturers: Vec<ManufacturerId>,
        scales: Vec<ScaleId>,
    }

    impl InMemoryCatalogItemRepository {
        /// Creates an empty in memory catalog items repository
        pub fn empty() -> Self {
            InMemoryCatalogItemRepository {
                catalog_items: InMemoryRepository::empty(),
                manufacturers: Vec::new(),
                scales: Vec::new(),
            }
        }

        /// Creates a new in-memory catalog items repository with an initial element
        pub fn with(command: NewCatalogItemCommand) -> Self {
            let id = CatalogItemId::from_str(&command.catalog_item_id.to_string()).unwrap();
            InMemoryCatalogItemRepository {
                catalog_items: InMemoryRepository::of(id, command),
                manufacturers: Vec::new(),
                scales: Vec::new(),
            }
        }

        /// Adds the given manufacturer id to the current repository
        pub fn with_manufacturer(mut self, manufacturer_id: ManufacturerId) -> Self {
            self.manufacturers.push(manufacturer_id);
            self
        }

        /// Adds the given scale id to the current repository
        pub fn with_scale(mut self, scale_id: ScaleId) -> Self {
            self.scales.push(scale_id);
            self
        }
    }

    #[async_trait]
    impl NewCatalogItemRepository<'static, NoOpUnitOfWork> for InMemoryCatalogItemRepository {
        async fn exists(
            &self,
            catalog_item_id: &CatalogItemId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool, anyhow::Error> {
            Ok(self.catalog_items.contains(catalog_item_id))
        }

        async fn insert(
            &self,
            new_item: &NewCatalogItemCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let id = CatalogItemId::from_str(&new_item.catalog_item_id.to_string()).unwrap();
            self.catalog_items.add(id, new_item.clone());
            Ok(())
        }

        async fn manufacturer_exists(
            &self,
            manufacturer_id: &ManufacturerId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool, anyhow::Error> {
            let result = self.manufacturers.contains(manufacturer_id);
            Ok(result)
        }

        async fn scale_exists(
            &self,
            scale_id: &ScaleId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool, anyhow::Error> {
            let result = self.scales.contains(scale_id);
            Ok(result)
        }
    }

    /// An in-memory rolling stock repository
    pub struct InMemoryRollingStockRepository {
        rolling_stocks: InMemoryRepository<RollingStockId, NewRollingStockCommand>,
        railways: Vec<RailwayId>,
    }

    impl InMemoryRollingStockRepository {
        /// Creates an empty in memory rolling stocks repository
        pub fn empty() -> Self {
            InMemoryRollingStockRepository {
                rolling_stocks: InMemoryRepository::empty(),
                railways: Vec::new(),
            }
        }

        /// Adds the given railway id to the current repository
        pub fn with_railway(mut self, railway_id: RailwayId) -> Self {
            self.railways.push(railway_id);
            self
        }
    }

    #[async_trait]
    impl NewRollingStockRepository<'static, NoOpUnitOfWork> for InMemoryRollingStockRepository {
        async fn insert(
            &self,
            new_item: &NewRollingStockCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let rolling_stock_id = new_item.rolling_stock_id;
            self.rolling_stocks.add(rolling_stock_id, new_item.clone());
            Ok(())
        }

        async fn railway_exists(
            &self,
            railway_id: &RailwayId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<bool, anyhow::Error> {
            let result = self.railways.contains(railway_id);
            Ok(result)
        }
    }
}
