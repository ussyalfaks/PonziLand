use crate::conversions::Error;

#[derive(thiserror::Error, Debug)]
pub enum ToriiConversionError {
    #[error("{0}: No such field")]
    NoSuchField(String),
    #[error("{0}: Not a primitive")]
    NotAPrimitive(String),
    #[error("{0}: Not an enum")]
    NotAnEnum(String),
    #[error("{0}: Wrong type: {1:#?}")]
    WrongType(String, Error),

    #[error("Unknown variant {0}")]
    UnknownVariant(String),
    #[error("JSON parsing error: {0}")]
    JsonParsingError(#[from] serde_json::Error),
}
