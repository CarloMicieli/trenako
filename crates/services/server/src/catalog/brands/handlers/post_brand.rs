use crate::catalog::brands::routes::BRANDS_ROOT_API;
use crate::state::AppState;
use crate::web::problem::ProblemDetail;
use crate::web::responders::{Created, ToProblemDetail};
use axum::Json;
use axum::extract::State;
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::commands::new_brand::{BrandCreationError, create_new_brand};
use data::catalog::brands::repositories::BrandsRepository;
use uuid::Uuid;

#[tracing::instrument(name = "create_new_brand", skip(app_state))]
pub async fn handle(
    State(app_state): State<AppState>,
    Json(request): Json<BrandRequest>,
) -> Result<Created, ProblemDetail> {
    let repo = BrandsRepository;
    let database = app_state.get_database();

    let result = create_new_brand(request, repo, database).await;
    result
        .map(|created| {
            let location = format!("{}/{}", BRANDS_ROOT_API, created.brand_id);
            Created::with_location(&location)
        })
        .map_err(|why| why.to_problem_detail(Uuid::new_v4(), None))
}

impl ToProblemDetail for BrandCreationError {
    fn to_problem_detail(self, request_id: Uuid, _path: Option<&str>) -> ProblemDetail {
        match self {
            BrandCreationError::BrandAlreadyExists(_) => {
                ProblemDetail::resource_already_exists(request_id, &self.to_string())
            }
            BrandCreationError::UnexpectedError(why) => ProblemDetail::error(request_id, &why.to_string()),
            BrandCreationError::DatabaseError(why) => ProblemDetail::error(request_id, &why.to_string()),
            BrandCreationError::InvalidRequest(_) => ProblemDetail::bad_request(request_id, ""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod brand_creation_error_to_problem_detail {
        use super::*;
        use anyhow::anyhow;
        use axum::http::StatusCode;
        use catalog::brands::brand_id::BrandId;
        use common::queries::errors::DatabaseError;
        use common::trn::Trn;
        use pretty_assertions::assert_eq;
        use validator::ValidationErrors;

        #[test]
        fn it_should_return_conflict_when_the_railway_already_exists() {
            let err = BrandCreationError::BrandAlreadyExists(BrandId::new("ACME"));

            let id = Uuid::new_v4();
            let problem_detail = err.to_problem_detail(id, None);
            assert_eq!(StatusCode::CONFLICT, problem_detail.status);
            assert_eq!("https://httpstatuses.com/409", problem_detail.problem_type.as_str());
            assert_eq!("The brand already exists (id: acme)", problem_detail.detail);
            assert_eq!("The resource already exists", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_bad_request_for_invalid_request() {
            let err = BrandCreationError::InvalidRequest(ValidationErrors::new());

            let id = Uuid::new_v4();
            let problem_detail = err.to_problem_detail(id, None);
            assert_eq!(StatusCode::BAD_REQUEST, problem_detail.status);
            assert_eq!("https://httpstatuses.com/400", problem_detail.problem_type.as_str());
            assert_eq!("", problem_detail.detail);
            assert_eq!("Bad request", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_an_internal_server_error_for_generic_errors() {
            let err = BrandCreationError::UnexpectedError(anyhow!("Something bad just happened"));

            let id = Uuid::new_v4();
            let problem_detail = err.to_problem_detail(id, None);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, problem_detail.status);
            assert_eq!("https://httpstatuses.com/500", problem_detail.problem_type.as_str());
            assert_eq!("Something bad just happened", problem_detail.detail);
            assert_eq!("Error: Internal Server Error", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }

        #[test]
        fn it_should_return_an_internal_server_error_for_database_errors() {
            let err = BrandCreationError::DatabaseError(DatabaseError::UnexpectedError(anyhow!(
                "Something bad just happened"
            )));

            let id = Uuid::new_v4();
            let problem_detail = err.to_problem_detail(id, None);
            assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, problem_detail.status);
            assert_eq!("https://httpstatuses.com/500", problem_detail.problem_type.as_str());
            assert_eq!("Something bad just happened", problem_detail.detail);
            assert_eq!("Error: Internal Server Error", problem_detail.title);
            assert_eq!(Trn::instance(&id), problem_detail.instance);
        }
    }
}
