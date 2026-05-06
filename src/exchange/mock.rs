use async_trait::async_trait;
use anyhow::Result;
use rand::Rng;
use super::{Exchange, Price};
use reqwest::Client;
use tokio::time::{Duration};



pub struct MockExchange {
    api_key: String,
    api_secret: String,
    client: Client,
    base_url: String,
}

impl MockExchange {
    pub fn new(api_key: String, api_secret: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Erreur lors de la création du client HTTP");

        Self {
            api_key,
            api_secret,
            client, 
            base_url: "https://api.crypto.com/exchange/v1/".to_string(),
        }
    }
}
    

#[async_trait]
impl Exchange for MockExchange{
    async fn get_price(&self, symbol: &str) -> Result<Price>{
        let mut rng = rand::thread_rng();
        let fake_price = rng.gen_range(0.0..2.0);
        Ok(Price{
            symbol:symbol.to_string(),
            price: fake_price
        })
    }

    async fn place_order(&self, symbol: &str, amount: f64) -> Result<String>{
        println!("ordre d'achat placé : {} de {}", amount, symbol);
        Ok("mock_order_id_123456".to_string())
    }

    async fn get_solde_current(&self, _asset:&str)-> Result<f64>{
        Ok(1000.0)
    }
}