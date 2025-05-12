use torii_ingester::ToriiClient;

use crate::repositories::land::LandRepository;

/// `LandModelsListenerTask` is a task that subscribes to some models of the on-chain indexer (torii),
/// and pushes them to the local database.
///
/// Supported models:
/// - Land
/// - LandStake
/// - Auctions (soon, TODO)
pub struct ModelsListenerTask {
    client: ToriiClient,
    event_repository: LandRepository,
}
