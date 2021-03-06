pub(super) mod binance_future;
pub(super) mod binance_linear_swap;
pub(super) mod binance_option;
pub(super) mod binance_spot;
mod utils;

use crate::{error::Result, Market, MarketType};

pub(crate) fn fetch_symbols(market_type: MarketType) -> Result<Vec<String>> {
    match market_type {
        MarketType::Spot => binance_spot::fetch_spot_symbols(),
        MarketType::InverseFuture => binance_future::fetch_inverse_future_symbols(),
        MarketType::LinearSwap => binance_linear_swap::fetch_linear_swap_symbols(),
        MarketType::InverseSwap => binance_future::fetch_inverse_swap_symbols(),
        MarketType::Option => binance_option::fetch_option_symbols(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}

pub(crate) fn fetch_markets(market_type: MarketType) -> Result<Vec<Market>> {
    match market_type {
        MarketType::Spot => binance_spot::fetch_spot_markets(),
        MarketType::InverseFuture => binance_future::fetch_inverse_future_markets(),
        MarketType::LinearSwap => binance_linear_swap::fetch_linear_swap_markets(),
        MarketType::InverseSwap => binance_future::fetch_inverse_swap_markets(),
        MarketType::Option => binance_option::fetch_option_markets(),
        _ => panic!("Unsupported market_type: {}", market_type),
    }
}
