use crate::web::problem::ProblemDetail;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

pub trait ToProblemDetail {
    /// Convert this value to a problem detail with the given `request_id`
    fn to_problem_detail(self, request_id: Uuid, path: Option<&str>) -> ProblemDetail;
}

pub struct Created {
    location: HeaderValue,
}

impl Created {
    pub fn with_location(location_url: &str) -> Self {
        let location: HeaderValue = location_url.parse().unwrap();
        Created { location }
    }
}

impl IntoResponse for Created {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, [(header::LOCATION, self.location)], ()).into_response()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod created_test {
        use super::*;

        #[test]
        fn it_should_create_created_responses() {
            let created = Created::with_location("http://localhost");

            let response = created.into_response();
            assert_eq!(StatusCode::CREATED, response.status());
            assert_eq!(
                HeaderValue::from_str("http://localhost").unwrap(),
                response.headers().get(header::LOCATION).unwrap()
            )
        }
    }
}
