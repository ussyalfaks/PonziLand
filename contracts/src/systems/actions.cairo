
// define the interface
#[starknet::interface]
trait IActions<T> {
    fn buy(ref self: T);
    fn claim(ref self: T);
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions};
    use starknet::{ContractAddress, get_caller_address};

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        fn buy(ref self: ContractState) {
            //do shit
        }
        fn claim(ref self:ContractState) {
            //do shit
        }
    }
}

