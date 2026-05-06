use std::sync::Arc;
use tokio::time::{sleep, Duration};
use crate::exchange::Exchange;
use crate::exchange::cryptocom::CryptoComExchange;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn run_brain_bot(
    user_id: Uuid,      
    pool: PgPool,       
    exchange: Arc<CryptoComExchange>,
    symbol: String,
) {
    loop {
        if let Ok(price_struct) = exchange.get_price(&symbol).await {
            // on récupère la valeur numérique f64 dans la structure
            let prix_actuel = price_struct.price; 
            println!("📈 Prix actuel de {} : {}", symbol, prix_actuel);

   
            let user_data = sqlx::query("SELECT virtual_balance FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await;

            if let Ok(row) = user_data {
                use sqlx::Row;
                let solde_virtuel: f64 = row.get("virtual_balance");

                if solde_virtuel >= 50.0 {
                    let quantite = 50.0 / prix_actuel; 
                    let nouveau_solde = solde_virtuel - 50.0;

                    // mise à jour du solde
                    let update_result = sqlx::query("UPDATE users SET virtual_balance = $1 WHERE id = $2")
                        .bind(nouveau_solde)
                        .bind(user_id)
                        .execute(&pool)
                        .await;

                    if update_result.is_ok() {
                        println!("✅ [SIMU] Achat réussi : {} unités. Nouveau solde : {:.2} €", quantite, nouveau_solde);
                    }
                } else {
                    println!("⚠️ Solde insuffisant ({:.2} €) pour acheter.", solde_virtuel);
                }
            }
        }

        sleep(Duration::from_secs(60)).await;
    }
}