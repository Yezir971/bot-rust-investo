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

    fn generate_signature(&self, id: u64, method: &str, params: &serde_json::Value, nonce: u64) -> String {
        // 1. On construit la chaîne à signer selon le standard Crypto.com v2
        // Format : "method" + "id" + "api_key" + "payload" + "nonce"
        
        let mut sig_payload = String::new();
        sig_payload.push_str(method);
        sig_payload.push_str(&id.to_string());
        sig_payload.push_str(&self.api_key);
        
        // On ajoute les paramètres s'ils existent (triés par clé)
        if let Some(p) = params.as_object() {
            for (key, value) in p {
                sig_payload.push_str(key);
                sig_payload.push_str(&value.to_string());
            }
        }
        
        sig_payload.push_str(&nonce.to_string());

        // 2. On calcule le HMAC-SHA256
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC error");
        mac.update(sig_payload.as_bytes());

        // 3. On encode le résultat en hexadécimal
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

    async fn place_order(&self, symbol: &str, amount: f64) -> Result<String> {
        let url = format!("{}private/create-order", self.base_url);
        let id = 123; // Identifiant unique de requête
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;

        // Paramètres de l'ordre
        let params = serde_json::json!({
            "instrument_name": symbol,
            "side": "BUY",
            "type": "MARKET",
            "quantity": amount,
        });

        // Génération de la signature
        let sig = self.generate_signature(id, "private/create-order", &params, nonce);

        // Construction du corps de la requête JSON
        let body = serde_json::json!({
            "id": id,
            "method": "private/create-order",
            "api_key": self.api_key,
            "params": params,
            "nonce": nonce,
            "sig": sig
        });

        let response = self.client.post(url)
            .json(&body)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // Vérification du code de réponse de l'exchange
        if response["code"] != 0 {
            return Err(anyhow::anyhow!("Erreur API : {}", response["message"]));
        }

        Ok(response["result"]["order_id"].to_string())
    }

    async fn get_solde_current(&self, _asset: &str) -> Result<f64> {
        // todo : appel à private/get-account-summary
        Ok(0.0) 
    }
}