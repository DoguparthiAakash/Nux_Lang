// Rust - Blockchain Implementation
// Memory-safe, high-performance blockchain

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
    pub timestamp: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Transaction {
            sender,
            recipient,
            amount,
            timestamp,
            signature: String::new(),
        }
    }
    
    pub fn hash(&self) -> String {
        let data = format!("{}{}{}{}", 
            self.sender, self.recipient, self.amount, self.timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        block.hash = block.calculate_hash();
        block
    }
    
    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{:?}{}{}", 
            self.index, self.timestamp, self.transactions, 
            self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        
        println!("Block mined: {}", self.hash);
    }
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
            mining_reward: 100.0,
        };
        
        blockchain.chain.push(blockchain.create_genesis_block());
        blockchain
    }
    
    fn create_genesis_block(&self) -> Block {
        Block::new(0, Vec::new(), String::from("0"))
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
    
    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.chain.last().unwrap().hash.clone(),
        );
        
        block.mine(self.difficulty);
        self.chain.push(block);
        
        // Reward miner
        self.pending_transactions = vec![
            Transaction::new(
                String::from("system"),
                miner_address,
                self.mining_reward,
            )
        ];
    }
    
    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;
        
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.sender == address {
                    balance -= tx.amount;
                }
                if tx.recipient == address {
                    balance += tx.amount;
                }
            }
        }
        
        balance
    }
    
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.hash != current.calculate_hash() {
                return false;
            }
            
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        
        true
    }
}

// C FFI exports
#[no_mangle]
pub extern "C" fn blockchain_create(difficulty: usize) -> *mut Blockchain {
    Box::into_raw(Box::new(Blockchain::new(difficulty)))
}

#[no_mangle]
pub extern "C" fn blockchain_add_transaction(
    blockchain: *mut Blockchain,
    sender: *const i8,
    recipient: *const i8,
    amount: f64,
) {
    unsafe {
        let chain = &mut *blockchain;
        let sender_str = std::ffi::CStr::from_ptr(sender).to_str().unwrap();
        let recipient_str = std::ffi::CStr::from_ptr(recipient).to_str().unwrap();
        
        chain.add_transaction(Transaction::new(
            sender_str.to_string(),
            recipient_str.to_string(),
            amount,
        ));
    }
}

#[no_mangle]
pub extern "C" fn blockchain_mine(
    blockchain: *mut Blockchain,
    miner_address: *const i8,
) {
    unsafe {
        let chain = &mut *blockchain;
        let miner = std::ffi::CStr::from_ptr(miner_address).to_str().unwrap();
        chain.mine_pending_transactions(miner.to_string());
    }
}

#[no_mangle]
pub extern "C" fn blockchain_get_balance(
    blockchain: *const Blockchain,
    address: *const i8,
) -> f64 {
    unsafe {
        let chain = &*blockchain;
        let addr = std::ffi::CStr::from_ptr(address).to_str().unwrap();
        chain.get_balance(addr)
    }
}

#[no_mangle]
pub extern "C" fn blockchain_is_valid(blockchain: *const Blockchain) -> bool {
    unsafe {
        let chain = &*blockchain;
        chain.is_valid()
    }
}

#[no_mangle]
pub extern "C" fn blockchain_destroy(blockchain: *mut Blockchain) {
    unsafe {
        drop(Box::from_raw(blockchain));
    }
}
