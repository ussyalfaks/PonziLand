use chrono::{DateTime, Utc};
use serde_json::Value;
use torii_ingester::{error::ToriiConversionError, prelude::Struct, RawToriiData};

use super::{
    event::EventData,
    land::{
        land::{Land, LandModel},
        land_stake::{LandStake, LandStakeModel},
    },
};

pub enum Model {
    Land(Land),
    LandStake(LandStake),
}

impl TryFrom<Struct> for Model {
    type Error = ToriiConversionError;
    fn try_from(value: Struct) -> Result<Self, Self::Error> {
        Ok(match &*value.name {
            "ponzi_land-Land" => Model::Land(Land::try_from(value)?),
            "ponzi_land-LandStake" => Model::LandStake(LandStake::try_from(value)?),
            name => Err(ToriiConversionError::UnknownVariant(name.to_string()))?,
        })
    }
}

impl Model {
    pub fn from_json(name: &str, json: Value) -> Result<Self, ToriiConversionError> {
        Ok(match name {
            "ponzi_land-Land" => Model::Land(serde_json::from_value(json)?),
            "ponzi_land-LandStake" => Model::LandStake(serde_json::from_value(json)?),
            name => Err(ToriiConversionError::UnknownVariant(name.to_string()))?,
        })
    }

    pub fn parse(
        data: RawToriiData,
    ) -> Result<(Self, Option<DateTime<Utc>>), ToriiConversionError> {
        Ok(match data {
            RawToriiData::Json { name, data, at } => (Self::from_json(&name, data)?, Some(at)),
            RawToriiData::Grpc(data) => (Self::try_from(data)?, None),
        })
    }
}
