use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.gateio.ws/api/v4";

/// The RESTful client for Gate Swap markets.
///
/// * RESTful API doc: <https://www.gateio.pro/docs/apiv4/zh_CN/index.html#gate-api-v4-futures>
/// * Trading at: <https://www.gateio.pro/cn/futures_trade/USDT/BTC_USDT>
pub struct GateSwapRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl GateSwapRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GateSwapRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 50 asks and bids are returned.
    ///
    /// For example:
    ///
    /// - <https://api.gateio.ws/api/v4/futures/btc/order_book?contract=BTC_USD&limit=50>
    /// - <https://api.gateio.ws/api/v4/futures/usdt/order_book?contract=BTC_USDT&limit=50>
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        let settle = if symbol.ends_with("_USD") {
            "btc"
        } else if symbol.ends_with("_USDT") {
            "usdt"
        } else {
            panic!("Unknown symbol {}", symbol);
        };
        gen_api!(format!(
            "/futures/{}/order_book?contract={}&limit=50",
            settle, symbol
        ))
    }
}
