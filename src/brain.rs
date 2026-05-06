use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::exchange::Exchange;

pub async fn run_brain_bot(id: u32, exchange: Arc<dyn Exchange>, symbol: String){
    let mut retry_count = 0;
    println!("Brain bot numéro-{} tourne pour du {} ", id, symbol);
    loop{
        match exchange.get_price(&symbol).await {
            Ok(price) => {
                retry_count=0;
                println!("Bot #{} - {} : {:.2} USDT", id, symbol, price.price);
                if price.price < 50100.0 {
                    let _order = exchange.place_order(&symbol, 0.01).await;
                }
            }
            Err(e) => {
                retry_count += 1;
                println!("BB numéro-{} à une l'erreur suivant : {}", id, e);
                println!("Tentative {}/5 - bot-id : {} - erreur : {}", retry_count , id, e);
                if retry_count > 5  {
                    print!("Trop d'erreurs consécutives. Arrêt du bot {}", id);
                    break;
                }

                // on attend avant de re essayer a nouveau 
                let wait_time = 2u64.pow(retry_count); 
                sleep(Duration::from_secs(wait_time)).await;
                continue;

            }
        }
        sleep(Duration::from_secs(2)).await;
    }

}