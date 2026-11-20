use crate::catalog::railways::routes;
use crate::hateoas::representations::EntityModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Path, State};
use catalog::railways::queries::find_railway_by_id::find_railway_by_id;
use catalog::railways::railway::Railway;
use catalog::railways::railway_id::RailwayId;
use data::catalog::railways::repositories::RailwaysRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_railway_by_id", skip(app_state))]
pub async fn handle(
    Path(railway_id): Path<RailwayId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Railway>, Box<ProblemDetail>> {
    let database = app_state.get_database();
    let repo = RailwaysRepository;

    let result = find_railway_by_id(&railway_id, repo, database).await;
    result
        .map(|railway| EntityModel::of(railway, Vec::new()))
        .map_err(|why| {
            let path = format!("{}/{}", routes::RAILWAY_ROOT_API, railway_id);
            Box::new(why.to_problem_detail(Uuid::new_v4(), Some(&path)))
        })
}
