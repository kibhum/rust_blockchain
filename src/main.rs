mod blockchain;
mod wallet;
use blockchain::{block_and_blockchain::BlockChain, transaction};
use wallet::wallet::Wallet;

fn main() {
    // let wallet = Wallet::new();
    // println!("Private Key: {}", wallet.private_key_str());
    // println!("Public Key: {}", wallet.public_key_str());
    // println!("Address: {}", wallet.get_address());

    // let transaction = wallet.sign_transaction(&"0x1234567890".to_string(), 100);
    // println!("Transaction: {:?}", transaction);
    // println!(
    //     "Verify Transaction: {}",
    //     Wallet::verify_transaction(&transaction)
    // );

    let miner_wallet = Wallet::new();
    let a_wallet = Wallet::new();
    let b_wallet = Wallet::new();

    let tx_a_b = a_wallet.sign_transaction(&b_wallet.get_address(), 100);
    let mut blockchain = BlockChain::new(miner_wallet.get_address());
    let is_tx_added = blockchain.add_transaction(&tx_a_b);
    println!("{}", is_tx_added);
    blockchain.mining();
    blockchain.print();
    println!(
        "A: {:?}\n",
        blockchain.calculate_total_amount(a_wallet.get_address())
    );
    println!(
        "B: {:?}\n",
        blockchain.calculate_total_amount(b_wallet.get_address())
    );
    println!(
        "Miner: {:?}\n",
        blockchain.calculate_total_amount(miner_wallet.get_address())
    );
}
