use crate::catalog::railways::routes;
use crate::hateoas::representations::CollectionModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Query, State};
use catalog::railways::queries::find_all_railways::find_all_railways;
use catalog::railways::railway::Railway;
use common::queries::pagination::PageRequest;
use data::catalog::railways::repositories::RailwaysRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_all_railways", skip(app_state))]
pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Railway>, Box<ProblemDetail>> {
    let database = app_state.get_database();
    let repo = RailwaysRepository;

    let results = find_all_railways(repo, database).await;
    results
        .map(|railways| CollectionModel::of(railways, Vec::new()))
        .map_err(|why| Box::new(why.to_problem_detail(Uuid::new_v4(), Some(routes::RAILWAY_ROOT_API))))
}
