use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid Transaction")]
    InvalidTransaction,

    #[error("Invalid Block")]
    InvalidBlock,

    #[error("Invalid Block Header")]
    InvalidBlockHeader,

    #[error("Invalid Transaction input")]
    InvalidTransactionInput,

    #[error("Invalid Transaction output")]
    InvalidTransactionOutput,

    #[error("Invalid Merkle Root")]
    InvalidMerkleRoot,

    #[error("Invalid Hash")]
    InvalidHash,

    #[error("Invalid Signature")]
    InvalidSignature,

    #[error("Invalid Public Key")]
    InvalidPublicKey,

    #[error("Invalid Private key")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, BtcError>;
