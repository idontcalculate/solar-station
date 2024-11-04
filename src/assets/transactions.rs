use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::types::{U256, TxHash, TransactionRequest};
use std::sync::Arc;
use tokio;
use std::str::FromStr;

// Setup wallet and provider
pub async fn setup_wallet_and_provider(
    private_key: &str,
    provider_url: &str,
) -> Result<Wallet<SigningKey>, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    let wallet = Wallet::from_str(private_key)?.connect(Arc::new(provider));
    Ok(wallet)
}

// Send ETH transaction
pub async fn send_eth_transaction(
    wallet: Wallet<SigningKey>,
    to: Address,
    amount_eth: f64,
) -> Result<TxHash, Box<dyn std::error::Error>> {
    let amount = U256::from((amount_eth * 1e18) as u64);
    let tx = TransactionRequest::pay(to, amount);
    let pending_tx = wallet.send_transaction(tx, None).await?;
    Ok(pending_tx.tx_hash())
}

// Get Wallet Balance
pub async fn get_balance(
    wallet_address: Address,
    provider_url: &str,
) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    let balance = provider.get_balance(wallet_address, None).await?;
    Ok(balance)
}

// Fetch Transaction History
pub async fn fetch_transaction_history(
    wallet_address: Address,
    provider_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    let block_number = provider.get_block_number().await?;
    for i in 0..block_number.as_u64() {
        if let Some(block) = provider.get_block_with_txs(i.into()).await? {
            for tx in block.transactions {
                if tx.from == wallet_address || tx.to == Some(wallet_address) {
                    println!("Transaction found: {:?}", tx);
                }
            }
        }
    }
    Ok(())
}

// Check Transaction Status
pub async fn check_transaction_status(
    tx_hash: TxHash,
    provider_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(provider_url)?;
    if let Some(receipt) = provider.get_transaction_receipt(tx_hash).await? {
        let status = match receipt.status {
            Some(status) if status.as_u64() == 1 => "Success".to_string(),
            Some(status) if status.as_u64() == 0 => "Failed".to_string(),
            _ => "Pending".to_string(),
        };
        Ok(status)
    } else {
        Ok("Pending".to_string())
    }
}

// Estimate Gas for a Transaction
pub async fn estimate_gas_for_transaction(
    wallet: Wallet<SigningKey>,
    to: Address,
    amount_eth: f64,
) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = wallet.provider();
    let amount = U256::from((amount_eth * 1e18) as u64);
    let tx = TransactionRequest::pay(to, amount);
    let gas_estimate = provider.estimate_gas(&tx, None).await?;
    Ok(gas_estimate)
}
