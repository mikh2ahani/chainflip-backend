
//! Autogenerated weights for pallet_cf_ingress_egress
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-09, STEPS: `20`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `janislav030.localdomain`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/chainflip-node
// benchmark
// pallet
// --extrinsic
// *
// --pallet
// pallet_cf_ingress-egress
// --output
// state-chain/pallets/cf-ingress-egress/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=20
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cf_ingress_egress.
pub trait WeightInfo {
	fn destination_assets(n: u32, ) -> Weight;
	fn egress_ccm(n: u32, ) -> Weight;
	fn disable_asset_egress() -> Weight;
	fn on_idle_with_nothing_to_send() -> Weight;
	fn process_single_deposit() -> Weight;
	fn finalise_ingress(a: u32, ) -> Weight;
}

/// Weights for pallet_cf_ingress_egress using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:1)
	// Storage: EthereumIngressEgress FetchParamDetails (r:1 w:0)
	// Storage: EthereumIngressEgress AddressStatus (r:1 w:0)
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:1 w:0)
	// Storage: Environment EthereumKeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment EthereumSignatureNonce (r:1 w:1)
	// Storage: EthereumBroadcaster BroadcastIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	// Storage: EthereumBroadcaster RequestCallbacks (r:0 w:1)
	/// The range of component `n` is `[1, 254]`.
	fn destination_assets(n: u32, ) -> Weight {
		// Minimum execution time: 168_000 nanoseconds.
		Weight::from_ref_time(171_011_523)
			// Standard Error: 12_917
			.saturating_add(Weight::from_ref_time(6_930_054).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(18))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:1 w:0)
	// Storage: Environment EthereumKeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment EthereumSignatureNonce (r:1 w:1)
	// Storage: EthereumBroadcaster BroadcastIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	/// The range of component `n` is `[1, 254]`.
	fn egress_ccm(n: u32, ) -> Weight {
		// Minimum execution time: 150_000 nanoseconds.
		Weight::from_ref_time(76_291_379)
			// Standard Error: 67_421
			.saturating_add(Weight::from_ref_time(77_097_549).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
	}
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:0 w:1)
	fn disable_asset_egress() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	fn on_idle_with_nothing_to_send() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EthereumIngressEgress DepositAddressDetailsLookup (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:1)
	// Storage: EthereumIngressEgress ChannelActions (r:1 w:0)
	// Storage: LiquidityProvider FreeBalances (r:1 w:1)
	fn process_single_deposit() -> Weight {
		// Minimum execution time: 114_000 nanoseconds.
		Weight::from_ref_time(115_000_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: EthereumIngressEgress AddressStatus (r:1 w:0)
	/// The range of component `a` is `[1, 100]`.
	fn finalise_ingress(a: u32, ) -> Weight {
		// Minimum execution time: 7_000 nanoseconds.
		Weight::from_ref_time(9_808_443)
			// Standard Error: 4_207
			.saturating_add(Weight::from_ref_time(1_981_649).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:1)
	// Storage: EthereumIngressEgress FetchParamDetails (r:1 w:0)
	// Storage: EthereumIngressEgress AddressStatus (r:1 w:0)
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:1 w:0)
	// Storage: Environment EthereumKeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment EthereumSignatureNonce (r:1 w:1)
	// Storage: EthereumBroadcaster BroadcastIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	// Storage: EthereumBroadcaster RequestCallbacks (r:0 w:1)
	/// The range of component `n` is `[1, 254]`.
	fn destination_assets(n: u32, ) -> Weight {
		// Minimum execution time: 168_000 nanoseconds.
		Weight::from_ref_time(171_011_523)
			// Standard Error: 12_917
			.saturating_add(Weight::from_ref_time(6_930_054).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(18))
			.saturating_add(RocksDbWeight::get().writes(11))
	}
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:1 w:0)
	// Storage: Environment EthereumKeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment EthereumSignatureNonce (r:1 w:1)
	// Storage: EthereumBroadcaster BroadcastIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	/// The range of component `n` is `[1, 254]`.
	fn egress_ccm(n: u32, ) -> Weight {
		// Minimum execution time: 150_000 nanoseconds.
		Weight::from_ref_time(76_291_379)
			// Standard Error: 67_421
			.saturating_add(Weight::from_ref_time(77_097_549).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(17))
			.saturating_add(RocksDbWeight::get().writes(6))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(n.into())))
	}
	// Storage: EthereumIngressEgress DisabledEgressAssets (r:0 w:1)
	fn disable_asset_egress() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(28_000_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressCcm (r:1 w:1)
	fn on_idle_with_nothing_to_send() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EthereumIngressEgress DepositAddressDetailsLookup (r:1 w:0)
	// Storage: EthereumIngressEgress ScheduledEgressFetchOrTransfer (r:1 w:1)
	// Storage: EthereumIngressEgress ChannelActions (r:1 w:0)
	// Storage: LiquidityProvider FreeBalances (r:1 w:1)
	fn process_single_deposit() -> Weight {
		// Minimum execution time: 114_000 nanoseconds.
		Weight::from_ref_time(115_000_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: EthereumIngressEgress AddressStatus (r:1 w:0)
	/// The range of component `a` is `[1, 100]`.
	fn finalise_ingress(a: u32, ) -> Weight {
		// Minimum execution time: 7_000 nanoseconds.
		Weight::from_ref_time(9_808_443)
			// Standard Error: 4_207
			.saturating_add(Weight::from_ref_time(1_981_649).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
	}
}
