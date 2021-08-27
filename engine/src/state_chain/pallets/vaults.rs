use std::marker::PhantomData;

use codec::{Decode, Encode};
use pallet_cf_vaults::{
    rotation::VaultRotationResponse, EthSigningTxRequest, KeygenRequest, VaultRotationRequest,
};
use sp_runtime::AccountId32;
use substrate_subxt::{module, system::System, Call, Event};

use crate::state_chain::{runtime::StateChainRuntime, sc_event::SCEvent};

#[module]
pub trait Vaults: System {}

// The order of these fields matter for decoding
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode, Encode)]
pub struct KeygenRequestEvent<V: Vaults> {
    pub request_index: u64,

    pub keygen_request: KeygenRequest<AccountId32>,

    pub _runtime: PhantomData<V>,
}

// The order of these fields matter for decoding
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode, Encode)]
pub struct EthSignTxRequestEvent<V: Vaults> {
    pub request_index: u64,

    pub eth_signing_tx_request: EthSigningTxRequest<AccountId32>,

    pub _runtime: PhantomData<V>,
}

// The order of these fields matter for decoding
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode, Encode)]
pub struct VaultRotationRequestEvent<V: Vaults> {
    pub request_index: u64,

    pub vault_rotation_request: VaultRotationRequest,

    pub _runtime: PhantomData<V>,
}

#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct VaultRotationResponseCall<T: Vaults> {
    pub request_id: u64,

    // Can we provide better types than this? It may require some changes
    // to the type accepted by the SC
    pub response: VaultRotationResponse<Vec<u8>, Vec<u8>>,

    pub _runtime: PhantomData<T>,
}

/// Derives an enum for the listed events and corresponding implementations of `From`.
macro_rules! impl_vaults_event_enum {
    ( $( $name:tt ),+ ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum VaultsEvent<V: Vaults> {
            $(
                $name($name<V>),
            )+
        }

        $(
            impl From<$name<StateChainRuntime>> for SCEvent {
                fn from(vaults_event: $name<StateChainRuntime>) -> Self {
                    SCEvent::VaultsEvent(VaultsEvent::$name(vaults_event))
                }
            }
        )+
    };
}

impl_vaults_event_enum!(
    KeygenRequestEvent,
    EthSignTxRequestEvent,
    VaultRotationRequestEvent
);
