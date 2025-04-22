// TaxesComponent using timestamp-based claiming system
use openzeppelin_token::erc20::interface::{IERC20CamelDispatcher, IERC20CamelDispatcherTrait};

#[starknet::component]
mod TaxesComponent {
    use dojo::model::{ModelStorage, ModelValueStorage};
    use starknet::{ContractAddress};
    use starknet::info::{get_contract_address, get_block_timestamp};
    use starknet::storage::{Map, StoragePointerReadAccess, StoragePointerWriteAccess};

    use ponzi_land::models::land::Land;
    use ponzi_land::store::{Store, StoreTrait};
    use ponzi_land::components::payable::{PayableComponent, IPayable};
    use ponzi_land::utils::get_neighbors::get_land_neighbors;
    use ponzi_land::utils::common_strucs::TokenInfo;
    use ponzi_land::helpers::taxes::get_taxes_per_neighbor;

    use super::{IERC20CamelDispatcher, IERC20CamelDispatcherTrait};

    #[storage]
    struct Storage {
        // (land_id, neighbor_id) -> last claim time
        last_claim_time: Map<(u16, u16), u64>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {}

    #[generate_trait]
    impl InternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Payable: PayableComponent::HasComponent<TContractState>,
    > of InternalTrait<TContractState> {
        fn claim(ref self: ComponentState<TContractState>, mut store: Store, my_land: Land,) {
            let current_time = get_block_timestamp();
            let neighbors = get_land_neighbors(store, my_land.location);
            let mut payable = get_dep_component_mut!(ref self, Payable);

            for neighbor in neighbors {
                let my_id = my_land.location;
                let neighbor_id = neighbor.location;
                let (a, b) = if my_id < neighbor_id {
                    (my_id, neighbor_id)
                } else {
                    (neighbor_id, my_id)
                };

                let key = (a, b);
                let last_time = self.last_claim_time.read(key);
                let last_time = if last_time == 0 {
                    my_land.block_date_bought
                } else {
                    last_time
                };

                let elapsed_time = current_time - last_time;
                if elapsed_time == 0 {
                    continue;
                }

                let tax_rate = get_taxes_per_neighbor(neighbor);
                let tax_amount = tax_rate * elapsed_time.into();

                if tax_amount > 0 {
                    let token = neighbor.token_used;
                    let validation_result = payable
                        .validate(token, get_contract_address(), tax_amount);
                    let transfer_status = payable.transfer(my_land.owner, validation_result);
                    assert(transfer_status, 'TRANSFER FAILED');
                }

                self.last_claim_time.write(key, current_time);
            }
        }

        fn get_claimable(
            self: @ComponentState<TContractState>, store: Store, my_land: Land,
        ) -> Array<TokenInfo> {
            let current_time = get_block_timestamp();
            let neighbors = get_land_neighbors(store, my_land.location);
            let mut tokens: Array<TokenInfo> = ArrayTrait::new();

            for neighbor in neighbors {
                let my_id = my_land.location;
                let neighbor_id = neighbor.location;
                let (a, b) = if my_id < neighbor_id {
                    (my_id, neighbor_id)
                } else {
                    (neighbor_id, my_id)
                };

                let key = (a, b);
                let last_time = self.last_claim_time.read(key);
                let last_time = if last_time == 0 {
                    my_land.block_date_bought
                } else {
                    last_time
                };

                let elapsed_time = current_time - last_time;
                if elapsed_time == 0 {
                    continue;
                }

                let tax_rate = get_taxes_per_neighbor(neighbor);
                let tax_amount = tax_rate * elapsed_time.into();

                if tax_amount > 0 {
                    tokens
                        .append(
                            TokenInfo { token_address: neighbor.token_used, amount: tax_amount, }
                        );
                }
            };
            tokens
        }
    }
}
