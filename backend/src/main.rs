use axum::{routing::get, Router, Json};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Clone)]
struct Donation {
    donor: String,
    amount: String, // ETH as string (formatted)
    message: String,
    timestamp: u64,
}

#[derive(Clone)]
struct AppState {
    donations: Arc<Mutex<Vec<Donation>>>,
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    let infura_api_key = env::var("INFURA_API_KEY").expect("INFURA_API_KEY must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS must be set");

    let provider = Provider::<Http>::try_from(
        format!("https://sepolia.infura.io/v3/{}", infura_api_key)
    )?.interval(std::time::Duration::from_millis(10));
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(11155111u64); // Sepolia
    let client = SignerMiddleware::new(provider.clone(), wallet);
    let client = Arc::new(client);

    let state = AppState {
        donations: Arc::new(Mutex::new(Vec::new())),
        client,
    };

    // Fetch existing donations
    let donations = fetch_donations(&state.client, &contract_address).await?;
    *state.donations.lock().await = donations;

    let app = Router::new()
        .route("/donations", get(get_donations))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn get_donations(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<Vec<Donation>> {
    let donations = state.donations.lock().await;
    Json(donations.clone())
}

async fn fetch_donations(
    client: &Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    contract_address: &str,
) -> eyre::Result<Vec<Donation>> {
    let contract_address: H160 = contract_address.parse()?;
    abigen!(
        CoffeeDonation,
        r#"[
            function getDonations() public view returns (address[], uint256[], string[], uint256[])
        ]"#
    );
    let contract = CoffeeDonation::new(contract_address, client.clone());
    let (donors, amounts, messages, timestamps): (Vec<H160>, Vec<U256>, Vec<String>, Vec<U256>) = 
        contract.get_donations().call().await?;

    let result = donors
        .into_iter()
        .zip(amounts.into_iter())
        .zip(messages.into_iter())
        .zip(timestamps.into_iter())
        .map(|(((donor, amount), message), timestamp)| Donation {
            donor: format!("{:?}", donor),
            amount: ethers::utils::format_ether(amount),
            message,
            timestamp: timestamp.as_u64(),
        })
        .collect();
    Ok(result)
}