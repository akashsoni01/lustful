use actix_web::web;
use actix_web_grants::PermissionGuard;

use error::permission_denied_error;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route("/product/add_product", web::post().to(handler::product::add));
}
