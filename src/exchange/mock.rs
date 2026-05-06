use async_trait::async_trait;
use anyhow::Result;
use rand::Rng;
use super::{Exchange, Price};


pub struct MockExchange;

#[async_trait]
impl Exchange for MockExchange{
    async fn get_price(&self, symbol: &str) -> Result<Price>{
        let mut rng = rand::thread_rng();
        let fake_price = rng.gen_range(50000.0..51000.0);
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