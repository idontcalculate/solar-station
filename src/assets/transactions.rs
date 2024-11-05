use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::types::{U256, TxHash, TransactionRequest, Bytes};
use ethers::types::transaction::eip2718::TypedTransaction;
use std::sync::Arc;
use std::str::FromStr;
use k256::ecdsa::SigningKey;

// Setup wallet and provider
pub async fn setup_wallet_and_provider(
    private_key: &str,
    provider_url: &str,
) -> Result<(Wallet<SigningKey>, Arc<Provider<Http>>), Box<dyn std::error::Error>> {
    let provider = Arc::new(Provider::<Http>::try_from(provider_url)?);
    let wallet = Wallet::from_str(private_key)?.with_chain_id(1u64); // Set chain ID to 1 (Ethereum Mainnet)
    Ok((wallet, provider))
}

// Send ETH transaction
pub async fn send_eth_transaction(
    wallet: Wallet<SigningKey>,
    provider: Arc<Provider<Http>>,
    to: Address,
    amount_eth: f64,
) -> Result<TxHash, Box<dyn std::error::Error>> {
    let amount = U256::from((amount_eth * 1e18) as u64);
    let mut tx: TypedTransaction = TransactionRequest::pay(to, amount).into();
    tx.set_chain_id(wallet.chain_id());

    // Sign the transaction and convert to RLP-encoded bytes
    let signature = wallet.sign_transaction(&tx).await?;
    let signed_tx = tx.rlp_signed(&signature);

    let pending_tx = provider.send_raw_transaction(Bytes::from(signed_tx)).await?;
    Ok(pending_tx.tx_hash())
}

// Get Wallet Balance
pub async fn get_balance(
    wallet_address: Address,
    provider: Arc<Provider<Http>>,
) -> Result<U256, Box<dyn std::error::Error>> {
    let balance = provider.get_balance(wallet_address, None).await?;
    Ok(balance)
}

// Fetch Transaction History with explicit type annotation
pub async fn fetch_transaction_history(
    wallet_address: Address,
    provider: Arc<Provider<Http>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let block_number = provider.get_block_number().await?;
    for i in 0..block_number.as_u64() {
        if let Some(block) = provider.get_block_with_txs::<u64>(i.into()).await? {
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
    provider: Arc<Provider<Http>>,
) -> Result<String, Box<dyn std::error::Error>> {
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
    _wallet: Wallet<SigningKey>, // Prefix with an underscore to silence the warning
    provider: Arc<Provider<Http>>,
    to: Address,
    amount_eth: f64,
) -> Result<U256, Box<dyn std::error::Error>> {
    let amount = U256::from((amount_eth * 1e18) as u64);
    let tx: TypedTransaction = TransactionRequest::pay(to, amount).into();
    let gas_estimate = provider.estimate_gas(&tx, None).await?;
    Ok(gas_estimate)
}
