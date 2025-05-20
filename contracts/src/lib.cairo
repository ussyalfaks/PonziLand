mod systems {
    mod actions;
    mod auth;
    mod token_registry;
}

mod interfaces {
    mod systems;
}

mod models {
    mod auction;
    mod land;
    mod message;
}

mod helpers {
    mod circle_expansion;
    mod coord;
    mod taxes;
}

mod components {
    mod payable;
    mod stake;
    mod taxes;
}

mod consts;
mod store;

mod tokens {
    mod erc20;
    mod main_currency;
}

mod mocks {
    mod ekubo_core;
    mod erc20;
}

mod utils {
    mod common_strucs;
    mod get_neighbors;
    mod level_up;
    mod spiral;
    mod stake;
}

#[cfg(test)]
mod tests {
    mod actions;
    mod setup;
}
