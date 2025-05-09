use torii_ingester::{error::ToriiConversionError, prelude::*};

use serde::{Deserialize, Serialize};

use crate::models::shared::EventPrint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressAuthorizedEvent {
    address: ContractAddress,
    authorized_at: u64,
}

impl EventPrint for AddressAuthorizedEvent {
    fn pretty(&self) -> String {
        format!("🔒 Address authorized: {}", self.address)
    }
}

impl TryFrom<Struct> for AddressAuthorizedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            address: get!(entity, "address", ContractAddress)?,
            authorized_at: get!(entity, "authorized_at", u64)?,
        })
    }
}
