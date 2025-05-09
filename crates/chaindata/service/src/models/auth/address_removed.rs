use serde::{Deserialize, Serialize};
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::shared::EventPrint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressRemovedEvent {
    address: ContractAddress,
    authorized_at: u64,
}

impl EventPrint for AddressRemovedEvent {
    fn pretty(&self) -> String {
        format!("🔒 Address unauthorized: {}", self.address)
    }
}

impl TryFrom<Struct> for AddressRemovedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            address: get!(entity, "address", ContractAddress)?,
            authorized_at: get!(entity, "authorized_at", u64)?,
        })
    }
}
