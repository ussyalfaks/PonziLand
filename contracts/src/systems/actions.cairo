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
    );

    fn claim(ref self: T, land_location: u64);

    fn nuke(ref self: T, land_location: u64);

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

    #[derive(Drop, Serde)]
    #[dojo::event]
    pub struct NewLandEvent {
        #[key]
        owner_land: ContractAddress,
        #[key]
        land_location: u64,
        token_for_sale: ContractAddress,
        sell_price: u64
    }

    #[derive(Drop, Serde)]
    #[dojo::event]
    pub struct RemainingStakeEvent {
        #[key]
        land_location: u64,
        remaining_stake: u64
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
        ) {
            assert(is_valid_position(land_location), 'Land location not valid');
            assert(sell_price > 0, 'sell_price > 0');
            assert(amount_to_stake > 0, 'amount_to_stake > 0');

            let mut world = self.world_default();
            let caller = get_caller_address();
            let mut land: Land = world.read_model(land_location);
            assert(land.owner != ContractAddressZeroable::zero(), 'must have a owner');
            self.internal_claim(ref world, land);

            self.payable._pay(caller, land.owner, land.token_used, land.sell_price);
            self.payable._refund_of_stake(land.owner);
            self.payable._stake(caller, token_for_sale, amount_to_stake);

            self
                .process_buy(
                    land_location,
                    token_for_sale,
                    sell_price,
                    amount_to_stake,
                    liquidity_pool,
                    land,
                    caller
                );
        }


        fn claim(ref self: ContractState, land_location: u64) {
            assert(is_valid_position(land_location), 'Land location not valid');

            let mut world = self.world_default();
            let mut land: Land = world.read_model(land_location);
            let caller = get_caller_address();

            assert(land.owner == caller, 'not the owner');
            self.internal_claim(ref world, land);
        }


        // TODO:see if we want pass this function into internalTrait
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
            //whe have to see how manage the caller or the owner of the land with auction part
            let caller = get_caller_address();
            //find the way to validate the liquidity pool for the new token_for_sale
            //some assert();
            assert(is_valid_position(land_location), 'Land location not valid');
            assert(land.owner == ContractAddressZeroable::zero(), 'must be without owner');
            assert(sell_price > 0, 'sell_price > 0');
            assert(amount_to_stake > 0, 'amount_to_stake > 0');

            //auction part

            //when the auction part its finished

            self.internal_claim(ref world, land);
            self
                .buy_from_bid(
                    land_location,
                    token_for_sale,
                    sell_price,
                    amount_to_stake,
                    liquidity_pool,
                    land,
                    caller
                );
        }


        //GETTERS FUNCTIONS

        //TODO: here we have to change the return to struct of TokenInfo, no only amount
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

        fn internal_claim(ref self: ContractState, ref world: WorldStorage, land: Land) {
            //generate taxes for each neighbor of claimer
            let neighbors = self.payable._add_neighbors(world, land.location);
            if neighbors.len() != 0 {
                for location in neighbors {
                    match self.payable._generate_taxes(world, location) {
                        Result::Ok(remaining_stake) => {
                            if remaining_stake != 0 {
                                world
                                    .emit_event(
                                        @RemainingStakeEvent {
                                            land_location: location, remaining_stake
                                        }
                                    )
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


        fn buy_from_bid(
            ref self: ContractState,
            land_location: u64,
            token_for_sale: ContractAddress,
            sell_price: u64,
            amount_to_stake: u64,
            liquidity_pool: ContractAddress,
            mut land: Land,
            caller: ContractAddress,
        ) {
            //TODO: we have to create our contract to send the tokens for the first sell
            //self.payable._pay_to_us();
            self.payable._stake(caller, token_for_sale, amount_to_stake);

            self
                .process_buy(
                    land_location,
                    token_for_sale,
                    sell_price,
                    amount_to_stake,
                    liquidity_pool,
                    land,
                    caller
                );
        }

        fn process_buy(
            ref self: ContractState,
            land_location: u64,
            token_for_sale: ContractAddress,
            sell_price: u64,
            amount_to_stake: u64,
            liquidity_pool: ContractAddress,
            mut land: Land,
            caller: ContractAddress,
        ) {
            land.owner = caller;
            land.block_date_bought = get_block_timestamp();
            land.sell_price = sell_price;
            land.pool_key = liquidity_pool;
            land.token_used = token_for_sale;
            land.last_pay_time = get_block_timestamp();
            let mut world = self.world_default();
            world.write_model(@land);
            world
                .emit_event(
                    @NewLandEvent {
                        owner_land: land.owner, land_location, token_for_sale, sell_price
                    }
                );
        }
    }
}

