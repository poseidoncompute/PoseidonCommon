use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Serialize, BorshSerialize, BorshDeserialize,
)]
pub enum PoseidonErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    NotConnected,
    ConnectionAborted,
    AddrInUse,
    AddrNotAvailable,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    BrokenPipe,
    TimedOut,
    WriteZero,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
    Unspecified(String),
}

impl From<std::io::Error> for PoseidonErrorKind {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => PoseidonErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied => PoseidonErrorKind::PermissionDenied,
            std::io::ErrorKind::ConnectionRefused => PoseidonErrorKind::ConnectionRefused,
            std::io::ErrorKind::ConnectionReset => PoseidonErrorKind::ConnectionReset,
            std::io::ErrorKind::ConnectionAborted => PoseidonErrorKind::ConnectionAborted,
            std::io::ErrorKind::NotConnected => PoseidonErrorKind::NotConnected,
            std::io::ErrorKind::AddrInUse => PoseidonErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable => PoseidonErrorKind::AddrNotAvailable,
            std::io::ErrorKind::BrokenPipe => PoseidonErrorKind::BrokenPipe,
            std::io::ErrorKind::AlreadyExists => PoseidonErrorKind::AlreadyExists,
            std::io::ErrorKind::WouldBlock => PoseidonErrorKind::WouldBlock,
            std::io::ErrorKind::InvalidInput => PoseidonErrorKind::InvalidInput,
            std::io::ErrorKind::InvalidData => PoseidonErrorKind::InvalidData,
            std::io::ErrorKind::TimedOut => PoseidonErrorKind::TimedOut,
            std::io::ErrorKind::WriteZero => PoseidonErrorKind::WriteZero,
            std::io::ErrorKind::Interrupted => PoseidonErrorKind::Interrupted,
            std::io::ErrorKind::Unsupported => PoseidonErrorKind::Unsupported,
            std::io::ErrorKind::UnexpectedEof => PoseidonErrorKind::UnexpectedEof,
            std::io::ErrorKind::OutOfMemory => PoseidonErrorKind::OutOfMemory,
            std::io::ErrorKind::Other => PoseidonErrorKind::Other,
            _ => {
                PoseidonErrorKind::Unspecified(format!("std::io::Error - `{}`", error.to_string()))
            }
        }
    }
}
