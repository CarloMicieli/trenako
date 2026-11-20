use crate::catalog::scales::routes;
use crate::hateoas::representations::CollectionModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Query, State};
use catalog::scales::queries::find_all_scales::find_all_scales;
use catalog::scales::scale::Scale;
use common::queries::pagination::PageRequest;
use data::catalog::scales::repositories::ScalesRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_all_scales", skip(app_state))]
pub async fn handle(
    Query(_page_request): Query<PageRequest>,
    State(app_state): State<AppState>,
) -> Result<CollectionModel<Scale>, Box<ProblemDetail>> {
    let database = app_state.get_database();
    let repo = ScalesRepository;

    let results = find_all_scales(repo, database).await;
    results
        .map(|scales| CollectionModel::of(scales, Vec::new()))
        .map_err(|why| Box::new(why.to_problem_detail(Uuid::new_v4(), Some(routes::SCALE_ROOT_API))))
}
