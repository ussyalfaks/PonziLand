// Starknet imports
use starknet::contract_address::ContractAddressZeroable;
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

fn NEIGHBOR_1() -> ContractAddress {
    contract_address_const::<'NEIGHBOR_1'>()
}

fn NEIGHBOR_2() -> ContractAddress {
    contract_address_const::<'NEIGHBOR_2'>()
}

fn NEIGHBOR_3() -> ContractAddress {
    contract_address_const::<'NEIGHBOR_3'>()
}

#[test]
fn test_buy_action() {
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

    actions_system.buy(11, erc20.contract_address, 10, 12, NEW_LIQUIDITY_POOL(), false);
    let stake_balance = actions_system.get_stake_balance(RECIPIENT());

    let mut land: Land = world.read_model(11);
    assert(land.owner == RECIPIENT(), 'has to be a new owner');
    assert(land.pool_key == NEW_LIQUIDITY_POOL(), 'has to be a new lp');
    assert(land.sell_price == 10, 'has to be a new price');
    assert(stake_balance == 12, 'error with stake_balance')
}

#[test]
#[should_panic]
fn test_invalid_land() {
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

    actions_system.buy(11000, erc20.contract_address, 10, 12, NEW_LIQUIDITY_POOL(), false);
}

//test for now without auction
#[test]
fn test_bid_action() {
    let (mut world, actions_system, erc20) = create_setup();
    set_contract_address(RECIPIENT());

    //permisions for the contract
    erc20.approve(actions_system.contract_address, 10000);

    actions_system.bid(11, erc20.contract_address, 10, 12, NEW_LIQUIDITY_POOL());
    let stake_balance = actions_system.get_stake_balance(RECIPIENT());

    let mut land: Land = world.read_model(11);
    assert(land.owner == RECIPIENT(), 'has to be a new owner');
    assert(land.pool_key == NEW_LIQUIDITY_POOL(), 'has to be a new lp');
    assert(land.sell_price == 10, 'has to be a new price');
    assert(stake_balance == 12, 'error with stake_balance')
}

#[test]
fn test_taxes() {
    let (mut world, actions_system, erc20) = create_setup();

    let erc20_neighbor_1 = deploy_erc20(NEIGHBOR_1());
    let erc20_neighbor_2 = deploy_erc20(NEIGHBOR_2());
    let erc20_neighbor_3 = deploy_erc20(NEIGHBOR_3());

    //permisions for the contract
    set_contract_address(RECIPIENT());
    erc20.approve(actions_system.contract_address, 10000);

    let allowance = erc20.allowance(RECIPIENT(), actions_system.contract_address);
    assert(allowance >= 10000, 'Approval failed');

    //create lands

    //land 1
    actions_system.create_land(1280, 500, erc20.contract_address, LIQUIDITY_POOL(), 1000);
    let claimer_land: Land = world.read_model(1280);
    assert(claimer_land.owner == RECIPIENT(), 'error with the land owner');

    //neighboors
    set_contract_address(NEIGHBOR_1());
    erc20_neighbor_1.approve(actions_system.contract_address, 10000);
    let neighbor_1_land_1 = actions_system
        .create_land(1281, 500, erc20_neighbor_1.contract_address, LIQUIDITY_POOL(), 1000);

    set_contract_address(NEIGHBOR_2());
    erc20_neighbor_2.approve(actions_system.contract_address, 100000);
    let neighbor_2_land_1 = actions_system
        .create_land(1216, 500, erc20_neighbor_2.contract_address, LIQUIDITY_POOL(), 1000);

    set_contract_address(NEIGHBOR_3());
    erc20_neighbor_3.approve(actions_system.contract_address, 10000);
    let neighbor_3_land_1 = actions_system
        .create_land(1344, 500, erc20_neighbor_3.contract_address, LIQUIDITY_POOL(), 1000);

    //generate taxes for land 1 to neighboors

    set_contract_address(RECIPIENT());
    erc20_neighbor_1.approve(RECIPIENT(), 100);
    erc20_neighbor_2.approve(RECIPIENT(), 100);
    erc20_neighbor_3.approve(RECIPIENT(), 100);

    actions_system.claim(1280);

    //verify the claimer land
    let claimer_land_taxes = actions_system.get_pending_taxes(claimer_land.owner);
    assert(claimer_land_taxes.len() == 0, 'have pending taxes');
    assert(erc20_neighbor_1.balanceOf(claimer_land.owner) == 3, 'fail in pay taxes');
    assert(erc20_neighbor_2.balanceOf(claimer_land.owner) == 5, 'fail in pay taxes');
    assert(erc20_neighbor_3.balanceOf(claimer_land.owner) == 5, 'fail in pay taxes');

    //verify the neighbors of the claimer land
    let taxes_neighbor_1 = actions_system.get_pending_taxes(neighbor_1_land_1.owner);
    let taxes_neighbor_2 = actions_system.get_pending_taxes(neighbor_2_land_1.owner);
    let taxes_neighbor_3 = actions_system.get_pending_taxes(neighbor_3_land_1.owner);

    assert(taxes_neighbor_1.len() > 0, 'must have pending taxes');
    assert(taxes_neighbor_2.len() > 0, 'must have pending taxes');
    assert(taxes_neighbor_3.len() > 0, 'must have pending taxes');

    let stake_balance_after_taxes_neighbor_1 = actions_system.get_stake_balance(NEIGHBOR_1());
    let stake_balance_after_taxes_neighbor_2 = actions_system.get_stake_balance(NEIGHBOR_2());
    let stake_balance_after_taxes_neighbor_3 = actions_system.get_stake_balance(NEIGHBOR_3());

    assert(stake_balance_after_taxes_neighbor_1 < 1000, 'must have less stake');
    assert(stake_balance_after_taxes_neighbor_2 < 1000, 'must have less stake');
    assert(stake_balance_after_taxes_neighbor_3 < 1000, 'must have less stake');
}

#[test]
fn test_nuke_action() {
    let (mut world, actions_system, erc20) = create_setup();

    let erc20_neighbor_1 = deploy_erc20(NEIGHBOR_1());

    //permisions for the contract
    set_contract_address(RECIPIENT());
    erc20.approve(actions_system.contract_address, 10000);

    let allowance = erc20.allowance(RECIPIENT(), actions_system.contract_address);
    assert(allowance >= 10000, 'Approval failed');

    //create lands

    //land 1
    actions_system.create_land(1280, 500, erc20.contract_address, LIQUIDITY_POOL(), 100);
    let claimer_land: Land = world.read_model(1280);
    assert(claimer_land.owner == RECIPIENT(), 'error with the land owner');

    //neighboors
    set_contract_address(NEIGHBOR_1());
    erc20_neighbor_1.approve(actions_system.contract_address, 10000);
    let neighbor_1_land_1 = actions_system
        .create_land(1281, 500, erc20_neighbor_1.contract_address, LIQUIDITY_POOL(), 10);

    //permisions for transfer tokens
    set_contract_address(RECIPIENT());
    erc20_neighbor_1.approve(RECIPIENT(), 100);

    let nuked_land = actions_system.get_land(1281);

    assert(nuked_land.owner != ContractAddressZeroable::zero(), 'owner must exist');
    assert(nuked_land.sell_price != 0, 'sell_price must exist');
    assert(nuked_land.pool_key != ContractAddressZeroable::zero(), 'pool_key must exist');

    actions_system.claim(claimer_land.location);
    let stake_neighbor_1 = actions_system.get_stake_balance(neighbor_1_land_1.owner);
    let stake_claimer_land = actions_system.get_stake_balance(claimer_land.owner);

    let nuked_land = actions_system.get_land(1281);

    assert(nuked_land.owner == ContractAddressZeroable::zero(), 'owner must be 0');
    assert(nuked_land.sell_price == 0, 'sell_price must be 0');
    assert(nuked_land.pool_key == ContractAddressZeroable::zero(), 'pool_key must be 0');
}

