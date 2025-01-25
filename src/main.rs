mod blockchain;
use blockchain::{
    block_and_blockchain::{Block, BlockChain, BlockSearch, BlockSearchResult, Serialization},
    transaction::Transaction,
};

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("Found Block: {:?}", block);
        }

        BlockSearchResult::FailOfIndex(index) => {
            println!("Failed to find block with index: {index}")
        }

        BlockSearchResult::FailOfEmptyBlocks => {
            println!("The block is empty")
        }

        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("Block with hash: {:?} doesn't exist", hash);
        }

        BlockSearchResult::FailOfNonce(nonce) => {
            println!("Failed to find block with nonce: {nonce}")
        }

        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("Block with previous hash: {:?} doesn't exist", hash);
        }

        BlockSearchResult::FailOfTimestamp(timestamp) => {
            println!("Failed to find block with timestamp: {timestamp}")
        }

        BlockSearchResult::FailOfTransaction(transaction) => {
            println!("Block with transaction: {:?} doesn't exist", transaction);
        }
    }
}
fn main() {
    let my_blockchain_address = "my blockchain address";
    let mut blockchain = BlockChain::new(my_blockchain_address.into());

    let tx = Transaction::new("A".into(), "B".into(), 1);
    blockchain.add_transaction(&tx);

    blockchain.mining();
    blockchain.print();

    let tx = Transaction::new("C".into(), "D".into(), 2);
    blockchain.add_transaction(&tx);
    let tx = Transaction::new("X".into(), "Y".into(), 3);
    blockchain.add_transaction(&tx);

    blockchain.mining();
    blockchain.print();

    println!(
        "Miner value: {}",
        blockchain.calculate_total_amount(my_blockchain_address.to_string())
    );
    println!(
        "A value: {}",
        blockchain.calculate_total_amount("A".to_string())
    );
    println!(
        "B value: {}",
        blockchain.calculate_total_amount("B".to_string())
    );
    println!(
        "C value: {}",
        blockchain.calculate_total_amount("C".to_string())
    );
    println!(
        "D value: {}",
        blockchain.calculate_total_amount("D".to_string())
    );
    println!(
        "X value: {}",
        blockchain.calculate_total_amount("X".to_string())
    );
    println!(
        "Y value: {}",
        blockchain.calculate_total_amount("Y".to_string())
    );
}
