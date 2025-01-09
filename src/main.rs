use std::time::SystemTime;

#[derive(Debug)]
struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

impl Block {
    fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Self {
            nonce,
            previous_hash,
            time_stamp: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(),
        }
    }

    fn print(&self) {
        // Formating value as hex
        println!("Timestamp: {:x}", self.time_stamp);
        // Formating value as integer
        println!("Nonce: {}", self.nonce);
        // Using Debug formatter for complex values
        println!("Transactions: {:?}", self.transactions);
        println!("previous_hash: {:?}", self.previous_hash);
    }
}

#[derive(Debug)]
struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
}
impl BlockChain {
    fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };
        bc.create_block(0, "Hash for the first block".to_string().into_bytes());
        bc
    }

    fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let b = Block::new(nonce, previous_hash);
        self.chain.push(b);
    }

    fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Chain: {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }
}

fn main() {
    // Convert a string to a bytes array
    // Convert it to a String, then using into_bytes to a vector array
    // let b = Block::new(0, "This is our first block".to_string().into_bytes());
    // b.print();
    let mut block_chain = BlockChain::new();
    println!("Block chain: {:?}", block_chain);
    block_chain.print();
    block_chain.create_block(1, "Hash 1".to_string().into_bytes());
    block_chain.print();
    block_chain.create_block(2, "Hash 2".to_string().into_bytes());
    block_chain.print();
    block_chain.create_block(3, "Hash 3".to_string().into_bytes());
    block_chain.print();
}
