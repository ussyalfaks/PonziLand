use crate::conversions::Error;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ToriiConversionError {
    #[error("{0}: No such field")]
    NoSuchField(String),
    #[error("{0}: Not a primitive")]
    NotAPrimitive(String),
    #[error("{0}: Wrong type: {1:#?}")]
    WrongType(String, Error),
}
