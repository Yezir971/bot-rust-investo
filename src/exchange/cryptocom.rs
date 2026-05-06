use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use super::{Exchange, Price};



pub struct CryptoComExchange {
    api_key: String,
    api_secret: String,
    client: Client,
    base_url: String,
}

impl CryptoComExchange{
    pub fn new(api_key: String, api_secret: String, url_brocker: String)-> Self {
        Self {
            api_key,
            api_secret,
            client: Client::new(),
            base_url: url_brocker,
        }
    }
    // fonction pour créer les signatures numérique exigé par crypto.com 
    fn sign_request(&self, method: &str, nonce: u64) -> String {
        // La signature chez Crypto.com est un condensé HMAC-SHA256 de la concaténation de la méthode, de l'id, de l'api_key, etc.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC peut prendre n'importe quelle taille de clé");
        let payload = format!("{}{}{}",method, self.api_key, nonce );
        mac.update(payload.as_bytes());

        hex::encode(mac.finalize().into_bytes())
    }
}

#[async_trait]
impl Exchange for CryptoComExchange {
    // async fn get_price(&self, symbol: &str) -> Result<Price>;
    async fn get_price(&self, symbol: &str) -> Result<Price> {
        let url = format!("{}public/get-ticker?instrument_name={}", self.base_url, symbol);
        
        let response = self.client.get(url).send().await?
            .json::<serde_json::Value>().await?;

        // Extraction du prix dans le JSON de réponse de Crypto.com
        let price_str = response["result"]["data"][0]["a"] // "a" pour le 'ask' price
            .as_str()
            .unwrap_or("0.0");

        Ok(Price {
            symbol: symbol.to_string(),
            price: price_str.parse::<f64>()?,
        })
    }

    async fn place_order(&self, symbol: &str, _amount: f64) -> Result<String> {
        // c'est ici que la signature intervient pour un ordre réel
        println!("🔐 Signature de l'ordre pour {}...", symbol);
        // todo : appel POST à private/create-order
        Ok("real_order_id_from_exchange".to_string())
    }

    async fn get_solde_current(&self, _asset: &str) -> Result<f64> {
        // todo : appel à private/get-account-summary
        Ok(0.0) 
    }
}