use std::sync::Arc;

use sqlx::{FromRow, SqlitePool};

use crate::repositories::CrudRepoTrait;

#[derive(Debug, Clone, FromRow)]
pub struct Flight {
    pub id: i64,
    pub dep: String,
    pub arr: String,
    pub dof: String,
    pub flight_number: String,
    pub tail: String,
    pub pyld: i32,
}

pub struct FlightsRepository {
    pool: Arc<SqlitePool>,
}

impl CrudRepoTrait<Flight> for FlightsRepository {
    fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    async fn create(&self, flight: &Flight) -> Result<Flight, sqlx::Error> {
        let id = sqlx::query(
            "INSERT INTO Flights (dep, arr, dof, flight_number, tail, pyld)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&flight.dep)
        .bind(&flight.arr)
        .bind(&flight.dof)
        .bind(&flight.flight_number)
        .bind(&flight.tail)
        .bind(flight.pyld)
        .execute(&*self.pool)
        .await?
        .last_insert_rowid();

        Ok(Flight {
            id,
            ..flight.clone()
        })
    }

    async fn get_all(&self) -> Result<Vec<Flight>, sqlx::Error> {
        sqlx::query_as::<_, Flight>("SELECT * FROM Flights ORDER BY id")
            .fetch_all(&*self.pool)
            .await
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<Flight>, sqlx::Error> {
        sqlx::query_as::<_, Flight>("SELECT * FROM Flights WHERE id = ?")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
    }

    async fn update(&self, flight: &Flight) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE Flights
             SET dep = ?, arr = ?, dof = ?, flight_number = ?, tail = ?, pyld = ?
             WHERE id = ?",
        )
        .bind(&flight.dep)
        .bind(&flight.arr)
        .bind(&flight.dof)
        .bind(&flight.flight_number)
        .bind(&flight.tail)
        .bind(flight.pyld)
        .bind(flight.id)
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM Flights WHERE id = ?")
            .bind(id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}
