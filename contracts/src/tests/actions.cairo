// Starknet imports

use starknet::testing::{set_contract_address, set_block_timestamp, set_caller_address};
use starknet::{contract_address_const, ContractAddress};
// Dojo imports

use dojo::world::{IWorldDispatcher, IWorldDispatcherTrait};
use dojo::model::{ModelStorage, ModelValueStorage, ModelStorageTest};

//Internal imports

use ponzi_land::tests::setup::{setup, setup::{create_setup, deploy_erc20, RECIPIENT}};
use ponzi_land::systems::actions::{actions, IActionsDispatcher, IActionsDispatcherTrait};
use ponzi_land::models::land::{Land};

// External dependencies
use openzeppelin_token::erc20::interface::{IERC20CamelDispatcher, IERC20CamelDispatcherTrait};

fn LIQUIDITY_POOL() -> ContractAddress {
    contract_address_const::<'LIQUIDITY_POOL'>()
}

fn NEW_LIQUIDITY_POOL() -> ContractAddress {
    contract_address_const::<'NEW_LIQUIDITY_POOL'>()
}

fn FIRST_OWNER() -> ContractAddress {
    contract_address_const::<'FIRST_OWNER'>()
}

#[test]
fn test_actions() {
    let (mut world, actions_system, erc20) = create_setup();
    set_contract_address(RECIPIENT());

    let mut land: Land = world.read_model(11);

    //permisions for the contract
    erc20.approve(actions_system.contract_address, 10000);
    let allowance = erc20.allowance(RECIPIENT(), actions_system.contract_address);
    assert(allowance >= land.sell_price.into(), 'Approval failed');

    //creation of the land
    land.pool_key = LIQUIDITY_POOL();
    land.owner = FIRST_OWNER();
    land.sell_price = 123;
    land.token_used = erc20.contract_address;

    world.write_model_test(@land);

    actions_system.buy(NEW_LIQUIDITY_POOL(), erc20.contract_address, 10, 11, 12);

    let mut land: Land = world.read_model(11);
    assert(land.owner == RECIPIENT(),'has to be a new owner');
    assert(land.pool_key == NEW_LIQUIDITY_POOL(),'has to be a new lp');
    assert(land.sell_price == 10, 'has to be a new price');
}

