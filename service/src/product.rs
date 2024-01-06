use actix_web::web;
use client::postgres::PgClient;
use sqlx::{Postgres, Transaction};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use crate::redis::{
  BlockEmailKey, ForgetPasswordKey, InvitationKey, RedisKey, SessionKey, TwoFactorLoginKey,
};
use crate::session;
use crate::token::{self, *};
use entity::product::Product;
use error::invalid_input_error;
use error::{AppError, AppResult};
use model::Template;
use model::{request::*, response::*};
use state::AppState;
use util::{claim::UserClaims, hash};

pub async fn addProduct(
  state: web::Data<AppState>,
  req: AddProductRequest,
) -> AppResult<(Uuid, AddProductResponse)> {
  info!("register user req: {req:?}");
  req.validate()?;
  // query::product::delete_all_inative_product()
    // .execute(&state.postgres)
    // .await?;
check_unique_product_id(&state.postgres, &req.product_id).await?;
  let mut product = Product {
    id: Uuid::new_v4(),
    title: req.title,
    product_id: req.product_id,
    is_active: true,
    create_at: None,
    update_at: None,
  };
  let product_id = query::get_transaction(&state.postgres, move |mut tx| async move {
    query::product::save(&product).execute(&mut *tx).await?;
    Ok(((product.id), tx))
  })
  .await?;
  Ok((
    product_id,
    AddProductResponse::new(product_id),
  ))
}


pub async fn check_unique_product_id(
    db: &PgClient,
    product_id: &str,
  ) -> AppResult<()> {
    if query::product::exist_by_product_id(product_id, None)
      .fetch_one(db)
      .await?
      .exist
      .unwrap()
    {
      Err(AppError::AlreadyExists(
        "Product Already Exists".to_string(),
      ))
    } else {
      Ok(())
    }
  }
  