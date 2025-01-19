pub mod blockchain;
use blockchain::BlockChain;
fn main() {
    let mut block_chain = BlockChain::new();

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);

    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(1, previous_hash);
    block_chain.print();
}
