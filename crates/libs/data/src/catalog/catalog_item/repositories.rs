use crate::catalog::catalog_item::catalog_item_row::CatalogItemRow;
use crate::catalog::catalog_item::rolling_stock_row::RollingStockRow;
use anyhow::Context;
use async_trait::async_trait;
use catalog::catalog_items::availability_status::AvailabilityStatus;
use catalog::catalog_items::catalog_item::CatalogItem;
use catalog::catalog_items::catalog_item_id::CatalogItemId;
use catalog::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use catalog::catalog_items::commands::new_catalog_item::{NewCatalogItemCommand, NewRollingStockCommand};
use catalog::catalog_items::commands::repositories::{NewCatalogItemRepository, NewRollingStockRepository};
use catalog::catalog_items::control::{Control, DccInterface};
use catalog::catalog_items::power_method::PowerMethod;
use catalog::catalog_items::queries::find_catalog_item_by_id::{
    FindCatalogItemByIdRepository, FindRollingStocksByCatalogItemIdRepository,
};
use catalog::catalog_items::rolling_stock::RollingStock;
use catalog::catalog_items::rolling_stock_id::RollingStockId;
use catalog::catalog_items::service_level::ServiceLevel;
use catalog::catalog_items::technical_specifications::{BodyShellType, ChassisType, CouplingSocket, FeatureFlag};
use catalog::manufacturers::manufacturer_id::ManufacturerId;
use catalog::railways::railway_id::RailwayId;
use catalog::scales::scale_id::ScaleId;
use common::queries::converters::ToOutputConverter;
use common::queries::errors::DatabaseError;
use common::unit_of_work::postgres::PgUnitOfWork;

#[derive(Debug)]
pub struct CatalogItemsRepository;
#[derive(Debug)]
pub struct RollingStocksRepository;

#[async_trait]
impl<'db> NewCatalogItemRepository<'db, PgUnitOfWork<'db>> for CatalogItemsRepository {
    async fn exists(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT catalog_item_id FROM catalog_items WHERE catalog_item_id = $1 LIMIT 1",
            catalog_item_id as &CatalogItemId
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for the catalog item existence.")?;

        Ok(result.is_some())
    }

    async fn insert(
        &self,
        new_item: &NewCatalogItemCommand,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<(), anyhow::Error> {
        let catalog_item_id = &new_item.catalog_item_id;
        let request = &new_item.payload;
        let metadata = &new_item.metadata;

        let manufacturer_id = &request.manufacturer_id;
        let scale_id = &request.scale_id;

        sqlx::query!(
            r#"INSERT INTO catalog_items (
                catalog_item_id,
                manufacturer_id,
                item_number,
                scale_id,
                category,
                description_de,
                description_en,
                description_fr,
                description_it,
                details_en,
                details_it,
                power_method,
                epoch,
                delivery_date,
                availability_status,
                count,
                created_at,
                version
            )
            VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, 
                $13, $14, $15, $16, $17, $18
            )"#,
            catalog_item_id as &CatalogItemId,
            manufacturer_id as &ManufacturerId,
            request.item_number.value(),
            scale_id as &ScaleId,
            request.category as Category,
            request.description.german(),
            request.description.english(),
            request.description.french(),
            request.description.italian(),
            request.details.english(),
            request.details.italian(),
            request.power_method as PowerMethod,
            request.epoch.to_string(),
            request.delivery_date.as_ref().map(|x| x.to_string()),
            request.availability_status as Option<AvailabilityStatus>,
            request.count,
            metadata.created(),
            metadata.version() as i32
        )
        .execute(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a catalog item.")?;

        Ok(())
    }

    async fn manufacturer_exists(
        &self,
        manufacturer_id: &ManufacturerId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT manufacturer_id FROM manufacturers WHERE manufacturer_id = $1 LIMIT 1",
            manufacturer_id
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for manufacturer existence.")?;

        Ok(result.is_some())
    }

    async fn scale_exists(
        &self,
        scale_id: &ScaleId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!("SELECT scale_id FROM scales WHERE scale_id = $1 LIMIT 1", scale_id)
            .fetch_optional(&mut *unit_of_work.transaction)
            .await
            .context("A database failure was encountered while trying to check for scale existence.")?;

        Ok(result.is_some())
    }
}

#[async_trait]
impl<'db> NewRollingStockRepository<'db, PgUnitOfWork<'db>> for RollingStocksRepository {
    async fn insert(
        &self,
        new_item: &NewRollingStockCommand,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<(), anyhow::Error> {
        let request = &new_item.payload;
        let catalog_item_id = &new_item.catalog_item_id;
        let rolling_stock_id = &new_item.rolling_stock_id;
        let railway_id = &new_item.railway_id;

        sqlx::query!(
            r#"INSERT INTO rolling_stocks (
                        rolling_stock_id,
                        catalog_item_id,
                        railway_id,
                        rolling_stock_category,
                        livery,
                        length_over_buffers_mm,
                        length_over_buffers_in,
                        type_name,
                        road_number,
                        series,
                        depot,
                        dcc_interface,
                        control,
                        electric_multiple_unit_type,
                        freight_car_type,
                        locomotive_type,
                        passenger_car_type,
                        railcar_type,
                        service_level,
                        is_dummy,
                        minimum_radius,
                        coupling_socket,
                        close_couplers,
                        digital_shunting_coupling,
                        flywheel_fitted,
                        body_shell,
                        chassis,
                        interior_lights,
                        lights,
                        sprung_buffers
                    )
                    VALUES (
                        $1, $2, $3, $4, $5, $6,
                        $7, $8, $9, $10, $11, $12, 
                        $13, $14, $15, $16, $17, $18,
                        $19, $20, $21, $22, $23, $24, 
                        $25, $26, $27, $28, $29, $30
                    )"#,
            rolling_stock_id as &RollingStockId,
            catalog_item_id as &CatalogItemId,
            railway_id as &RailwayId,
            request.category.expect("rolling stock category is mandatory") as RollingStockCategory,
            request.livery,
            request.length_over_buffers_mm.map(|x| x.quantity()),
            request.length_over_buffers_in.map(|x| x.quantity()),
            request.type_name,
            request.road_number,
            request.series,
            request.depot,
            request.dcc_interface as Option<DccInterface>,
            request.control as Option<Control>,
            request.electric_multiple_unit_type as Option<ElectricMultipleUnitType>,
            request.freight_car_type as Option<FreightCarType>,
            request.locomotive_type as Option<LocomotiveType>,
            request.passenger_car_type as Option<PassengerCarType>,
            request.railcar_type as Option<RailcarType>,
            request.service_level as Option<ServiceLevel>,
            request.is_dummy,
            request.minimum_radius.map(|x| x.value().quantity()),
            request.coupling_socket as Option<CouplingSocket>,
            request.close_couplers as Option<FeatureFlag>,
            request.digital_shunting_coupling as Option<FeatureFlag>,
            request.flywheel_fitted as Option<FeatureFlag>,
            request.body_shell as Option<BodyShellType>,
            request.chassis as Option<ChassisType>,
            request.interior_lights as Option<FeatureFlag>,
            request.lights as Option<FeatureFlag>,
            request.sprung_buffers as Option<FeatureFlag>
        )
        .execute(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to store a rolling stock.")?;

        Ok(())
    }

    async fn railway_exists(
        &self,
        railway_id: &RailwayId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<bool, anyhow::Error> {
        let result = sqlx::query!(
            "SELECT railway_id FROM railways WHERE railway_id = $1 LIMIT 1",
            railway_id
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to check for a railway existence.")?;

        Ok(result.is_some())
    }
}

#[async_trait]
impl<'db> FindCatalogItemByIdRepository<'db, PgUnitOfWork<'db>> for CatalogItemsRepository {
    async fn find_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Option<CatalogItem>, DatabaseError> {
        let result = sqlx::query_as!(
            CatalogItemRow,
            r#"SELECT
                c.catalog_item_id as "catalog_item_id: CatalogItemId",
                c.item_number,
                c.manufacturer_id as "manufacturer_id: ManufacturerId",
                b.name as manufacturer_display,
                c.scale_id as "scale_id: ScaleId",
                s.name as scale_display,
                c.category as "category: Category",
                c.power_method as "power_method: PowerMethod",
                c.epoch,
                c.description_de,
                c.description_en,
                c.description_fr,
                c.description_it,
                c.details_de,
                c.details_en,
                c.details_fr,
                c.details_it,
                c.delivery_date,
                c.availability_status as "availability_status: AvailabilityStatus",
                c.count,
                c.created_at,
                c.last_modified_at,
                c.version
            FROM catalog_items AS c
            JOIN manufacturers AS b
              ON c.manufacturer_id = b.manufacturer_id
            JOIN scales AS s
              ON s.scale_id = c.scale_id
            WHERE c.catalog_item_id = $1 "#,
            catalog_item_id as &CatalogItemId
        )
        .fetch_optional(&mut *unit_of_work.transaction)
        .await
        .context("A database failure was encountered while trying to fetch a catalog item.")?;

        result.to_output().map_err(DatabaseError::ConversionError)
    }
}

#[async_trait]
impl<'db> FindRollingStocksByCatalogItemIdRepository<'db, PgUnitOfWork<'db>> for CatalogItemsRepository {
    async fn find_rolling_stocks_by_id(
        &self,
        catalog_item_id: &CatalogItemId,
        unit_of_work: &mut PgUnitOfWork<'db>,
    ) -> Result<Vec<RollingStock>, DatabaseError> {
        let rolling_stocks = sqlx::query_as!(
            RollingStockRow,
            r#"SELECT 
                rs.rolling_stock_id as "rolling_stock_id: RollingStockId",
                rs.catalog_item_id as "catalog_item_id: CatalogItemId",
                rs.railway_id as "railway_id: RailwayId",
                r.name as railway_label, 
                rs.rolling_stock_category as "rolling_stock_category: RollingStockCategory",
                rs.livery,
                rs.length_over_buffers_mm,
                rs.length_over_buffers_in,
                rs.type_name,
                rs.road_number,
                rs.series,
                rs.depot,
                rs.dcc_interface as "dcc_interface: DccInterface",
                rs.control as "control: Control",
                rs.electric_multiple_unit_type as "electric_multiple_unit_type: ElectricMultipleUnitType",
                rs.freight_car_type as "freight_car_type: FreightCarType",
                rs.locomotive_type as "locomotive_type: LocomotiveType",
                rs.passenger_car_type as "passenger_car_type: PassengerCarType",
                rs.railcar_type as "railcar_type: RailcarType",
                rs.service_level as "service_level: ServiceLevel",
                rs.is_dummy,
                rs.minimum_radius,
                rs.coupling_socket as "coupling_socket: CouplingSocket",
                rs.close_couplers as "close_couplers: FeatureFlag",
                rs.digital_shunting_coupling as "digital_shunting_coupling: FeatureFlag",
                rs.flywheel_fitted as "flywheel_fitted: FeatureFlag",
                rs.body_shell as "body_shell: BodyShellType",
                rs.chassis as "chassis: ChassisType",
                rs.interior_lights as "interior_lights: FeatureFlag",
                rs.lights as "lights: FeatureFlag",
                rs.sprung_buffers as "sprung_buffers: FeatureFlag"
            FROM rolling_stocks AS rs
            JOIN railways AS r
              ON r.railway_id = rs.railway_id
            WHERE rs.catalog_item_id = $1"#,
            catalog_item_id as &CatalogItemId
        )
        .fetch_all(&mut *unit_of_work.transaction)
        .await
        .expect("A database failure was encountered while trying to fetch the rolling stock(s).");

        rolling_stocks.to_output().map_err(DatabaseError::ConversionError)
    }
}
