#[macro_use]
mod utils;

#[cfg(test)]
mod mxc_spot {
    use crypto_ws_client::{MXCSpotWSClient, Trade, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(MXCSpotWSClient, &vec!["symbol:BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(MXCSpotWSClient, &vec!["BTC_USDT".to_string()]);
    }
}

#[cfg(test)]
mod mxc_swap {
    use crypto_ws_client::{MXCSwapWSClient, Trade, WSClient};

    #[test]
    fn subscribe() {
        gen_test_subscribe!(MXCSwapWSClient, &vec!["deal:BTC_USDT".to_string()]);
    }

    #[test]
    fn subscribe_trade() {
        gen_test_subscribe_trade!(MXCSwapWSClient, &vec!["BTC_USDT".to_string()]);
    }
}
