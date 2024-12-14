use starknet::ContractAddress;

use dojo::world::WorldStorage;
use ponzi_land::models::land::Land;
use ponzi_land::components::payable::PayableComponent::TokenInfo;

// define the interface
#[starknet::interface]
trait IActions<T> {
    fn bid(ref self: T);
    fn buy(
        ref self: T,
        liquidity_pool: ContractAddress,
        token_for_sale: ContractAddress,
        sell_price: u64,
        land_location: u64,
        amount_to_stake: u64
    );

    fn claim(ref self: T, land_location: u64);

    fn nuke(ref self: T, land_location: u64);

    fn create_land(
        ref self: T,
        location: u64,
        sell_price: u64,
        token_used: ContractAddress,
        pool_key: ContractAddress,
        stake_amount: u64
    ) -> Land;


    //getters
    fn get_stake_balance(self: @T, staker: ContractAddress) -> u64;
    fn get_land(self: @T, land_location: u64) -> Land;
    fn get_pending_taxes(self: @T, owner_land: ContractAddress) -> Array<TokenInfo>;
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions, WorldStorage};
    use starknet::{ContractAddress, get_caller_address, get_block_timestamp, get_contract_address};
    use starknet::contract_address::ContractAddressZeroable;
    use dojo::model::{ModelStorage, ModelValueStorage};
    use ponzi_land::models::land::Land;
    use ponzi_land::components::payable::{PayableComponent, PayableComponent::TokenInfo};
    use ponzi_land::helpers::coord::{is_valid_position, up, down, left, right};
    use ponzi_land::consts::{TAX_RATE};

    component!(path: PayableComponent, storage: payable, event: PayableEvent);
    impl PayableInternalImpl = PayableComponent::InternalImpl<ContractState>;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        PayableEvent: PayableComponent::Event,
        LandEvent: LandCreated,
    }

    #[derive(Drop, starknet::Event)]
    struct LandCreated {
        land: Land
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
            land_location: u64,
            amount_to_stake: u64,
        ) {
            let mut world = self.world_default();
            let caller = get_caller_address();
            let mut land: Land = world.read_model(land_location);

            //preguntar si es mejor ponerlo al principio de la funcion por temas de gas
            assert(is_valid_position(land_location), 'Land location not valid');
            assert(sell_price > 0, 'sell_price > 0');

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

        //see what we want return in this function
        fn claim(ref self: ContractState, land_location: u64) {
            //review valid_position function
            is_valid_position(land_location);
            let mut world = self.world_default();
            let caller = get_caller_address();
            let land: Land = world.read_model(land_location);

            assert(land.owner == caller, 'not the owner');
            let neighbors = self.payable._add_neighbors(world, land_location);
            if neighbors.len() != 0 {
                for location in neighbors {
                    match self.payable._generate_taxes(world, location) {
                        Result::Ok(_) => {},
                        Result::Err(_) => { self.nuke(location); }
                    };
                };
            }

            let taxes = self.get_pending_taxes(land.owner);
            if taxes.len() != 0 {
                self.payable._claim_taxes(taxes, land.owner)
            }
        }

        fn nuke(
            ref self: ContractState, land_location: u64,
        ) { // nuke all land wherer the LP is smaller than the sell price
        // println!("inside of function nukeee");
        }


        //inputs: LP, token to sell for, price, Bid offer(in a main currency(Lords?))
        //In this function we want to use the auction logic and then reuse the buy function?
        fn bid(ref self: ContractState) { // to buy fresh unowned land
        // bid on a land
        }


        //GETTERS FUNCTIONS

        fn get_stake_balance(self: @ContractState, staker: ContractAddress) -> u64 {
            self.payable.stake_balance.read(staker).amount
        }

        fn get_pending_taxes(
            self: @ContractState, owner_land: ContractAddress
        ) -> Array<TokenInfo> {
            let mut token_info: Array<TokenInfo> = ArrayTrait::new();
            let taxes_length = self.payable.pending_taxes_length.read(owner_land);
            for mut i in 0
                ..taxes_length {
                    let pending_tax = self.payable.pending_taxes.read((owner_land, i));
                    if pending_tax.amount > 0 {
                        token_info.append(pending_tax);
                    };
                };
            token_info
        }

        fn get_land(self: @ContractState, land_location: u64) -> Land {
            assert(is_valid_position(land_location), 'Land location not valid');
            let mut world = self.world_default();
            let mut land: Land = world.read_model(land_location);
            land
        }

        //I will change this for bid function after, only for fast test
        fn create_land(
            ref self: ContractState,
            location: u64,
            sell_price: u64,
            token_used: ContractAddress,
            pool_key: ContractAddress,
            stake_amount: u64
        ) -> Land {
            assert(is_valid_position(location), 'Land location not valid');
            let mut world = self.world_default();
            let mut land: Land = world.read_model(location);
            let caller = get_caller_address();
            land.owner = caller;
            land.block_date_bought = get_block_timestamp();
            land.sell_price = sell_price;
            land.pool_key = pool_key;
            land.token_used = token_used;
            self.payable._stake(caller, token_used, stake_amount);

            world.write_model(@land);
            self.emit(LandCreated { land });
            land
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

