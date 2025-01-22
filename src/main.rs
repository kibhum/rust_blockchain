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
    let mut block_chain = BlockChain::new();

    let previous_hash = block_chain.last_block().hash();
    let hash_to_find = previous_hash.clone();

    block_chain.create_block(1, previous_hash);

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);

    let tx = Transaction::new(
        "sender".as_bytes().to_vec(),
        "recipient".as_bytes().to_vec(),
        100,
    );
    block_chain.add_transaction(&tx);
    // println!("{}", tx);

    // println!("Transaction before serialization: {:?}", tx);
    // let tx_bin = tx.serialization();
    // println!("Transaction after serialization: {:?}", tx_bin);
    // let tx_des = Transaction::deserialization(tx_bin);
    // println!("Transaction after deserialization: {:?}", tx_des);

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);
    block_chain.print();

    let result = block_chain.search_block(BlockSearch::SearchByIndex(1));
    get_block_search_result(result);

    let result = block_chain.search_block(BlockSearch::SearchByIndex(5));
    get_block_search_result(result);

    let result = block_chain.search_block(BlockSearch::SearchByBlockHash(hash_to_find));
    get_block_search_result(result);
}
