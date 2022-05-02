use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::PoseidonError;

pub type Ed25519Keypair = [u8; 64];
pub type Ed25519Signature = [u8; 64];
pub type Ed25519PublicKey = [u8; 32];

pub type Base58PublicKey = String;
pub type Base58Signature = String;

pub type Base58Sha256Hash = String;
pub type Lamports = u64;
pub type PdaData = Vec<u8>;
pub type Base58BlockHash = String;
pub type Base58EncodedData = String;
pub type UnixTimestamp = i64;
pub type EncryptedData = Vec<u8>;

pub type DataID = [u8; 32];
pub type TokenID = [u8; 32];
pub type SubscriptionID = [u8; 32];
pub type DataOwnedBytes = Vec<u8>;
pub type DataBytes<'a> = &'a [u8];

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd)]
pub enum Cluster {
    MainnetBeta,
    MainnetBetaSerum,
    Testnet,
    Devnet,
}

impl Cluster {
    pub fn url(&self) -> &'static str {
        match self {
            Cluster::MainnetBeta => "https://api.mainnet-beta.solana.com",
            Cluster::MainnetBetaSerum => "https://solana-api.projectserum.com",
            Cluster::Testnet => "https://api.testnet.solana.com",
            Cluster::Devnet => "https://api.devnet.solana.com",
        }
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Cluster::Devnet
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum PoseidonOutcome {
    Success,
    Failure(PoseidonError),
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct UserData {
    pub mime: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, BorshSerialize, BorshDeserialize)]
pub enum AccountOptions {
    /// Creates an account if the account doesn't exist
    CreateIfNone,
    /// Error if the account doesn't exist
    ErrIfNone,
}
