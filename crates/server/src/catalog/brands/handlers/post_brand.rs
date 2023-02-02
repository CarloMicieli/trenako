use crate::catalog::brands::repositories::PgNewBrandRepository;
use crate::catalog::brands::routes::BRAND_ROOT_API;
use crate::web::problem_detail::ProblemDetail;
use crate::web::responders::ToCreated;
use actix_web::{web, HttpResponse, Responder};
use catalog::brands::brand_request::BrandRequest;
use catalog::brands::brand_response::BrandCreated;
use catalog::brands::commands::new_brand::{create_new_brand, BrandCreationError};
use common::unit_of_work::postgres::PgDatabase;
use sqlx::PgPool;
use tracing_actix_web::RequestId;

pub async fn handle(
    request_id: RequestId,
    request: web::Json<BrandRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let repo = PgNewBrandRepository;
    let database = PgDatabase::new(&db_pool);

    let result = create_new_brand(request.0, repo, database).await;
    match result {
        Ok(created) => created.to_created(),
        Err(why) => match why {
            BrandCreationError::BrandAlreadyExists(_) => HttpResponse::Conflict().finish(),
            _ => {
                tracing::error!("{:?}", why);
                ProblemDetail::error(*request_id, &why.to_string()).to_response()
            }
        },
    }
}

impl ToCreated for BrandCreated {
    fn location(&self) -> String {
        format!("{}/{}", BRAND_ROOT_API, self.brand_id)
    }
}
