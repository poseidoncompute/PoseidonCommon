use crate::TransactionError;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::PoseidonErrorKind;

#[cfg(feature = "http")]
use crate::{HttpError, JsonError};

#[cfg(feature = "rustls")]
use crate::RustlsError;

pub type PoseidonResult<T> = Result<T, PoseidonError>;

#[derive(
    Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize, BorshDeserialize, BorshSerialize,
)]
pub enum PoseidonError {
    MissingEd25519PublicKey,
    MissingKeypair,
    MissingTxSignature,
    HomeDirectoryNotFound,
    PathIsNotValidUtf8,
    InvalidUtf8(String),
    InvalidBase58Ed25519SecretKey,
    InvalidBase58Ed25519PublicKey,
    InvalidBase58Ed25519Signature,
    RepoCreatePermissionDenied,
    RepoAlreadyExists,
    IoErr(PoseidonErrorKind),
    InvalidByteToUtf8StringConversion,
    InvalidEd25519PublicKeyHex,
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter {
        c: String,
        index: usize,
    },
    /// A hex string's length needs to be even, as two digits correspond to
    /// one byte.
    OddLength,

    /// If the hex string is decoded into a fixed sized container, such as an
    /// array, the hex string's length * 2 has to match the container's
    /// length.
    InvalidStringLength,
    /// The underlying collection no longer exists.
    SledCollectionNotFound(String),
    /// The system has been used in an unsupported way.
    SledUnsupported(String),
    /// An unexpected bug has happened. Please open an issue on github!
    SledReportableBug(String),
    /// Corruption has been detected in the storage file.
    SledCorruption(String),
    AccountNotFound,
    UnableToDeserializeAccountInfo,
    UnableToSerializeTx,
    #[cfg(feature = "rustls")]
    Rustls(RustlsError),
    Tx(TransactionError),
    #[cfg(feature = "http")]
    Http(HttpError),
    #[cfg(feature = "http")]
    Json(JsonError),
    SerdeJson(String),
    Unspecified(String),
}

impl From<hex::FromHexError> for PoseidonError {
    fn from(error: hex::FromHexError) -> Self {
        match error {
            hex::FromHexError::InvalidHexCharacter { c, index } => {
                PoseidonError::InvalidHexCharacter {
                    c: c.to_string(),
                    index,
                }
            }
            hex::FromHexError::OddLength => PoseidonError::OddLength,
            hex::FromHexError::InvalidStringLength => PoseidonError::InvalidStringLength,
        }
    }
}

impl From<std::io::Error> for PoseidonError {
    fn from(error: std::io::Error) -> Self {
        PoseidonError::IoErr(error.into())
    }
}

#[cfg(feature = "serde_json")]
impl From<serde_json::Error> for PoseidonError {
    fn from(error: serde_json::Error) -> Self {
        PoseidonError::SerdeJson(error.to_string())
    }
}
