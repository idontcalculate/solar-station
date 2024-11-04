use solarpunk::assets::wallet::{create_wallet, import_wallet, get_wallet_address};
use ethers::types::Address;
//dummy private key just to ensure the code logic is working

#[test]
fn test_create_wallet() {
    // Create a new wallet and retrieve the address
    let wallet = create_wallet();
    let address = get_wallet_address(&wallet);
    
    // Ensure that a valid Ethereum address is generated (non-zero)
    println!("Generated wallet address: {:?}", address);
    assert!(address != Address::zero());
}

#[test]
fn test_import_wallet() {
    // Use a dummy private key for testing (this is not a real key)
    // If someone with a real key wants to test, they can replace this key with an actual valid one
    let dummy_private_key = "4c0883a69102937d6231471b5dbb6204fe512961708279a8e2e560c7b45c0079"; // Example key for testing

    // Attempt to import the wallet using the dummy key
    match import_wallet(dummy_private_key) {
        Ok(wallet) => {
            let address = get_wallet_address(&wallet);
            println!("Imported wallet address: {:?}", address);
            assert!(address != Address::zero()); // Check that the address is valid
        },
        Err(e) => eprintln!("Failed to import wallet: {:?}", e),
    }
}
