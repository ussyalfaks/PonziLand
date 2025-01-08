mod systems {
    mod actions;
}

mod models {
    mod land;
    mod auction;
}

mod helpers {
    mod coord;
}

mod components {
    mod payable;
}

mod consts;
mod store;

mod tokens {
    mod erc20;
    mod main_currency;
}

mod mocks {
    mod erc20;
}


#[cfg(test)]
mod tests {
    mod setup;
    mod actions;
}
