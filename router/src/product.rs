use actix_web::web;
use actix_web_grants::PermissionGuard;

use error::permission_denied_error;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // domain includes: /products/{product_id}/parts/{part_id}
    cfg.service(
      web::scope("/products")
          .service(
              web::resource("")
                  .route(web::get().to(products::get_products))
                  .route(web::post().to(handler::product::add)),
          )
          .service(
              web::scope("/{product_id}")
                  .service(
                      web::resource("")
                          .route(web::get().to(products::get_product_detail))
                          .route(web::delete().to(products::remove_product)),
                  )
                  .service(
                      web::scope("/parts")
                          .service(
                              web::resource("")
                                  .route(web::get().to(parts::get_parts))
                                  .route(web::post().to(parts::add_part)),
                          )
                          .service(
                              web::resource("/{part_id}")
                                  .route(web::get().to(parts::get_part_detail))
                                  .route(web::delete().to(parts::remove_part)),
                          ),
                  ),
          ),
  );
}
