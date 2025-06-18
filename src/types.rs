use std::collections::HashMap;

use crate::{
    U256,
    crypto::{PublicKey, Signature},
    error::{BtcError, Result},
    sha256::Hash,
    util::MerkleRoot,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transactions: transactions,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }

    pub fn verify_transactions(&self, utxos: &HashMap<Hash, TransactionOutput>) -> Result<()> {
        if self.transactions.is_empty() {
            return Err(BtcError::InvalidTransaction);
        }

        use std::collections::HashSet;

        for transaction in &self.transactions {
            let mut input_value = 0;
            let mut output_value = 0;
            let mut seen_inputs = HashSet::new();

            for input in &transaction.inputs {
                let prev_output = utxos.get(&input.prev_transaction_output_hash);

                if prev_output.is_none() {
                    return Err(BtcError::InvalidTransaction);
                }
                let prev_output = prev_output.unwrap();

                if seen_inputs.contains(&input.prev_transaction_output_hash) {
                    return Err(BtcError::InvalidTransaction);
                }
                seen_inputs.insert(input.prev_transaction_output_hash.clone());

                if !input
                    .signature
                    .verify(&input.prev_transaction_output_hash, &prev_output.pubkey)
                {
                    return Err(BtcError::InvalidSignature);
                }
                input_value += prev_output.value;
            }
            for output in &transaction.outputs {
                output_value += output.value;
            }
            if input_value < output_value {
                return Err(BtcError::InvalidTransaction);
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,
    pub nonce: u64,
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    //a number which has to be > hash if this block (difficulty)
    pub target: U256,
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction {
            inputs: inputs,
            outputs: outputs,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,

    //Compressed public keys 33bytes because they have 0x00 prefix
    pub pubkey: PublicKey,
}
impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            utxos: HashMap::new(),
            blocks: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        if self.blocks.is_empty() {
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(BtcError::InvalidBlock);
            }
        } else {
            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash() {
                println!("prev hash is wrong");
                return Err(BtcError::InvalidBlock);
            }
            if !block.header.hash().matches_target(block.header.target) {
                print!("didn't not match target");
                return Err(BtcError::InvalidBlock);
            }

            let caluclated_merkle_root = MerkleRoot::calculate(&block.transactions);
            if caluclated_merkle_root != block.header.merkle_root {
                println!("invalid merkle root");
                return Err(BtcError::InvalidMerkleRoot);
            }

            if block.header.timestamp <= last_block.header.timestamp {
                return Err(BtcError::InvalidBlock);
            }
            block.verify_transactions(&self.utxos)?;
        }
        self.blocks.push(block);
        Ok(())
    }

    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }
                for output in transaction.outputs.iter() {
                    self.utxos.insert(transaction.hash(), output.clone());
                }
            }
        }
    }
}
