use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, PartialOrd, Eq, Ord, Deserialize, Serialize, BorshDeserialize, BorshSerialize,
)]
pub enum StoreErr {
    PermissionDenied,
    StoreNotFound(String),
    EntryExists,
    DeletionErr(String),
    UpdateError(String),
    UpsertError(String),
    RepoPermissionDenied,
    RepoNotFound,
    SubscribeError,
}
