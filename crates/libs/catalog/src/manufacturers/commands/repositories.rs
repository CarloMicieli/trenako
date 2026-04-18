//! the manufacturer command repositories

use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::manufacturers::commands::new_manufacturer::NewManufacturerCommand;
use async_trait::async_trait;
use common::unit_of_work::UnitOfWork;

/// The persistence related functionality for the manufacturer commands
#[async_trait]
pub trait NewManufacturerRepository<'db, U: UnitOfWork<'db>> {
    /// Checks if a manufacturer with the input id already exists
    async fn exists(&self, manufacturer_id: &ManufacturerId, unit_of_work: &mut U) -> Result<bool, anyhow::Error>;

    /// Inserts a new manufacturer
    async fn insert(&self, new_manufacturer: &NewManufacturerCommand, unit_of_work: &mut U) -> Result<(), anyhow::Error>;
}

#[cfg(test)]
pub mod in_memory {
    use crate::manufacturers::manufacturer_id::ManufacturerId;
    use crate::manufacturers::commands::new_manufacturer::NewManufacturerCommand;
    use crate::manufacturers::commands::repositories::NewManufacturerRepository;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;

    /// An in-memory manufacturer repository
    pub struct InMemoryManufacturerRepository(InMemoryRepository<ManufacturerId, NewManufacturerCommand>);

    impl InMemoryManufacturerRepository {
        /// Creates an empty in memory manufacturers repository
        pub fn empty() -> Self {
            InMemoryManufacturerRepository(InMemoryRepository::empty())
        }

        /// Creates a new in-memory manufacturers repository with an initial element
        pub fn with(command: NewManufacturerCommand) -> Self {
            let id = ManufacturerId::new(&command.manufacturer_id);
            InMemoryManufacturerRepository(InMemoryRepository::of(id, command))
        }
    }

    #[async_trait]
    impl NewManufacturerRepository<'static, NoOpUnitOfWork> for InMemoryManufacturerRepository {
        async fn exists(&self, manufacturer_id: &ManufacturerId, _unit_of_work: &mut NoOpUnitOfWork) -> Result<bool, anyhow::Error> {
            Ok(self.0.contains(manufacturer_id))
        }

        async fn insert(
            &self,
            new_manufacturer: &NewManufacturerCommand,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<(), anyhow::Error> {
            let id = ManufacturerId::new(&new_manufacturer.manufacturer_id);
            self.0.add(id, new_manufacturer.clone());
            Ok(())
        }
    }
}
