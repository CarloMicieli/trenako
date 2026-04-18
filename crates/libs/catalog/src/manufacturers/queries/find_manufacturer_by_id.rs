use crate::manufacturers::manufacturer::Manufacturer;
use crate::manufacturers::manufacturer_id::ManufacturerId;
use async_trait::async_trait;
use common::queries::errors::{DatabaseError, QueryError};
use common::unit_of_work::{Database, UnitOfWork};

/// The query to find a modelling manufacturer by its `manufacturer_id`
pub async fn find_manufacturer_by_id<'db, U, Repo, DB>(
    manufacturer_id: &ManufacturerId,
    repo: Repo,
    db: DB,
) -> Result<Manufacturer, QueryError>
where
    U: UnitOfWork<'db>,
    Repo: FindManufacturerByIdRepository<'db, U>,
    DB: Database<'db, U>,
{
    let mut unit_of_work = db.begin().await?;

    let result = repo
        .find_by_id(manufacturer_id, &mut unit_of_work)
        .await?
        .map(Ok)
        .unwrap_or_else(|| Err(QueryError::EmptyResultSet));

    unit_of_work.commit().await?;

    result
}

/// The find manufacturer by id repository
#[async_trait]
pub trait FindManufacturerByIdRepository<'db, U: UnitOfWork<'db>> {
    async fn find_by_id(
        &self,
        manufacturer_id: &ManufacturerId,
        unit_of_work: &mut U,
    ) -> Result<Option<Manufacturer>, DatabaseError>;
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use async_trait::async_trait;
    use common::in_memory::InMemoryRepository;
    use common::unit_of_work::noop::NoOpUnitOfWork;
    use tokio;

    mod find_by_id_query {
        use super::*;
        use common::unit_of_work::noop::NoOpDatabase;
        use pretty_assertions::assert_eq;

        #[tokio::test]
        async fn it_should_return_a_result_when_the_manufacturer_is_found() {
            let repo = InMemoryFindManufacturerByIdRepository::with(manufacturer_with_name("ACME"));

            let result = find_manufacturer_by_id(&ManufacturerId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(ManufacturerId::new("ACME"), result.manufacturer_id);
            assert_eq!("ACME", result.name);
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_manufacturer_is_not_found() {
            let repo = InMemoryFindManufacturerByIdRepository::new();

            let result = find_manufacturer_by_id(&ManufacturerId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("No results were found", error.to_string());
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_query_fails() {
            let repo = FindManufacturerByIdRepositoryWithError;

            let result = find_manufacturer_by_id(&ManufacturerId::new("ACME"), repo, NoOpDatabase).await;

            assert!(result.is_err());
            let error = result.unwrap_err();
            assert_eq!("something bad happened", error.to_string());
            assert_eq!("something bad happened", error.to_string());
        }
    }

    struct InMemoryFindManufacturerByIdRepository(InMemoryRepository<ManufacturerId, Manufacturer>);

    impl InMemoryFindManufacturerByIdRepository {
        pub fn new() -> Self {
            InMemoryFindManufacturerByIdRepository(InMemoryRepository::empty())
        }

        pub fn with(manufacturer: Manufacturer) -> Self {
            let repo = InMemoryFindManufacturerByIdRepository(InMemoryRepository::empty());
            repo.0.add(manufacturer.manufacturer_id.clone(), manufacturer);
            repo
        }
    }

    #[async_trait]
    impl FindManufacturerByIdRepository<'static, NoOpUnitOfWork> for InMemoryFindManufacturerByIdRepository {
        async fn find_by_id(
            &self,
            manufacturer_id: &ManufacturerId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Manufacturer>, DatabaseError> {
            Ok(self.0.find_by_id(manufacturer_id))
        }
    }

    fn manufacturer_with_name(name: &str) -> Manufacturer {
        Manufacturer {
            manufacturer_id: ManufacturerId::new(name),
            name: name.to_string(),
            registered_company_name: None,
            organization_entity_type: None,
            group_name: None,
            description: Default::default(),
            address: None,
            contact_info: None,
            kind: Default::default(),
            status: None,
            socials: None,
            metadata: Default::default(),
        }
    }

    struct FindManufacturerByIdRepositoryWithError;

    #[async_trait]
    impl FindManufacturerByIdRepository<'static, NoOpUnitOfWork> for FindManufacturerByIdRepositoryWithError {
        async fn find_by_id(
            &self,
            _manufacturer_id: &ManufacturerId,
            _unit_of_work: &mut NoOpUnitOfWork,
        ) -> Result<Option<Manufacturer>, DatabaseError> {
            Err(DatabaseError::UnexpectedError(anyhow!("something bad happened")))
        }
    }
}
