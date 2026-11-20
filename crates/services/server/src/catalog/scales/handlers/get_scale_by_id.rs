use crate::catalog::scales::routes;
use crate::hateoas::representations::EntityModel;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::ToProblemDetail;
use axum::extract::{Path, State};
use catalog::scales::queries::find_scale_by_id::find_scale_by_id;
use catalog::scales::scale::Scale;
use catalog::scales::scale_id::ScaleId;
use data::catalog::scales::repositories::ScalesRepository;
use uuid::Uuid;

#[tracing::instrument(name = "get_scale_by_id", skip(app_state))]
pub async fn handle(
    Path(scale_id): Path<ScaleId>,
    State(app_state): State<AppState>,
) -> Result<EntityModel<Scale>, Box<ProblemDetail>> {
    let database = app_state.get_database();
    let repo = ScalesRepository;

    let result = find_scale_by_id(&scale_id, repo, database).await;
    result.map(|scale| EntityModel::of(scale, Vec::new())).map_err(|why| {
        let path = format!("{}/{}", routes::SCALE_ROOT_API, scale_id);
        Box::new(why.to_problem_detail(Uuid::new_v4(), Some(&path)))
    })
}
