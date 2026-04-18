use crate::catalog::manufacturers::routes;
use crate::hateoas::representations::CollectionModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Query, State};
use catalog::manufacturers::manufacturer::Manufacturer;
use catalog::manufacturers::queries::find_all_manufacturers::find_all_manufacturers;
use common::queries::pagination::PageRequest;
use data::catalog::manufacturers::repositories::ManufacturersRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_all_manufacturers", skip(app_state))]
pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Manufacturer>, ProblemDetail> {
    let database = app_state.get_database();
    let repo = ManufacturersRepository;

    let results = find_all_manufacturers(repo, database).await;
    results
        .map(|manufacturers| CollectionModel::of(manufacturers, Vec::new()))
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), Some(routes::MANUFACTURERS_ROOT_API)))
}
