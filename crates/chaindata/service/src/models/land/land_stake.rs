use crate::models::{shared::Location, types::U256};
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use torii_ingester::{error::ToriiConversionError, get, prelude::Struct};

/// Rust representation of the on-chain land stake model.
///
/// Used to convert from the torii-representation to the rust one
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandStake {
    pub location: Location,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub last_pay_time: u64,
    pub amount: U256,
}

crate::define_id!(LandStakeId, Uuid);

pub struct LandStakeModel {
    pub id: LandStakeId,
    pub at: NaiveDateTime,
    pub location: Location,
    pub last_pay_time: NaiveDateTime,
    pub amount: U256,
}

impl LandStakeModel {
    pub fn from_land_at(land: &LandStake, at: NaiveDateTime) -> Self {
        Self {
            id: LandStakeId::new(),
            at,
            location: land.location,
            last_pay_time: DateTime::from_timestamp(land.last_pay_time as i64, 0)
                .unwrap()
                .naive_utc(),
            amount: land.amount,
        }
    }
}

impl TryFrom<Struct> for LandStake {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            location: get!(entity, "land_location", Location)?,
            last_pay_time: get!(entity, "last_pay_time", u64)?,
            amount: get!(entity, "amount", U256)?,
        })
    }
}
