mod blockchain;
mod wallet;
use wallet::wallet::Wallet;

fn main() {
    let wallet = Wallet::new();
    println!("Private Key: {}", wallet.private_key_str());
    println!("Public Key: {}", wallet.public_key_str());
    println!("Address: {}", wallet.get_address());
}
