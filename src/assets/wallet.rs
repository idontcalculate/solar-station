use ethers::prelude::*;
use ethers::signers::{LocalWallet, WalletError};
use ethers::providers::{Provider, Http};
use std::str::FromStr;
use std::convert::TryFrom;
use std::sync::Arc;

// Function to create a new wallet with a randomly generated private key
pub fn create_wallet() -> LocalWallet {
    let mut rng = rand::thread_rng();
    LocalWallet::new(&mut rng)
}

// Function to import a wallet using a private key
pub fn import_wallet(private_key: &str) -> Result<LocalWallet, WalletError> {
    LocalWallet::from_str(private_key)
}

// Function to retrieve the wallet's public address
pub fn get_wallet_address(wallet: &LocalWallet) -> Address {
    wallet.address()
}

// Function to check the wallet balance
pub async fn get_wallet_balance(wallet: &LocalWallet, provider_url: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    let provider = Arc::new(provider);
    let balance = provider.get_balance(wallet.address(), None).await?;
    Ok(balance)
}
