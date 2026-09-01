use std::sync::Arc;

use sqlx::SqlitePool;

pub mod flights;

pub trait CrudRepoTrait<T> {
    type Dto;
    fn new(pool: Arc<SqlitePool>) -> Self;

    async fn create(&self, t: &Self::Dto) -> Result<T, sqlx::Error>;

    async fn get_all(&self) -> Result<Vec<T>, sqlx::Error>;

    async fn get_by_id(&self, id: i64) -> Result<Option<T>, sqlx::Error>;

    async fn update(&self, flight: &T) -> Result<(), sqlx::Error>;

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error>;
}
