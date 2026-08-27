use sqlx::FromRow;

#[derive(Debug, FromRow, Default)]
pub struct FlightDTO {
    pub dep: String,
    pub arr: String,
    pub dof: String,
    pub flight_number: String,
    pub tail: String,
    pub pyld: i32,
}
