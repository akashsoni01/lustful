use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{info, warn};

use error::AppResult;
use model::response::MessageResponse;
use model::*;
use state::AppState;
use util::claim::UserClaimsRequest;

/// register new user
#[utoipa::path(
    post,
    request_body = AddProductRequest,
    path = "/api/v1/products/add_product",
    responses(
        (status = 201, description = "success added product", body = [AddProductResponse]),
        (status = 400, description = "invalid data input", body = [AppResponseError]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
)]
pub async fn add(
  state: web::Data<AppState>,
  web::Json(req): web::Json<AddProductRequest>,
) -> AppResult<HttpResponse> {
  info!("register user with request: {req:?}");
  match service::product::addProduct(state, req).await {
    Ok((product_id, resp)) => {
      info!("success added product: {product_id}");
      Ok(HttpResponse::Created().json(resp))
    }
    Err(e) => {
      warn!("unsuccessfully added product: {e:?}");
      Err(e)
    }
  }
}
