use crate::{
    common::{Coin, Timestamp, WalletAddress},
    transactions::{QuoteId, QuoteTx},
};

/// Test helpers for Block Processor
pub mod block_processor;
/// Test helpers for Vault Node API
pub mod vault_node_api;

/// Test helper for transaction provider
pub mod transaction_provider;

/// Test helper for ethereum
pub mod ethereum;

/// Create a dummy quote transaction to be used for tests
pub fn create_fake_quote_tx() -> QuoteTx {
    let return_address = WalletAddress::new("Alice");
    let input_address = WalletAddress::new("Bob");
    let timestamp = Timestamp::now();

    let quote = QuoteTx {
        id: QuoteId::new(0),
        timestamp,
        input: Coin::LOKI,
        output: Coin::BTC,
        input_address_id: "".to_owned(),
        input_address,
        return_address,
    };

    quote
}

/// Create a fake but a valid looking quote request
pub fn make_valid_quote_request() -> serde_json::Value {
    serde_json::json!({
        "inputCoin": "BTC",
        "inputReturnAddress": "TODO",
        "inputAddressID": "0",
        "inputAmount": "0.5",
        "outputCoin": "LOKI",
        "outputAddress": "TODO",
        "slippageLimit": 0.5,
    })
}

/// Creates a new random file name that (if created)
/// gets removed when this object is destructed
pub struct TempRandomFile {
    path: String,
}

impl TempRandomFile {
    /// Creates a random file name
    pub fn new() -> Self {
        use rand::Rng;

        let rand_filename = format!("temp-{}.db", rand::thread_rng().gen::<u64>());

        TempRandomFile {
            path: rand_filename,
        }
    }

    /// Get the internal file name
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Drop for TempRandomFile {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path)
            .expect(&format!("Could not remove temp file {}", &self.path));
    }
}
