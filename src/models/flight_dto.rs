use sqlx::FromRow;

#[derive(Debug, FromRow, Default, Clone)]
pub struct FlightDTO {
    pub dep: String,
    pub arr: String,
    pub dof: String,
    pub flight_number: String,
    pub tail: String,
    pub pyld: String,
}
