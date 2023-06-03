use std::{env, sync::Mutex};

use futures_util::{future, pin_mut, StreamExt};
use redis::Commands;
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
extern crate redis;

use crate::models::trade_history::TradeEventResponse;

pub async fn connect_binance_socket<'a>(
    coin_code: &'a String,
    pair_name: &'a String,
    redis_client: &Mutex<redis::Connection>
) -> Result<(), std::io::Error> {
    let connect_addr = format!(
        "{}/stream?streams={}@trade",
        env::var("BINANCE_WS_BASE_URL").unwrap(),
        pair_name.to_lowercase()
    );

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    println!("connecting ws {url} server");
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to the server");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);

    let ws_to_stdout = {
        read.fold(redis_client, |redis_client, message| async move {
            match message {
                Ok(Message::Ping(_)) => {}
                Ok(Message::Text(text)) => {
                    let data = text.to_string();

                    let parsed: TradeEventResponse;
                    match serde_json::from_str::<TradeEventResponse>(&data) {
                        Ok(m) => {
                            parsed = m;
                            // println!("{:?}", parsed.data);

                            let _: () = redis_client.lock().unwrap()
                                .hset(
                                    "coin_price",
                                    coin_code,
                                    serde_json::to_string(&parsed.data).unwrap(),
                                )
                                .unwrap();
                        }
                        Err(_) => {}
                    }
                }
                _ => {}
            }
            redis_client
        })
    };
    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
    Ok(())
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
