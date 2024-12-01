// Starknet imports

use starknet::ContractAddress;
use openzeppelin_token::erc20::interface::{IERC20CamelDispatcher, IERC20CamelDispatcherTrait};
// Interfaces

// #[starknet::interface]
// trait IERC20<TContractState> {
//     fn transfer(ref self: TContractState, recipient: ContractAddress, amount: u256) -> bool;
//     fn transferFrom(
//         ref self: TContractState, sender: ContractAddress, recipient: ContractAddress, amount: u256
//     ) -> bool;
// }

// Component

#[starknet::component]
mod PayableComponent {
    // Starknet imports

    use starknet::{ContractAddress};
    use starknet::info::get_contract_address;
    use starknet::storage::Map;

    // Internal imports

    // Local imports

    use super::{IERC20CamelDispatcher, IERC20CamelDispatcherTrait};

    // Errors

    mod errors {
        //     const ERC20_REWARD_FAILED: felt252 = 'ERC20: reward failed';
        const ERC20_STAKE_FAILED: felt252 = 'ERC20: stake failed';
        const ERC20_PAY_FAILED: felt252 = 'ERC20: pay failed';
        const ERC20_REFUND_FAILED: felt252 = 'ERC20: refund of stake failed';
    }

    #[derive(Drop, Serde, starknet::Store)]
    struct StakeInfo {
        token_address: ContractAddress,
        amount: u64,
    }

    // Storage

    #[storage]
    struct Storage {
        // token_address: ContractAddress,
        token_dispatcher: IERC20CamelDispatcher,
        //this stake_balance can be in another component
        stake_balance: Map<ContractAddress, StakeInfo>
    }

    // Events

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {}

    #[generate_trait]
    impl InternalImpl<
        TContractState, +HasComponent<TContractState>
    > of InternalTrait<TContractState> {
        fn _initialize(ref self: ComponentState<TContractState>, token_address: ContractAddress) {
            // [Storage] Set token address
            self.token_dispatcher.write(IERC20CamelDispatcher { contract_address: token_address });
        }

        fn _pay(
            self: @ComponentState<TContractState>,
            seller: ContractAddress,
            buyer: ContractAddress,
            amount: u64
        ) {
            // [Check] Amount is not null, otherwise return
            if amount == 0 {
                return;
            }

            // [Interaction] Transfer
            // let contract = get_contract_address();
            // let erc20 = IERC20Dispatcher { contract_address: self.token_address.read() };

            let status = self.token_dispatcher.read().transferFrom(buyer, seller, amount.into());

            // [Check] Status
            assert(status, errors::ERC20_PAY_FAILED);
        }

        fn _refund_of_stake(ref self: ComponentState<TContractState>, recipient: ContractAddress) {
            // [Interaction] Transfer
            // let erc20 = IERC20Dispatcher { contract_address:  };
            // let status = erc20.transfer(recipient, amount);

            let info_of_stake = self.stake_balance.read(recipient);
            self._initialize(info_of_stake.token_address);
            assert(info_of_stake.amount > 0, 'has to be more than 0');
            let status = self
                .token_dispatcher
                .read()
                .transfer(recipient, info_of_stake.amount.into());

            // [Check] Status
            assert(status, errors::ERC20_REFUND_FAILED);
        }

        fn _stake(
            ref self: ComponentState<TContractState>,
            staker: ContractAddress,
            token_address: ContractAddress,
            amount: u64
        ) {
            let contract = get_contract_address();
            self._initialize(token_address);
            let stake_info = StakeInfo{
                token_address,
                amount
            };
            let status = self.token_dispatcher.read().transferFrom(staker, contract, amount.into());
            assert(status, errors::ERC20_STAKE_FAILED);

            self.stake_balance.write(staker,stake_info);
        }
    }
}
