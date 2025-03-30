use crate::storage::database::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum NapfulError {
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Io error: {0}")]
    Io(#[from] std::io::Error)
}
