use starknet::ContractAddress;

use dojo::world::WorldStorage;

// define the interface
#[starknet::interface]
trait IActions<T> {
    fn buy(
        ref self: T,
        liquidity_pool: ContractAddress,
        token_for_sale: ContractAddress,
        sell_price: u64,
        location_land: u64,
        amount_to_stake: u64
    );
    fn claim(ref self: T);
    fn nuke(ref self: T);
    fn bid(ref self: T);
    fn get_stake_balance(self: @T, staker: ContractAddress) -> u64;
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions, WorldStorage};
    use starknet::{ContractAddress, get_caller_address, get_block_timestamp, get_contract_address};
    use dojo::model::{ModelStorage, ModelValueStorage};
    use ponzi_land::models::land::Land;
    use ponzi_land::components::payable::PayableComponent;
    use ponzi_land::helpers::coord::is_valid_position;


    component!(path: PayableComponent, storage: payable, event: PayableEvent);
    impl PayableInternalImpl = PayableComponent::InternalImpl<ContractState>;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        PayableEvent: PayableComponent::Event,
    }

    // Storage
    #[storage]
    struct Storage {
        #[substorage(v0)]
        payable: PayableComponent::Storage,
    }

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn buy(
            ref self: ContractState,
            liquidity_pool: ContractAddress,
            token_for_sale: ContractAddress,
            sell_price: u64,
            location_land: u64,
            amount_to_stake: u64,
        ) {
            let mut world = self.world_default();
            let caller = get_caller_address();
            let mut land: Land = world.read_model(location_land);

            assert(is_valid_position(location_land), 'land not exists');
            assert(sell_price > 0, 'has to be more than 0');

            self.payable._validate(caller, token_for_sale, land.sell_price);

            // //find the way to validate the liquidity pool for the new token_for_sale
            // //some assert();

            self.payable._pay(caller, land.owner, land.sell_price.into());

            //TODO:we have to see in what moment we do the stake for the first sell
            // self.payable._refund_of_stake(land.owner);

            self.payable._stake(caller, token_for_sale, amount_to_stake);

            land.owner = caller;
            land.block_date_bought = get_block_timestamp();
            land.sell_price = sell_price;
            land.pool_key = liquidity_pool;
            land.token_used = token_for_sale;

            world.write_model(@land);
        }

        fn claim(ref self: ContractState) { //do shit
        }

        fn nuke(
            ref self: ContractState
        ) { // nuke all land wherer the LP is smaller than the sell price
        }

        //inputs: LP, token to sell for, price, Bid offer(in a main currency(Lords?))

        //In this function we want to use the auction logic and then reuse the buy function?

        fn bid(ref self: ContractState) { // to buy fresh unowned land
        // bid on a land
        }

        fn get_stake_balance(self: @ContractState, staker: ContractAddress) -> u64 {
            self.payable.stake_balance.read(staker).amount
        }
    }


    #[generate_trait]
    impl InternalImpl of InternalTrait {
        /// This function is handy since the ByteArray can't be const.
        fn world_default(self: @ContractState) -> WorldStorage {
            self.world(@"ponzi_land")
        }
    }
}

