use starknet::ContractAddress;

use dojo::world::WorldStorage;
use ponzi_land::models::land::Land;
use ponzi_land::components::payable::PayableComponent::TokenInfo;

// define the interface
#[starknet::interface]
trait IActions<T> {
    fn bid(
        ref self: T,
        land_location: u64,
        token_for_sale: ContractAddress,
        sell_price: u64,
        amount_to_stake: u64,
        liquidity_pool: ContractAddress,
    );
    fn buy(
        ref self: T,
        land_location: u64,
        token_for_sale: ContractAddress,
        sell_price: u64,
        amount_to_stake: u64,
        liquidity_pool: ContractAddress,
        is_from_bid: bool
    );

    fn claim(ref self: T, land_location: u64, is_from_sell: bool);

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
    use dojo::event::EventStorage;

    component!(path: PayableComponent, storage: payable, event: PayableEvent);
    impl PayableInternalImpl = PayableComponent::InternalImpl<ContractState>;

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        PayableEvent: PayableComponent::Event,
    }

    //events

    #[derive(Drop, Serde)]
    #[dojo::event]
    pub struct LandNukedEvent {
        #[key]
        owner_nuked: ContractAddress,
        land_location: u64,
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
            land_location: u64,
            token_for_sale: ContractAddress,
            sell_price: u64,
            amount_to_stake: u64,
            liquidity_pool: ContractAddress,
            is_from_bid: bool
        ) {
            let mut world = self.world_default();
            let caller = get_caller_address();
            let mut land: Land = world.read_model(land_location);

            assert(is_valid_position(land_location), 'Land location not valid');
            assert(sell_price > 0, 'sell_price > 0');
            assert(amount_to_stake > 0, 'amount_to_stake > 0');

            if is_from_bid {
                //TODO: we have to create our contract to send the tokens for the first sell
                //self.payable._pay_us();
                //self.payable._validate(caller, token_for_sale, land.sell_price);

                self.payable._stake(caller, token_for_sale, amount_to_stake);
            } else {
                //we see if the buyer has the token and the amount for buy the land
                assert(land.owner != ContractAddressZeroable::zero(), 'must have a owner');
                self.claim(land_location, true);
                self.payable._validate(caller, land.token_used, land.sell_price);
                self.payable._pay(caller, land.owner, land.sell_price.into());

                self.payable._refund_of_stake(land.owner);
                self.payable._stake(caller, token_for_sale, amount_to_stake);
            }

            land.owner = caller;
            land.block_date_bought = get_block_timestamp();
            land.sell_price = sell_price;
            land.pool_key = liquidity_pool;
            land.token_used = token_for_sale;
            land.last_pay_time = Option::None;

            world.write_model(@land);
        }

        //TODO:what happens if someone wants to call this from voyager?
        fn claim(ref self: ContractState, land_location: u64, is_from_sell: bool) {
            is_valid_position(land_location);

            let mut world = self.world_default();
            let caller = get_caller_address();
            let land: Land = world.read_model(land_location);

            if !is_from_sell {
                assert(land.owner == caller, 'not the owner')
            };

            let neighbors = self.payable._add_neighbors(world, land_location);
            if neighbors.len() != 0 {
                for location in neighbors {
                    match self.payable._generate_taxes(world, location) {
                        Result::Ok(x) => {
                            if x == 'First taxes' {//do some event
                            // println!("First taxes");
                            }
                        },
                        Result::Err(_) => {
                            // println!("nuke");
                            self.nuke(location);
                        }
                    };
                };
            }

            let taxes = self.get_pending_taxes(land.owner);
            if taxes.len() != 0 {
                self.payable._claim_taxes(taxes, land.owner)
            }
        }

        fn nuke(ref self: ContractState, land_location: u64,) {
            let mut world = self.world_default();
            let mut land: Land = world.read_model(land_location);
            let owner_nuked = land.owner;

            //delete land
            world.erase_model(@land);

            //emit event de nuke land
            world.emit_event(@LandNukedEvent { owner_nuked, land_location });
        }


        //Bid offer(in a main currency(Lords?))
        // how we know who will be the owner of the land?
        fn bid(
            ref self: ContractState,
            land_location: u64,
            token_for_sale: ContractAddress,
            sell_price: u64,
            amount_to_stake: u64,
            liquidity_pool: ContractAddress,
        ) {
            let mut world = self.world_default();
            let land: Land = world.read_model(land_location);

            //find the way to validate the liquidity pool for the new token_for_sale
            //some assert();
            assert(is_valid_position(land_location), 'Land location not valid');
            assert(land.owner == ContractAddressZeroable::zero(), 'must be without owner');

            //auction part

            //when the auction part its finished

            self
                .buy(
                    land_location, token_for_sale, sell_price, amount_to_stake, liquidity_pool, true
                );
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
            land
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
    }


    #[generate_trait]
    impl InternalImpl of InternalTrait {
        fn world_default(self: @ContractState) -> WorldStorage {
            self.world(@"ponzi_land")
        }
    }
}

