mod exchange;
mod brain;

use dotenvy::dotenv;


use std::sync::Arc;
use exchange::mock::MockExchange;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let base_url_cryptocom = std::env::var("BASE_URL_CRYPTOCOM").unwrap();

    let exchange = CryptoComExchange::new(base_url_cryptocom);

    // let mock_exchange = Arc::new(MockExchange);

    // // On lance 3 instances de bots différentes en parallèle !
    // let bot1 = tokio::spawn(brain::run_brain_bot(1, mock_exchange.clone(), "BTC_USDT".to_string()));
    // let bot2 = tokio::spawn(brain::run_brain_bot(2, mock_exchange.clone(), "ETH_USDT".to_string()));
    // let bot3 = tokio::spawn(brain::run_brain_bot(3, mock_exchange.clone(), "CRO_USDT".to_string()));

    // // On attend qu'ils finissent (c'est-à-dire jamais ici)
    // let _ = tokio::join!(bot1, bot2, bot3);
}