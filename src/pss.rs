use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PSSProtocol {
    FetchTransaction(String),
    Account(String),
    SendTx(String),
}
