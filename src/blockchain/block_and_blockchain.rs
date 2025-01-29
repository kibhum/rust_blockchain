use crate::blockchain::transaction::*;
use crate::wallet::wallet::{Transaction as WalletTransaction, Wallet};
use sha2::{Digest, Sha256};
use std::cmp::PartialEq;
use std::ops::{AddAssign, Index};
use std::time::{Instant, SystemTime};

pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(bytes: Vec<u8>) -> T;
}

pub enum BlockSearch {
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByNonce(i32),
    SearchByTimestamp(u128),
    SearchByTransaction(Vec<u8>),
}

pub enum BlockSearchResult<'a> {
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimestamp(u128),
    FailOfTransaction(Vec<u8>),
}

#[derive(Debug)]
pub struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamp: u128,
    transactions: Vec<Vec<u8>>,
}

impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        let self_hash = self.hash();
        let other_hash = other.hash();
        self_hash == other_hash
    }
}

impl Block {
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
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

    pub fn print(&self) {
        // Formating value as hex
        println!("Timestamp: {:x}", self.time_stamp);
        // Formating value as integer
        println!("Nonce: {}", self.nonce);
        // Using Debug formatter for complex values
        println!("previous_hash: {:?}", self.previous_hash);
        println!("Transactions: {:?}", self.transactions);
        for (idx, tx) in self.transactions.iter().enumerate() {
            let transaction = Transaction::deserialization(tx.to_vec());
            println!("The transaction: {} is number: {}", transaction, idx)
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamp.to_be_bytes());
        for tx in self.transactions.iter() {
            bin.extend(tx);
        }
        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
    blockchain_address: String,
}
impl BlockChain {
    const DIFFICULTY: usize = 4;
    const MINING_SENDER: &str = "THE BLOCKCHAIN";
    const MINING_REWARD: u64 = 1;

    pub fn new(address: String) -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
            blockchain_address: address,
        };
        let b = Block::new(0, vec![0 as u8; 32]);
        bc.chain.push(b);
        bc.mining();
        bc
    }

    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);
        for tx in self.transaction_pool.iter() {
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();
        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b);
        let elapsted_time = now.elapsed();
        println!(
            "Compute time: {:?}\nProof Hash for the current block is:{:?}",
            elapsted_time, proof_hash
        );
        self.chain.push(b);
    }

    pub fn print(&self) {
        for (i, block) in self.chain.iter().enumerate() {
            println!("{} Chain: {} {}", "=".repeat(25), i, "=".repeat(25));
            block.print();
        }
        println!("{}", "*".repeat(25));
    }

    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            return &self.chain[self.chain.len() - 1];
        }
        &self.chain[0]
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if idx == index {
                        return BlockSearchResult::Success(block);
                    } else {
                        return BlockSearchResult::FailOfIndex(index);
                    }
                }

                BlockSearch::SearchByPreviousHash(ref hash) => {
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    } else {
                        return BlockSearchResult::FailOfPreviousHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    } else {
                        return BlockSearchResult::FailOfBlockHash(hash.to_vec());
                    }
                }

                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    } else {
                        return BlockSearchResult::FailOfNonce(nonce);
                    }
                }

                BlockSearch::SearchByTimestamp(timestamp) => {
                    if block.time_stamp == timestamp {
                        return BlockSearchResult::Success(block);
                    } else {
                        return BlockSearchResult::FailOfTimestamp(timestamp);
                    }
                }

                BlockSearch::SearchByTransaction(ref transaction) => {
                    for tx in block.transactions.iter() {
                        if transaction == tx {
                            return BlockSearchResult::Success(block);
                        } else {
                            return BlockSearchResult::FailOfTransaction(transaction.to_vec());
                        }
                    }
                }
            }
        }

        return BlockSearchResult::FailOfEmptyBlocks;
    }

    pub fn add_transaction(&mut self, tx: &WalletTransaction) -> bool {
        // Making sure we are not sending to ourself
        if tx.sender == self.blockchain_address {
            println!("The miner cannot send money to themselves");
            return false;
        }
        // We should also avoid verify the transaction that is a reward from the blockchain
        if tx.sender != BlockChain::MINING_SENDER && !Wallet::verify_transaction(tx) {
            println!("Invalid transation");
            return false;
        }
        // Making sure the sender has enough balance to send a particular amount of money
        // The sender should also not be the blockchain
        if tx.sender != BlockChain::MINING_SENDER
            && self.calculate_total_amount(tx.sender.clone()) < tx.amount as i64
        {
            println!("The sender doesn't have enough balance to make that transation");
            return false;
        }

        let transaction = Transaction::new(
            tx.sender.as_bytes().to_vec(),
            tx.recipient.as_bytes().to_vec(),
            tx.amount,
        );
        for tx_in_pool in self.transaction_pool.iter() {
            if *tx_in_pool == transaction.serialization() {
                break;
            }
        }
        self.transaction_pool.push(transaction.serialization());
        true
    }

    fn do_proof_of_work(block: &mut Block) -> String {
        loop {
            let hash = block.hash();
            let hash_str = hex::encode(&hash);
            if hash_str[0..BlockChain::DIFFICULTY] == "0".repeat(BlockChain::DIFFICULTY) {
                return hash_str;
            }
            *block += 1;
        }
    }

    pub fn mining(&mut self) -> bool {
        let tx = WalletTransaction {
            sender: BlockChain::MINING_SENDER.into(),
            recipient: self.blockchain_address.clone().into(),
            amount: 20,
            public_key: String::new(),
            signature: String::new(),
        };
        self.add_transaction(&tx);
        self.create_block(0, self.last_block().hash());
        true
    }

    pub fn calculate_total_amount(&self, address: String) -> i64 {
        let mut total_amount: i64 = 0;
        for i in 0..self.chain.len() {
            let block = &self[i];
            for t in block.transactions.iter() {
                let tx = Transaction::deserialization(t.to_vec());
                let value = tx.value;

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.recipient_address {
                    total_amount += value as i64;
                }

                if <String as Into<Vec<u8>>>::into(address.clone()) == tx.sender_address {
                    total_amount -= value as i64;
                }
            }
        }

        total_amount
    }
}

impl Index<usize> for BlockChain {
    type Output = Block;
    fn index(&self, index: usize) -> &Self::Output {
        let res = self.chain.get(index);
        match res {
            Some(block) => {
                return block;
            }
            None => {
                panic!("Index out of range");
            }
        }
    }
}
