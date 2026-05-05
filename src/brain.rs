use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::exchange::Exchange;

pub async fn run_brain_bot(id: u32, exchange: Arc<dyn Exchange>, symbol: String){
    println!("Brain bot numéro-{} tourne pour du {} ", id, symbol);
    loop{
        match exchange.get_price(&symbol).await {
            Ok(price) => {
                println!("Bot #{} - {} : {:.2} USDT", id, symbol, price.price);
                if price.price < 50100.0 {
                    let _order = exchange.place_order(&symbol, 0.01).await;
                }
            }
            Err(e) => println!("BB numéro-{} à une l'erreur suivant : {}", id, e),
        }
        sleep(Duration::from_secs(2)).await;
    }

}