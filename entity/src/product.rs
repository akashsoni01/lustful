use chrono::{DateTime, Utc};
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Dummy;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, FromRow)]
pub struct Prduct {
  pub id: Uuid,
  pub title: String,
  pub product_id: String,
  pub is_active: bool,
  pub create_at: Option<DateTime<Utc>>,
  pub update_at: Option<DateTime<Utc>>,
}

impl Prduct {
  pub fn new(
    title: impl Into<String>,
    product_id: impl Into<String>,
  ) -> Self {
    Self {
      id: Uuid::new_v4(),
      title: title.into(),
      product_id: product_id.into(),
      is_active: false,
      create_at: None,
      update_at: None,
    }
  }
}
