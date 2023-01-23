
//! Autogenerated weights for pallet_cf_threshold_signature
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-23, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kylezs.localdomain`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/chainflip-node
// benchmark
// pallet
// --extrinsic
// *
// --pallet
// pallet_cf_threshold-signature
// --output
// state-chain/pallets/cf-threshold-signature/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=10
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cf_threshold_signature.
pub trait WeightInfo {
	fn signature_success() -> Weight;
	fn report_signature_failed(a: u32, ) -> Weight;
	fn set_threshold_signature_timeout() -> Weight;
	fn on_initialize(a: u32, r: u32, ) -> Weight;
	fn report_offenders(o: u32, ) -> Weight;
}

/// Weights for pallet_cf_threshold_signature using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	// Storage: EthereumThresholdSigner PendingCeremonies (r:1 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:1 w:0)
	// Storage: EthereumThresholdSigner PendingRequestInstructions (r:0 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	fn signature_success() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(37_000_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: AccountRoles AccountRoles (r:1 w:0)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:1 w:1)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	/// The range of component `a` is `[1, 100]`.
	fn report_signature_failed(_a: u32, ) -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(45_476_538)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:1)
	fn set_threshold_signature_timeout() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:2 w:2)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:50 w:100)
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: EthereumThresholdSigner RequestRetryQueue (r:1 w:0)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	/// The range of component `a` is `[10, 150]`.
	/// The range of component `r` is `[0, 50]`.
	fn on_initialize(a: u32, r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(11_289_319)
			// Standard Error: 62_565
			.saturating_add(Weight::from_ref_time(207_509).saturating_mul(a.into()))
			// Standard Error: 173_928
			.saturating_add(Weight::from_ref_time(33_102_908).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
	}
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation Suspensions (r:1 w:1)
	/// The range of component `o` is `[1, 100]`.
	fn report_offenders(o: u32, ) -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(17_464_120)
			// Standard Error: 29_253
			.saturating_add(Weight::from_ref_time(6_229_654).saturating_mul(o.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(o.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(o.into())))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: EthereumThresholdSigner PendingCeremonies (r:1 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:1 w:0)
	// Storage: EthereumThresholdSigner PendingRequestInstructions (r:0 w:1)
	// Storage: EthereumThresholdSigner Signature (r:0 w:1)
	fn signature_success() -> Weight {
		// Minimum execution time: 30_000 nanoseconds.
		Weight::from_ref_time(37_000_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: AccountRoles AccountRoles (r:1 w:0)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:1 w:1)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:1 w:1)
	/// The range of component `a` is `[1, 100]`.
	fn report_signature_failed(_a: u32, ) -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(45_476_538)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:1)
	fn set_threshold_signature_timeout() -> Weight {
		// Minimum execution time: 17_000 nanoseconds.
		Weight::from_ref_time(19_000_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner CeremonyRetryQueues (r:2 w:2)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:50 w:100)
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: EthereumVault CurrentVaultEpochAndState (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Validator HistoricalAuthorities (r:1 w:0)
	// Storage: EthereumThresholdSigner RequestRetryQueue (r:1 w:0)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	/// The range of component `a` is `[10, 150]`.
	/// The range of component `r` is `[0, 50]`.
	fn on_initialize(a: u32, r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(11_289_319)
			// Standard Error: 62_565
			.saturating_add(Weight::from_ref_time(207_509).saturating_mul(a.into()))
			// Standard Error: 173_928
			.saturating_add(Weight::from_ref_time(33_102_908).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(3))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(r.into())))
	}
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation Suspensions (r:1 w:1)
	/// The range of component `o` is `[1, 100]`.
	fn report_offenders(o: u32, ) -> Weight {
		// Minimum execution time: 24_000 nanoseconds.
		Weight::from_ref_time(17_464_120)
			// Standard Error: 29_253
			.saturating_add(Weight::from_ref_time(6_229_654).saturating_mul(o.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(o.into())))
			.saturating_add(RocksDbWeight::get().writes(1))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(o.into())))
	}
}
