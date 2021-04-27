// Implements support for the transactions module

use chainflip_common::types::addresses::{Address, EthereumAddress};
use codec::{Codec, Decode, Encode};
use serde::Serialize;
use substrate_subxt::{
    module,
    sp_runtime::{app_crypto::RuntimePublic, traits::Member},
    system::System,
    Event,
};

#[module]
pub trait Transactions: System {}

// Apparently should be an event type here
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode, Serialize)]
pub struct DataAddedEvent<T: Transactions> {
    pub who: <T as System>::AccountId,

    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_raw_data_added() {
        todo!()
    }
}
