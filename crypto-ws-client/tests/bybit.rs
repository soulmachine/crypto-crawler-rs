#[macro_use]
mod utils;

#[cfg(test)]
mod bybit_inverse_swap {
    use crypto_ws_client::{BybitInverseSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe() {
        gen_test_code!(
            BybitInverseSwapWSClient,
            subscribe,
            &vec!["trade.BTCUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_raw_json() {
        gen_test_code!(
            BybitInverseSwapWSClient,
            subscribe,
            &vec![r#"{"op":"subscribe","args":["trade.BTCUSD"]}"#.to_string()]
        );
    }

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BybitInverseSwapWSClient,
            subscribe_trade,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BybitInverseSwapWSClient,
            subscribe_bbo,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BybitInverseSwapWSClient,
            subscribe_orderbook,
            &vec!["BTCUSD".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitInverseSwapWSClient, &vec!["BTCUSD".to_string()], 60);
        gen_test_subscribe_candlestick!(
            BybitInverseSwapWSClient,
            &vec!["BTCUSD".to_string()],
            2592000
        );
    }
}

#[cfg(test)]
mod bybit_linear_swap {
    use crypto_ws_client::{BybitLinearSwapWSClient, WSClient};
    use std::sync::{Arc, Mutex};

    #[test]
    fn subscribe_trade() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_trade,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_bbo() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_bbo,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_orderbook() {
        gen_test_code!(
            BybitLinearSwapWSClient,
            subscribe_orderbook,
            &vec!["BTCUSDT".to_string()]
        );
    }

    #[test]
    fn subscribe_candlestick() {
        gen_test_subscribe_candlestick!(BybitLinearSwapWSClient, &vec!["BTCUSDT".to_string()], 60);
        gen_test_subscribe_candlestick!(
            BybitLinearSwapWSClient,
            &vec!["BTCUSDT".to_string()],
            2592000
        );
    }
}
