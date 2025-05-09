use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;
use torii_ingester::{error::ToriiConversionError, prelude::*};

use crate::models::shared::EventPrint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifierUpdatedEvent {
    new_verifier: Felt,
    old_verifier: Felt,
}

impl EventPrint for VerifierUpdatedEvent {
    fn pretty(&self) -> String {
        format!("🔒 Updated verifier to {:#x}", self.new_verifier)
    }
}

impl TryFrom<Struct> for VerifierUpdatedEvent {
    type Error = ToriiConversionError;

    fn try_from(entity: Struct) -> Result<Self, Self::Error> {
        Ok(Self {
            new_verifier: get!(entity, "new_verifier", Felt)?,
            old_verifier: get!(entity, "old_verifier", Felt)?,
        })
    }
}
