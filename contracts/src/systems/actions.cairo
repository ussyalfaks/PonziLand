
// define the interface
#[starknet::interface]
trait IActions<T> {
    fn buy(ref self: T);
    fn claim(ref self: T);
    fn nuke(ref self: T);
    fn bid(ref self: T);
}

// dojo decorator
#[dojo::contract]
pub mod actions {
    use super::{IActions};
    use starknet::{ContractAddress, get_caller_address};

    #[abi(embed_v0)]
    impl ActionsImpl of IActions<ContractState> {
        //inputs: LP, token to sell for, price
        fn buy(ref self: ContractState ) {
            //do shit
        }
        fn claim(ref self:ContractState) {
            //do shit
        }
        fn nuke(ref self: ContractState) {
            // nuke all land wherer the LP is smaller than the sell price
        }
        //inputs: LP, token to sell for, price, Bid offer(in a main currency(Lords?))
        fn bid(ref self: ContractState) { // to buy fresh unowned land
            // bid on a land
        }
    }
}

