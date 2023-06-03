use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinPair {
    pub symbol : String,
    pub status : String,
    pub baseAsset : String,
    pub quoteAsset : String,
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

