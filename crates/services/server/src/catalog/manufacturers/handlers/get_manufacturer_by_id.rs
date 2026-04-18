use crate::catalog::manufacturers::routes;
use crate::hateoas::representations::EntityModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Path, State};
use catalog::manufacturers::manufacturer::Manufacturer;
use catalog::manufacturers::manufacturer_id::ManufacturerId;
use catalog::manufacturers::queries::find_manufacturer_by_id::find_manufacturer_by_id;
use data::catalog::manufacturers::repositories::ManufacturersRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_manufacturer_by_id", skip(app_state))]
pub async fn handle(
    Path(manufacturer_id): Path<ManufacturerId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Manufacturer>, ProblemDetail> {
    let database = app_state.get_database();
    let repo = ManufacturersRepository;

    let result = find_manufacturer_by_id(&manufacturer_id, repo, database).await;
    result
        .map(|manufacturer| EntityModel::of(manufacturer, vec![]))
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), Some(routes::MANUFACTURERS_ROOT_API)))
}
