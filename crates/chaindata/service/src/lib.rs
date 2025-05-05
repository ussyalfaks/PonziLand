use serde::*;

/// ChainDataService is a service that handles the importation and syncing of new events and data
/// to the database for further processing.
pub struct ChainDataService {}

#[derive(Serialize, Deserialize)]
pub struct ChainDataServiceConfiguration {}

impl ChainDataService {
    pub fn new() -> Self {
        Self {}
    }
}
