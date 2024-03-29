use crate::api::controller::controller;
use crate::dbs::mongodb::connect_mongodb;
use crate::dbs::redis::connect_redis;
use crate::models::actix;
use crate::models::coin_pairs::CoinPair;
use crate::ws::binance_ws;
use actix_web::{middleware::Logger, web, App, HttpServer};
use bson::doc;
use futures::executor::block_on;
use redis::{self, Commands};
use std::sync::{Arc, Mutex};
pub async fn start_price_agg() -> Result<(), std::io::Error> {
    let redis_client: Arc<Mutex<redis::Connection>> = Arc::new(Mutex::new(connect_redis()));
    let mongo_client: Arc<Mutex<mongodb::Database>> = Arc::new(Mutex::new(
        connect_mongodb()
            .await
            .expect("failed to get db connection"),
    ));

    let coin_pairs_col: mongodb::Collection<CoinPair> = mongo_client
        .lock()
        .unwrap()
        .collection::<CoinPair>("coin_pairs");

    let _: () = redis_client.lock().unwrap().del("coin_price").unwrap();

    let mut coin_pairs_data = coin_pairs_col
        .find(
            doc! {
                "status" : "TRADING",
                // "baseAsset" : "ETH",
                "quoteAsset" : "USDT"
                // "symbol" : "BTCUSDT"
            },
            None,
        )
        .await
        .expect("error in finding active coin pairs");

    let mut m: Vec<CoinPair> = Vec::new();

    while coin_pairs_data
        .advance()
        .await
        .ok()
        .expect("Error mapping through data")
    {
        let temp_data = coin_pairs_data
            .deserialize_current()
            .ok()
            .expect("Error mapping through data");

        m.push(temp_data)
    }

    block_on(async move {
        for coin_pair_data in m {
            println!("{:?}", coin_pair_data);
            let redis_client = Arc::clone(&redis_client);
            tokio::spawn(async move {
                let _ = binance_ws::connect_binance_socket(
                    &coin_pair_data.baseAsset,
                    &coin_pair_data.symbol,
                    &redis_client,
                )
                .await;
            });
        }
        let arc_redis_client = Arc::clone(&redis_client.clone());
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(actix::AppConnection {
                    mongo: mongo_client.clone(),
                    redis: arc_redis_client.clone(),
                }))
                .wrap(Logger::default())
                .service(web::resource("/list").route(web::get().to(controller::get_coin_price_list)))
                .service(web::resource("/health-check").route(web::get().to(controller::health_check)))
        })
        .bind(("127.0.0.1", 8080))
        .expect("failed to start http server")
        .workers(1)
        .run()
        .await
        .expect("error in starting server");
    });
    Ok(())
}
