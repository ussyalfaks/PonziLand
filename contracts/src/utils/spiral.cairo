use ponzi_land::consts::MAX_AUCTIONS;
use ponzi_land::helpers::coord::{
    down, down_left, down_right, index_to_position, left, position_to_index, right, up, up_left,
    up_right,
};
use ponzi_land::store::{Store, StoreTrait};
use starknet::storage::{
    Map, MutableVecTrait, StoragePointerReadAccess, StoragePointerWriteAccess, Vec, VecTrait,
};


#[derive(Copy, Drop, Serde, starknet::Store, Debug)]
struct SpiralState {
    direction: u8, // 0=left, 1=top, 2=right, 3=down
    current_head: u8,
    steps: u64,
    advance: u64,
    steps_remaining: Option<u64>,
}


// Helper function to get next position based on direction
fn get_next_position(direction: u8, location: u16) -> Option<u16> {
    match direction {
        0 => left(location),
        1 => up(location),
        2 => right(location),
        3 => down(location),
        _ => Option::None,
    }
}

