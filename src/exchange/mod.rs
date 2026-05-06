use async_trait::async_trait;
use anyhow::Result;

pub mod mock;
// pub use mock::*;

pub mod cryptocom;
// pub use cryptocom::*;

#[derive(Debug)]
pub struct Price{
    pub symbol: String,
    pub price: f64,
}

#[async_trait]
pub trait Exchange: Send + Sync {
    async fn get_price(&self, symbol: &str) -> Result<Price>;
    
    async fn place_order(&self, symbol: &str, amount: f64) -> Result<String>;

    async fn get_solde_current(&self, asset:&str)-> Result<f64>; 
}

