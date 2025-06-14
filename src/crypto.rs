use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey, signature::Signer};
use k256::Secp256k1;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Singnature(ECDSASignature<Secp256k1>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PrivateKey(SigningKey<Secp256k1>);
