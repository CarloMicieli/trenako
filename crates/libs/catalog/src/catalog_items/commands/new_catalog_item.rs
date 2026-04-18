use crate::manufacturers::manufacturer_id::ManufacturerId;
use crate::catalog_items::availability_status::AvailabilityStatus;
use crate::catalog_items::catalog_item_id::CatalogItemId;
use crate::catalog_items::catalog_item_request::CatalogItemRequest;
use crate::catalog_items::catalog_item_response::CatalogItemCreated;
use crate::catalog_items::category::{
    Category, ElectricMultipleUnitType, FreightCarType, LocomotiveType, PassengerCarType, RailcarType,
    RollingStockCategory,
};
use crate::catalog_items::commands::repositories::{NewCatalogItemRepository, NewRollingStockRepository};
use crate::catalog_items::control::{Control, DccInterface};
use crate::catalog_items::delivery_date::DeliveryDate;
use crate::catalog_items::epoch::Epoch;
use crate::catalog_items::item_number::ItemNumber;
use crate::catalog_items::power_method::PowerMethod;
use crate::catalog_items::rolling_stock_id::RollingStockId;
use crate::catalog_items::rolling_stock_request::RollingStockRequest;
use crate::catalog_items::service_level::ServiceLevel;
use crate::catalog_items::technical_specifications::{
    BodyShellType, ChassisType, Coupling, CouplingSocket, FeatureFlag, Radius,
};
use crate::railways::railway_id::RailwayId;
use crate::scales::scale_id::ScaleId;
use chrono::Utc;
use common::length::Length;
use common::localized_text::LocalizedText;
use common::metadata::Metadata;
use common::queries::errors::DatabaseError;
use common::unit_of_work::{Database, UnitOfWork};
use std::result;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

pub type Result<R> = result::Result<R, CatalogItemCreationError>;

pub async fn create_new_catalog_item<'db, U, R, RR, DB>(
    request: CatalogItemRequest,
    repo: R,
    rs_repo: RR,
    db: DB,
) -> Result<CatalogItemCreated>
where
    U: UnitOfWork<'db>,
    R: NewCatalogItemRepository<'db, U>,
    RR: NewRollingStockRepository<'db, U>,
    DB: Database<'db, U>,
{
    let manufacturer_id = ManufacturerId::new(&request.manufacturer);
    let scale_id = ScaleId::new(&request.scale);
    let catalog_item_id = CatalogItemId::of(&manufacturer_id, &request.item_number);

    let mut unit_of_work = db.begin().await?;

    if repo.exists(&catalog_item_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::CatalogItemAlreadyExists(catalog_item_id));
    }

    if !repo.manufacturer_exists(&manufacturer_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::ManufacturerNotFound(manufacturer_id));
    }

    if !repo.scale_exists(&scale_id, &mut unit_of_work).await? {
        return Err(CatalogItemCreationError::ScaleNotFound(scale_id));
    }

    let command = NewCatalogItemCommand::try_from(request)?;

    repo.insert(&command, &mut unit_of_work).await?;

    for rs in command.rolling_stocks {
        if !rs_repo.railway_exists(&rs.railway_id, &mut unit_of_work).await? {
            return Err(CatalogItemCreationError::RailwayNotFound(rs.railway_id));
        }
        rs_repo.insert(&rs, &mut unit_of_work).await?;
    }

    unit_of_work.commit().await?;

    Ok(CatalogItemCreated {
        catalog_item_id,
        created_at: *command.metadata.created(),
    })
}

#[derive(Debug, Error)]
pub enum CatalogItemCreationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("The catalog item request is not valid")]
    InvalidRequest(ValidationErrors),

    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error("The catalog item already exists (id: {0})")]
    CatalogItemAlreadyExists(CatalogItemId),

    #[error("Unable to create the catalog item due to manufacturer not found (id: {0})")]
    ManufacturerNotFound(ManufacturerId),

    #[error("Unable to create the catalog item due to railway not found (id: {0})")]
    RailwayNotFound(RailwayId),

    #[error("Unable to create the catalog item due to scale not found (id: {0})")]
    ScaleNotFound(ScaleId),
}

#[derive(Debug, Clone)]
pub struct NewCatalogItemCommand {
    pub catalog_item_id: CatalogItemId,
    pub payload: CatalogItemCommandPayload,
    pub rolling_stocks: Vec<NewRollingStockCommand>,
    pub metadata: Metadata,
}

impl TryFrom<CatalogItemRequest> for NewCatalogItemCommand {
    type Error = CatalogItemCreationError;

    fn try_from(value: CatalogItemRequest) -> result::Result<Self, Self::Error> {
        validate_request(&value)?;
        let manufacturer_id = ManufacturerId::new(&value.manufacturer);
        let catalog_item_id = CatalogItemId::of(&manufacturer_id, &value.item_number);

        let rolling_stocks: Vec<NewRollingStockCommand> = value
            .rolling_stocks
            .clone()
            .into_iter()
            .map(|rs| NewRollingStockCommand::new(&catalog_item_id, rs).unwrap())
            .collect();
        let payload = CatalogItemCommandPayload::try_from(value)?;
        let metadata = Metadata::created_at(Utc::now());

        Ok(NewCatalogItemCommand {
            catalog_item_id,
            payload,
            rolling_stocks,
            metadata,
        })
    }
}

fn validate_request(request: &CatalogItemRequest) -> result::Result<(), CatalogItemCreationError> {
    request.validate().map_err(CatalogItemCreationError::InvalidRequest)
}

#[derive(Debug, Clone)]
pub struct CatalogItemCommandPayload {
    pub manufacturer_id: ManufacturerId,
    pub item_number: ItemNumber,
    pub scale_id: ScaleId,
    pub category: Category,
    pub description: LocalizedText,
    pub details: LocalizedText,
    pub power_method: PowerMethod,
    pub epoch: Epoch,
    pub delivery_date: Option<DeliveryDate>,
    pub availability_status: Option<AvailabilityStatus>,
    pub count: i32,
}

impl TryFrom<CatalogItemRequest> for CatalogItemCommandPayload {
    type Error = CatalogItemCreationError;

    fn try_from(request: CatalogItemRequest) -> result::Result<Self, Self::Error> {
        let manufacturer_id = ManufacturerId::new(&request.manufacturer);
        let scale_id = ScaleId::new(&request.scale);

        let payload = CatalogItemCommandPayload {
            manufacturer_id,
            item_number: request.item_number,
            scale_id,
            category: request.category,
            description: request.description,
            details: request.details,
            power_method: request.power_method,
            epoch: request.epoch,
            delivery_date: request.delivery_date,
            availability_status: request.availability_status,
            count: request.count,
        };
        Ok(payload)
    }
}

#[derive(Debug, Clone)]
pub struct NewRollingStockCommand {
    pub catalog_item_id: CatalogItemId,
    pub rolling_stock_id: RollingStockId,
    pub railway_id: RailwayId,
    pub payload: RollingStockPayload,
}

impl NewRollingStockCommand {
    pub fn new(catalog_item_id: &CatalogItemId, request: RollingStockRequest) -> Result<NewRollingStockCommand> {
        Ok(NewRollingStockCommand {
            catalog_item_id: catalog_item_id.clone(),
            rolling_stock_id: RollingStockId::new(),
            railway_id: RailwayId::new(request.railway()),
            payload: RollingStockPayload::try_from(request)?,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct RollingStockPayload {
    pub category: Option<RollingStockCategory>,
    pub livery: Option<String>,
    pub length_over_buffers_mm: Option<Length>,
    pub length_over_buffers_in: Option<Length>,
    pub type_name: Option<String>,
    pub road_number: Option<String>,
    pub series: Option<String>,
    pub depot: Option<String>,
    pub dcc_interface: Option<DccInterface>,
    pub control: Option<Control>,
    pub electric_multiple_unit_type: Option<ElectricMultipleUnitType>,
    pub freight_car_type: Option<FreightCarType>,
    pub locomotive_type: Option<LocomotiveType>,
    pub passenger_car_type: Option<PassengerCarType>,
    pub railcar_type: Option<RailcarType>,
    pub service_level: Option<ServiceLevel>,
    pub is_dummy: bool,
    pub minimum_radius: Option<Radius>,
    pub coupling_socket: Option<CouplingSocket>,
    pub close_couplers: Option<FeatureFlag>,
    pub digital_shunting_coupling: Option<FeatureFlag>,
    pub flywheel_fitted: Option<FeatureFlag>,
    pub body_shell: Option<BodyShellType>,
    pub chassis: Option<ChassisType>,
    pub interior_lights: Option<FeatureFlag>,
    pub lights: Option<FeatureFlag>,
    pub sprung_buffers: Option<FeatureFlag>,
}

impl TryFrom<RollingStockRequest> for RollingStockPayload {
    type Error = CatalogItemCreationError;

    fn try_from(request: RollingStockRequest) -> result::Result<Self, Self::Error> {
        let category = request.category();
        let (minimum_radius, coupling, flywheel_fitted, body_shell, chassis, interior_lights, lights, sprung_buffers) =
            if let Some(ts) = request.technical_specifications() {
                (
                    ts.minimum_radius,
                    ts.coupling,
                    ts.flywheel_fitted,
                    ts.body_shell,
                    ts.chassis,
                    ts.interior_lights,
                    ts.lights,
                    ts.sprung_buffers,
                )
            } else {
                (None, None, None, None, None, None, None, None)
            };

        let Coupling {
            socket,
            close_couplers,
            digital_shunting,
        } = coupling.unwrap_or_default();

        let (millimeters, inches) = if let Some(length_over_buffers) = request.length_over_buffers() {
            (length_over_buffers.millimeters, length_over_buffers.inches)
        } else {
            (None, None)
        };

        match request {
            RollingStockRequest::ElectricMultipleUnitRequest {
                railway: _,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                depot,
                electric_multiple_unit_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                depot,
                dcc_interface,
                control,
                electric_multiple_unit_type: Some(electric_multiple_unit_type),
                is_dummy,
                minimum_radius,
                coupling_socket: socket,
                close_couplers,
                digital_shunting_coupling: digital_shunting,
                flywheel_fitted,
                body_shell,
                chassis,
                interior_lights,
                lights,
                sprung_buffers,
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::RailcarRequest {
                railway: _,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                depot,
                railcar_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                depot,
                dcc_interface,
                control,
                railcar_type: Some(railcar_type),
                is_dummy,
                minimum_radius,
                coupling_socket: socket,
                close_couplers,
                digital_shunting_coupling: digital_shunting,
                flywheel_fitted,
                body_shell,
                chassis,
                interior_lights,
                lights,
                sprung_buffers,
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::LocomotiveRequest {
                railway: _,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                class_name,
                road_number,
                series,
                depot,
                locomotive_type,
                dcc_interface,
                control,
                is_dummy,
            } => Ok(RollingStockPayload {
                category: Some(category),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(class_name),
                road_number: Some(road_number),
                series,
                depot,
                dcc_interface,
                control,
                locomotive_type: Some(locomotive_type),
                is_dummy,
                minimum_radius,
                coupling_socket: socket,
                close_couplers,
                digital_shunting_coupling: digital_shunting,
                flywheel_fitted,
                body_shell,
                chassis,
                interior_lights,
                lights,
                sprung_buffers,
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::PassengerCarRequest {
                railway: _,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                series,
                passenger_car_type,
                service_level,
            } => Ok(RollingStockPayload {
                category: Some(category),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                series,
                passenger_car_type,
                service_level,
                minimum_radius,
                coupling_socket: socket,
                close_couplers,
                digital_shunting_coupling: digital_shunting,
                flywheel_fitted,
                body_shell,
                chassis,
                interior_lights,
                lights,
                sprung_buffers,
                ..RollingStockPayload::default()
            }),
            RollingStockRequest::FreightCarRequest {
                railway: _,
                livery,
                length_over_buffers: _,
                technical_specifications: _,
                type_name,
                road_number,
                freight_car_type,
            } => Ok(RollingStockPayload {
                category: Some(category),
                livery,
                length_over_buffers_mm: millimeters,
                length_over_buffers_in: inches,
                type_name: Some(type_name),
                road_number,
                freight_car_type,
                minimum_radius,
                coupling_socket: socket,
                close_couplers,
                digital_shunting_coupling: digital_shunting,
                flywheel_fitted,
                body_shell,
                chassis,
                interior_lights,
                lights,
                sprung_buffers,
                ..RollingStockPayload::default()
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::localized_text::LocalizedText;

    mod new_catalog_item_command {
        use crate::manufacturers::manufacturer_id::ManufacturerId;
        use crate::catalog_items::catalog_item_id::CatalogItemId;
        use crate::catalog_items::commands::new_catalog_item::test::{catalog_item, new_catalog_item};
        use crate::catalog_items::commands::new_catalog_item::{CatalogItemCreationError, create_new_catalog_item};
        use crate::catalog_items::commands::repositories::in_memory::{
            InMemoryCatalogItemRepository, InMemoryRollingStockRepository,
        };
        use crate::catalog_items::item_number::ItemNumber;
        use crate::railways::railway_id::RailwayId;
        use crate::scales::scale_id::ScaleId;
        use common::unit_of_work::noop::NoOpDatabase;

        #[tokio::test]
        async fn it_should_return_an_error_when_the_manufacturer_is_not_found() {
            let repo = InMemoryCatalogItemRepository::empty();
            let rr_repo = InMemoryRollingStockRepository::empty();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::ManufacturerNotFound(manufacturer)) => assert_eq!(ManufacturerId::new("ACME"), manufacturer),
                _ => panic!("CatalogItemCreationError::ManufacturerNotFound is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_scale_is_not_found() {
            let repo = InMemoryCatalogItemRepository::empty().with_manufacturer(ManufacturerId::new("ACME"));
            let rr_repo = InMemoryRollingStockRepository::empty();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::ScaleNotFound(scale)) => assert_eq!(ScaleId::new("H0"), scale),
                _ => panic!("CatalogItemCreationError::ScaleNotFound is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_catalog_item_already_exists() {
            let repo =
                InMemoryCatalogItemRepository::with(catalog_item(ManufacturerId::new("ACME"), ItemNumber::new("123456")));
            let rr_repo = InMemoryRollingStockRepository::empty();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());

            match result {
                Err(CatalogItemCreationError::CatalogItemAlreadyExists(catalog_item_id)) => {
                    assert_eq!("acme-123456", catalog_item_id.to_string())
                }
                _ => panic!("CatalogItemCreationError::CatalogItemAlreadyExists is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_return_an_error_when_the_railway_is_not_found() {
            let repo = InMemoryCatalogItemRepository::empty()
                .with_manufacturer(ManufacturerId::new("ACME"))
                .with_scale(ScaleId::new("H0"));
            let rr_repo = InMemoryRollingStockRepository::empty();
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_err());
            match result {
                Err(CatalogItemCreationError::RailwayNotFound(railway_id)) => {
                    assert_eq!("fs", railway_id.to_string())
                }
                _ => panic!("CatalogItemCreationError::RailwayNotFound is expected (found: {result:?})"),
            }
        }

        #[tokio::test]
        async fn it_should_create_a_new_catalog_item() {
            let repo = InMemoryCatalogItemRepository::empty()
                .with_manufacturer(ManufacturerId::new("ACME"))
                .with_scale(ScaleId::new("H0"));
            let rr_repo = InMemoryRollingStockRepository::empty().with_railway(RailwayId::new("FS"));
            let db = NoOpDatabase;

            let request = new_catalog_item();
            let result = create_new_catalog_item(request, repo, rr_repo, db).await;

            assert!(result.is_ok());
            assert_eq!(
                CatalogItemId::of(&ManufacturerId::new("ACME"), &ItemNumber::new("123456")),
                result.unwrap().catalog_item_id
            );
        }
    }

    fn new_catalog_item() -> CatalogItemRequest {
        CatalogItemRequest {
            manufacturer: "ACME".to_string(),
            item_number: ItemNumber::new("123456"),
            scale: "H0".to_string(),
            category: Category::Locomotives,
            power_method: PowerMethod::DC,
            epoch: Epoch::IV,
            description: LocalizedText::with_italian("Descrizione"),
            details: LocalizedText::with_italian("Dettagli"),
            delivery_date: Some(DeliveryDate::ByYear(2022)),
            availability_status: Some(AvailabilityStatus::Available),
            rolling_stocks: vec![locomotive_request()],
            count: 1,
        }
    }

    fn locomotive_request() -> RollingStockRequest {
        RollingStockRequest::LocomotiveRequest {
            railway: "FS".to_string(),
            livery: None,
            length_over_buffers: None,
            technical_specifications: None,
            class_name: "E656".to_string(),
            road_number: "E656 077".to_string(),
            series: None,
            depot: None,
            locomotive_type: LocomotiveType::ElectricLocomotive,
            dcc_interface: Some(DccInterface::Nem652),
            control: Some(Control::DccReady),
            is_dummy: false,
        }
    }

    fn catalog_item(manufacturer_id: ManufacturerId, item_number: ItemNumber) -> NewCatalogItemCommand {
        NewCatalogItemCommand {
            catalog_item_id: CatalogItemId::of(&manufacturer_id, &item_number),
            payload: CatalogItemCommandPayload {
                manufacturer_id,
                item_number,
                scale_id: ScaleId::new("H0"),
                category: Category::Locomotives,
                description: LocalizedText::with_italian("Descrizione"),
                details: LocalizedText::with_italian("Dettagli"),
                power_method: PowerMethod::DC,
                epoch: Epoch::IV,
                delivery_date: Some(DeliveryDate::ByYear(2022)),
                availability_status: Some(AvailabilityStatus::Available),
                count: 1,
            },
            rolling_stocks: Vec::new(),
            metadata: Metadata::created_at(Utc::now()),
        }
    }
}
