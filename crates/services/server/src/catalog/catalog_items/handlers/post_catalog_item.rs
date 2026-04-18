use crate::catalog::catalog_items::routes::CATALOG_ITEMS_ROOT_API;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::{Created, ToProblemDetail};
use axum::Json;
use axum::extract::State;
use catalog::catalog_items::catalog_item_request::CatalogItemRequest;
use catalog::catalog_items::commands::new_catalog_item::{CatalogItemCreationError, create_new_catalog_item};
use data::catalog::catalog_item::repositories::{CatalogItemsRepository, RollingStocksRepository};
use uuid::Uuid;

#[tracing::instrument(name = "create_new_catalog_item", skip(app_state))]
pub async fn handle(
    State(app_state): State<AppState>,
    Json(request): Json<CatalogItemRequest>,
) -> Result<Created, ProblemDetail> {
    let repo = CatalogItemsRepository;
    let rr_repo = RollingStocksRepository;
    let database = app_state.get_database();

    let result = create_new_catalog_item(request, repo, rr_repo, database).await;
    result
        .map(|created| {
            let location = format!("{}/{}", CATALOG_ITEMS_ROOT_API, created.catalog_item_id);
            Created::with_location(&location)
        })
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), None))
}

impl ToProblemDetail for CatalogItemCreationError {
    fn to_problem_detail(self, request_id: Uuid, _path: Option<&str>) -> ProblemDetail {
        match self {
            CatalogItemCreationError::ManufacturerNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::CatalogItemAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            CatalogItemCreationError::RailwayNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::ScaleNotFound(_) => {
                ProblemDetail::unprocessable_entity(request_id, &self.to_string())
            }
            CatalogItemCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            CatalogItemCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            CatalogItemCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod catalog_item_creation_error_to_problem_detail {
        use super::*;
        use anyhow::anyhow;
        use axum::http::StatusCode;
        use catalog::manufacturers::manufacturer_id::ManufacturerId;
        use catalog::catalog_items::catalog_item_id::CatalogItemId;
        use catalog::catalog_items::item_number::ItemNumber;
        use catalog::railways::railway_id::RailwayId;
        use catalog::scales::scale_id::ScaleId;
        use common::queries::errors::DatabaseError;
        use common::trn::Trn;
        use pretty_assertions::assert_eq;
        use validator::ValidationErrors;

        #[test]
        fn it_should_return_conflict_when_the_catalog_item_already_exists() {
            let catalog_item_id = CatalogItemId::of(&ManufacturerId::new("acme"), &ItemNumber::new("12345"));
            let error = CatalogItemCreationError::CatalogItemAlreadyExists(catalog_item_id);

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::CONFLICT, problem_detail.status);
            assert_eq!("https://httpstatuses.com/409", problem_detail.problem_type.as_str());
            assert_eq!(
                "The catalog item already exists (id: acme-12345)",
                problem_detail.detail
            );
            assert_eq!("The resource already exists", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_unprocessable_entity_when_the_manufacturer_was_not_found() {
            let error = CatalogItemCreationError::ManufacturerNotFound(ManufacturerId::new("acme"));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, problem_detail.status);
            assert_eq!("https://httpstatuses.com/422", problem_detail.problem_type.as_str());
            assert_eq!(
                "Unable to create the catalog item due to manufacturer not found (id: acme)",
                problem_detail.detail
            );
            assert_eq!("Unprocessable entity", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_unprocessable_entity_when_the_railway_was_not_found() {
            let error = CatalogItemCreationError::RailwayNotFound(RailwayId::new("fs"));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, problem_detail.status);
            assert_eq!("https://httpstatuses.com/422", problem_detail.problem_type.as_str());
            assert_eq!(
                "Unable to create the catalog item due to railway not found (id: fs)",
                problem_detail.detail
            );
            assert_eq!("Unprocessable entity", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_unprocessable_entity_when_the_scale_was_not_found() {
            let error = CatalogItemCreationError::ScaleNotFound(ScaleId::new("h0"));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, problem_detail.status);
            assert_eq!("https://httpstatuses.com/422", problem_detail.problem_type.as_str());
            assert_eq!(
                "Unable to create the catalog item due to scale not found (id: h0)",
                problem_detail.detail
            );
            assert_eq!("Unprocessable entity", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_bad_request_for_invalid_request() {
            let error = CatalogItemCreationError::InvalidRequest(ValidationErrors::new());

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::BAD_REQUEST, problem_detail.status);
            assert_eq!("https://httpstatuses.com/400", problem_detail.problem_type.as_str());
            assert_eq!("", problem_detail.detail);
            assert_eq!("Bad request", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_an_internal_server_error_for_generic_errors() {
            let error = CatalogItemCreationError::UnexpectedError(anyhow!("Something bad just happened"));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, problem_detail.status);
            assert_eq!("https://httpstatuses.com/500", problem_detail.problem_type.as_str());
            assert_eq!("Something bad just happened", problem_detail.detail);
            assert_eq!("Error: Internal Server Error", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_an_internal_server_error_for_database_errors() {
            let error = CatalogItemCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
                "Something bad just happened"
            )));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, problem_detail.status);
            assert_eq!("https://httpstatuses.com/500", problem_detail.problem_type.as_str());
            assert_eq!("Something bad just happened", problem_detail.detail);
            assert_eq!("Error: Internal Server Error", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }
    }
}
