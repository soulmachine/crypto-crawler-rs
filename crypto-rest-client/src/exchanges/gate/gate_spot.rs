use super::super::utils::http_get;
use crate::error::Result;
use std::collections::HashMap;

const BASE_URL: &str = "https://api.gateio.ws/api/v4";

/// The RESTful client for Gate spot market.
///
/// * RESTful API doc: <https://www.gateio.pro/docs/apiv4/zh_CN/index.html#gate-api-v4-spot>
/// * Trading at: <https://www.gateio.pro/cn/trade/BTC_USDT>
pub struct GateSpotRestClient {
    _api_key: Option<String>,
    _api_secret: Option<String>,
}

impl GateSpotRestClient {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        GateSpotRestClient {
            _api_key: api_key,
            _api_secret: api_secret,
        }
    }

    /// Get the latest Level2 snapshot of orderbook.
    ///
    /// Top 100 asks and bids are returned.
    ///
    /// For example: <https://api.gateio.ws/api/v4/spot/order_book?currency_pair=BTC_USDT&limit=100>,
    pub fn fetch_l2_snapshot(symbol: &str) -> Result<String> {
        gen_api!(format!(
            "/spot/order_book?currency_pair={}&limit=100",
            symbol
        ))
    }
}
