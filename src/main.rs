use rust_price_aggeregator::startup::start_price_agg;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().unwrap();
    start_price_agg().await
}
