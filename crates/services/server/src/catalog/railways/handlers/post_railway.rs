use crate::catalog::railways::routes::RAILWAY_ROOT_API;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::{Created, ToProblemDetail};
use axum::Json;
use axum::extract::State;
use catalog::railways::commands::new_railways::{RailwayCreationError, create_new_railway};
use catalog::railways::railway_request::RailwayRequest;
use data::catalog::railways::repositories::RailwaysRepository;
use uuid::Uuid;

#[tracing::instrument(name = "create_railway", skip(app_state))]
pub async fn handle(
    State(app_state): State<AppState>,
    Json(request): Json<RailwayRequest>,
) -> Result<Created, ProblemDetail> {
    let repo = RailwaysRepository;
    let database = app_state.get_database();

    let result = create_new_railway(request, repo, database).await;
    result
        .map(|created| {
            let location = format!("{}/{}", RAILWAY_ROOT_API, created.railway_id);
            Created::with_location(&location)
        })
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), None))
}

impl ToProblemDetail for RailwayCreationError {
    fn to_problem_detail(self, request_id: Uuid, _path: Option<&str>) -> ProblemDetail {
        match self {
            RailwayCreationError::RailwayAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            RailwayCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            RailwayCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            RailwayCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod railway_creation_error_to_problem_detail {
        use super::*;
        use anyhow::anyhow;
        use axum::http::StatusCode;
        use catalog::railways::railway_id::RailwayId;
        use common::queries::errors::DatabaseError;
        use common::trn::Trn;
        use pretty_assertions::assert_eq;
        use validator::ValidationErrors;

        #[test]
        fn it_should_return_conflict_when_the_railway_already_exists() {
            let error = RailwayCreationError::RailwayAlreadyExists(RailwayId::new("FS"));

            let id = Uuid::new_v4();
            let problem_detail = error.to_problem_detail(id, None);
            assert_eq!(StatusCode::CONFLICT, problem_detail.status);
            assert_eq!("https://httpstatuses.com/409", problem_detail.problem_type.as_str());
            assert_eq!("The railway already exists (id: fs)", problem_detail.detail);
            assert_eq!("The resource already exists", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_bad_request_for_invalid_request() {
            let error = RailwayCreationError::InvalidRequest(ValidationErrors::new());

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
            let error = RailwayCreationError::UnexpectedError(anyhow!("Something bad just happened"));

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
            let error = RailwayCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
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
