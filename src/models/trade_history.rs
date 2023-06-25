use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f64>().map_err(de::Error::custom)
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeHistory {
    #[serde(deserialize_with = "de_float_from_str")]
    p: f64, //trade price
    E: i128, // event time
    #[serde(deserialize_with = "de_float_from_str")]
    q: f64,
    T: i128, // event time
}

#[derive(Debug, Deserialize, Clone)]
pub struct TradeEventResponse {
    pub data: TradeHistory,
}
// {
//     "e": "trade",     // Event type
//     "E": 123456789,   // Event time
//     "s": "BNBBTC",    // Symbol
//     "t": 12345,       // Trade ID
//     "p": "0.001",     // Price
//     "q": "100",       // Quantity
//     "b": 88,          // Buyer order ID
//     "a": 50,          // Seller order ID
//     "T": 123456785,   // Trade time
//     "m": true,        // Is the buyer the market maker?
//     "M": true         // Ignore
// }

#[derive(Debug, Deserialize, Clone)]
pub struct AllCoinPrice {
    pub data: HashMap<String, TradeHistory>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinPrice {
    p: f64, //trade price
    T: i128, // event time
}