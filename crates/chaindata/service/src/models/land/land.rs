use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::prelude::{FromRow, Type};
use torii_ingester::{
    conversions::{torii_enum_deserializer, Error, FromEnum},
    error::ToriiConversionError,
    get,
    prelude::{ContractAddress, Enum, Struct},
};

use crate::{
    define_id,
    models::{shared::Location, types::U256},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type, Copy)]
#[repr(i32)]
pub enum Level {
    Zero = 0,
    First = 1,
    Second = 2,
}

impl FromEnum for Level {
    fn from_enum(variant: &Enum) -> Result<Self, Error> {
        match variant.option() {
            Ok(variant) => match &*variant.name {
                "Zero" => Ok(Level::Zero),
                "First" => Ok(Level::First),
                "Second" => Ok(Level::Second),
                _ => Err(Error::InvalidEnumVariant(variant.name.clone())),
            },
            Err(err) => Err(err.into()),
        }
    }
}

define_id!(LandModelId, Uuid);

/// Rust representation of the on-chain land model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Land {
    pub location: Location,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub block_date_bought: u64,
    pub owner: ContractAddress,
    pub sell_price: U256,
    pub token_used: ContractAddress,
    #[serde(deserialize_with = "torii_enum_deserializer")]
    pub level: Level,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LandModel {
    pub id: LandModelId,
    pub at: NaiveDateTime,
    pub location: Location,
    pub bought_at: NaiveDateTime,
    pub owner: String,
    pub sell_price: U256,
    pub token_used: String,
    pub level: Level,
}

impl LandModel {
    pub fn from_land_at(land: &Land, at: NaiveDateTime) -> Self {
        Self {
            id: LandModelId::new(),
            at,
            location: land.location,
            bought_at: DateTime::from_timestamp(land.block_date_bought as i64, 0)
                .unwrap()
                .naive_utc(),
            owner: land.owner.to_string(),
            sell_price: land.sell_price,
            token_used: land.token_used.to_string(),
            level: land.level,
        }
    }
}

impl TryFrom<Struct> for Land {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            location: get!(entity, "land_location", Location)?,
            block_date_bought: get!(entity, "block_date_bought", u64)?,
            owner: get!(entity, "owner", ContractAddress)?,
            sell_price: get!(entity, "sell_price", U256)?,
            token_used: get!(entity, "token_used", ContractAddress)?,
            level: get!(entity, "level", enum Level)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_land_model_deserialization_torii() {
        let json = r#"
            {
              "block_date_bought":"0",
              "level":{"Zero":[]},
              "location":2080,
              "owner":"0x0",
              "sell_price":"0x000000000000000000000000000000000000000000000006f05b59d3b2000000",
              "token_used":"0x5735fa6be5dd248350866644c0a137e571f9d637bb4db6532ddd63a95854b58"
            }
            "#;

        let deserialization =
            serde_json::from_str::<Land>(json).expect("Error while deserializing!");

        assert_eq!(deserialization.level, Level::Zero);
    }
}
