use std::{collections::{BTreeMap,HashMap} };

use crate::models::{
    actix,
    coin_pairs::CoinNameQuery,
    trade_history::{ CoinPrice},
};
use actix_web::{web, Error, HttpResponse, Result};
use redis::Commands;

pub async fn health_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("healthy"))
}

pub async fn get_coin_price_list(
    connection: web::Data<actix::AppConnection>,
    params: web::Query<CoinNameQuery>,
) -> Result<HttpResponse, Error> {
    let redis_client = &connection.redis;

    let mut data: Vec<HashMap<String,CoinPrice>> = Vec::new();

    if params.coin_name.is_some() {
        let coin_name: &String = &params.coin_name.as_ref().unwrap();
        let result: String = redis_client
            .lock()
            .unwrap()
            .hget("coin_price", coin_name)
            .expect("failed to get coin price in redis");

        match serde_json::from_str::<CoinPrice>(&result) {
            Ok(m) => {
                let mut hm = HashMap::new();
                hm.insert(coin_name.into(), m);
                data.push(hm);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    } else {
        let result: BTreeMap<String, String> = redis_client
            .lock()
            .unwrap()
            .hgetall("coin_price")
            .expect("failed to get coin price in redis");

        for (coin_name,coin_price) in result.iter() {
            match serde_json::from_str::<CoinPrice>(&coin_price) {
                Ok(m) => {
                    let mut hm = HashMap::new();
                    hm.insert(coin_name.clone(), m);
                    data.push(hm);
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
    }


    Ok(HttpResponse::Ok().json(data))
}
