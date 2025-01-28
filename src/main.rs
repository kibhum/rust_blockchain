mod blockchain;
mod wallet;
use blockchain::transaction;
use wallet::wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("Private Key: {}", wallet.private_key_str());
    println!("Public Key: {}", wallet.public_key_str());
    println!("Address: {}", wallet.get_address());

    let transaction = wallet.sign_transaction(&"0x1234567890".to_string(), 100);
    println!("Transaction: {:?}", transaction);
    println!(
        "Verify Transaction: {}",
        Wallet::verify_transaction(&transaction)
    );
}
