use std::time::Duration;
use chrono::Utc;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Small example of a blockchain

// The variables below can be used to control how fast the chain is allowed to 
// add blocks, and how much it logs to the terminal. 

// Number of zeros required in hash to allow new block on chain, higher numbers
// mean harder to find a successful block
const ACCEPT_SIZE: usize = 2;

// Milliseconds of sleep when generating a new nonce, higher values mean slower 
// chain generation.
const NONCE_GENERATION_DELAY: u64 = 40;

// How often to print the generated nonces to the terminal while searching for 
// a new nonce. Higher values means fewer logs
const NONCE_LOGGING_MODULO: u64 = 50;


fn main() {
    let mut nonce_generator = NonceGenerator::new(ACCEPT_SIZE);
    let mut blockchain = Blockchain::new();

    loop {
        let prev = blockchain.chain.last()
            .expect("Expected at least one block");
        let new_block = Block::create_random(&prev, &mut nonce_generator);

        blockchain.add_block(new_block);
    }
}


#[derive(Debug,Serialize,Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount: f64
}

#[derive(Debug,Serialize,Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    prev_hash: String,
    nonce: u64
}

impl Block {
    #[allow(dead_code)] 
    fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String, nonce: u64) -> Self {
        Self {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            prev_hash,
            nonce
        }
    }

    fn calculate_hash(&self) -> String {
        let data = serde_json::to_string(self)
            .expect("Expected successful conversion to json");

        let data = Sha256::digest(data);

        format!("{:x}", data)
    }

    fn create_random(prev: &Block, nonce_generator: &mut NonceGenerator) -> Self {
        let mut rng = thread_rng();
        let names = ["Alice".to_string(), "Bob".to_string()];

        let num_transactions = rng.gen_range(1..10);
        let transactions: Vec<Transaction> = (0..num_transactions)
            .map(|_| {
                let i = rng.gen_range(0..=1);
                let j = (i + 1) % 2;
                Transaction {
                    from: names[i as usize].clone(),
                    to: names[j as usize].clone(),
                    amount: rng.gen_range(1..1000) as f64,

                }
            })
            .collect();

        let mut new_block = Self {
            index: prev.index + 1,
            timestamp: Utc::now().timestamp(),
            transactions,
            prev_hash: prev.calculate_hash(),
            nonce: 0,
        };

        nonce_generator.set_nonce(&mut new_block);
        new_block
    }
}

struct Blockchain {
    chain: Vec<Block>
}

impl Blockchain {
    fn new() -> Self {
        let initial_block = Block {
            index: 1,
            timestamp: Utc::now().timestamp(),
            transactions: vec![],
            prev_hash: "From Zero".to_string(),
            nonce: 0,
        };

        Self {
            chain: vec![initial_block]
        }
    }

    fn add_block(&mut self, block: Block) {
        println!("Adding block to Blockchain: {:#?}", block);
        self.chain.push(block);
    }
}

struct NonceGenerator {
    current_nonce: u64,
    accept_size: usize
}

impl NonceGenerator {
    fn new(accept_size: usize) -> Self {
        Self {
            current_nonce: 0,
            accept_size,
        }
    }

    fn set_nonce(&mut self, block: &mut Block) {
        let mut i = 0;
        loop {
            block.nonce = self.current_nonce;
            let json_block = serde_json::to_string(&block)
                .expect("Expected json");

            let hash = Sha256::digest(json_block);
            let hash = format!("{:x}", hash);

            if i % NONCE_LOGGING_MODULO == 0 {
                println!("Iteration {}, found hash: {:#?}",i, hash);
            }

            if hash.chars().take(self.accept_size).all(|c| c == '0') {
                println!("Hash found on iteration {}: {:#?}",i, hash);
                break;
            }

            self.current_nonce += 1;
            i += 1;

            if NONCE_GENERATION_DELAY > 0 {
                std::thread::sleep(Duration::from_millis(NONCE_GENERATION_DELAY));
            }
        }
    }
}
