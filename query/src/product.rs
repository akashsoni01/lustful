use chrono::Utc;
use sqlx::postgres::PgRow;
use sqlx::Postgres;
use sqlx::{postgres::PgArguments, query::Query};
use uuid::Uuid;
use entity::product::Product;
use model::request::{Direction, PageParamQuery};

use model::record::*;

#[tracing::instrument]
pub fn save(item: &Product) -> Query<Postgres, PgArguments> {
  assert!(item.create_at.is_none());
  assert!(item.update_at.is_none());
  sqlx::query!(
    r#"INSERT INTO products (id,title,product_id,is_active) VALUES ($1,$2,$3,$4)"#,
    item.id,
    item.title,
    item.product_id,
    item.is_active,
  )
}

#[tracing::instrument]
pub fn find_by_id(
  id: &Uuid,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    Product,
    r#"SELECT id,title,product_id,
        is_active,create_at,update_at FROM products WHERE id = $1"#,
    id
  )
}

#[tracing::instrument]
pub fn find_page(
  page: PageParamQuery,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    Product,
    r#"SELECT id,title,product_id,is_active,
    create_at,update_at FROM products ORDER BY 
            (CASE WHEN $1 = 'create_at' AND $2 = 'ASC' THEN create_at END) ASC,
            (CASE WHEN $1 = 'create_at' AND $2 = 'DESC' THEN create_at END) DESC,
            (CASE WHEN $1 = 'title' AND $2 = 'ASC' THEN title END) ASC,
            (CASE WHEN $1 = 'title' AND $2 = 'DESC' THEN title END) DESC
            LIMIT $3 OFFSET $4"#,
    page.sort_by.unwrap_or("create_at".to_string()),
    page.sort_direction.unwrap_or(Direction::DESC).to_string(),
    page.page_size,
    page.page_num * page.page_size,
  )
}

#[tracing::instrument]
pub fn update(item: &Product) -> Query<Postgres, PgArguments> {
  sqlx::query!(
    r#"UPDATE products SET title=$1,product_id=$2,is_active=$3, WHERE id = $4"#,
    item.title,
    item.product_id,
    item.is_active,
    item.id,
  )
}

#[tracing::instrument]
pub fn exist_by_product_id(
  product_id: &str,
  is_active: Option<bool>,
) -> sqlx::query::Map<
  'static,
  Postgres,
  impl FnMut(PgRow) -> Result<ExistRecord, sqlx::Error>,
  PgArguments,
> {
  sqlx::query_as!(
    ExistRecord,
    r#"SELECT EXISTS(SELECT 1 FROM products WHERE (is_active = $1 OR $1 IS NULL) 
            AND (product_id = $2)) AS exist"#,
    is_active,
    product_id,
  )
}

#[tracing::instrument]
pub fn delete_all_inative_product() -> Query<'static, Postgres, PgArguments> {
  sqlx::query!(
    r#"DELETE FROM products WHERE is_active = $1 AND create_at <= $2"#,
    false,
    Utc::now() - chrono::Duration::days(1)
  )
}
