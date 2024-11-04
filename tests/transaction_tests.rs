use ethers::types::Address;
use solarpunk::assets::transactions::{
    setup_wallet_and_provider, send_eth_transaction, get_balance,
    check_transaction_status, estimate_gas_for_transaction
};
use std::str::FromStr;
use tokio;

#[tokio::test]
async fn test_get_balance() {
    let provider_url = "https://mainnet.infura.io/v3/YOUR_INFURA_KEY";
    let wallet_address = Address::from_str("YOUR_WALLET_ADDRESS").unwrap();
    let balance = get_balance(wallet_address, provider_url).await.unwrap();
    println!("Wallet balance: {:?}", balance);
}

#[tokio::test]
async fn test_send_transaction() {
    let provider_url = "https://mainnet.infura.io/v3/YOUR_INFURA_KEY";
    let private_key = "YOUR_PRIVATE_KEY";
    let wallet = setup_wallet_and_provider(private_key, provider_url).await.unwrap();
    let recipient = Address::from_str("RECIPIENT_WALLET_ADDRESS").unwrap();
    let tx_hash = send_eth_transaction(wallet, recipient, 0.01).await.unwrap();
    println!("Transaction sent with hash: {:?}", tx_hash);
}

#[tokio::test]
async fn test_check_transaction_status() {
    let provider_url = "https://mainnet.infura.io/v3/YOUR_INFURA_KEY";
    let tx_hash = TxHash::from_str("SOME_TX_HASH").unwrap();
    let status = check_transaction_status(tx_hash, provider_url).await.unwrap();
    println!("Transaction status: {:?}", status);
}

#[tokio::test]
async fn test_estimate_gas() {
    let provider_url = "https://mainnet.infura.io/v3/YOUR_INFURA_KEY";
    let private_key = "YOUR_PRIVATE_KEY";
    let wallet = setup_wallet_and_provider(private_key, provider_url).await.unwrap();
    let recipient = Address::from_str("RECIPIENT_WALLET_ADDRESS").unwrap();
    let gas_estimate = estimate_gas_for_transaction(wallet, recipient, 0.01).await.unwrap();
    println!("Estimated gas: {:?}", gas_estimate);
}
