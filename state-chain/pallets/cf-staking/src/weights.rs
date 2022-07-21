//! Autogenerated weights for pallet_cf_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING CHAINFLIP NODE BENCHMARK CMD VERSION 4.0.0-dev
//! DATE: 2022-06-22, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/chainflip-node
// benchmark
// pallet
// --extrinsic
// *
// --pallet
// pallet_cf_staking
// --output
// state-chain/pallets/cf-staking/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=10
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cf_staking.
pub trait WeightInfo {
	fn staked() -> Weight;
	fn claim() -> Weight;
	fn claim_all() -> Weight;
	fn claimed() -> Weight;
	fn post_claim_signature() -> Weight;
	fn retire_account() -> Weight;
	fn activate_account() -> Weight;
	fn on_initialize_best_case() -> Weight;
	fn expire_pending_claims_at(b: u32, ) -> Weight;
	fn update_minimum_stake() -> Weight;
}

/// Weights for pallet_cf_staking using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	// Storage: Environment CurrentSystemState (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:0 w:1)
	// Storage: Staking AccountRetired (r:0 w:1)
	fn staked() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(41_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Validator RotationPhase (r:1 w:0)
	// Storage: Validator CurrentEpochStartedAt (r:1 w:0)
	// Storage: Validator BlocksPerEpoch (r:1 w:0)
	// Storage: Validator ClaimPeriodAsPercentage (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:1 w:0)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Staking MinimumStake (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Staking ClaimTTL (r:1 w:0)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Environment KeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment GlobalSignatureNonce (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: Validator CurrentEpoch (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner RetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner OpenRequests (r:0 w:1)
	// Storage: EthereumThresholdSigner Signatures (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner LiveCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	fn claim() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(100_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: Flip Account (r:1 w:1)
	// Storage: Validator RotationPhase (r:1 w:0)
	// Storage: Validator CurrentEpochStartedAt (r:1 w:0)
	// Storage: Validator BlocksPerEpoch (r:1 w:0)
	// Storage: Validator ClaimPeriodAsPercentage (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Staking ClaimTTL (r:1 w:0)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Environment KeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment GlobalSignatureNonce (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: Validator CurrentEpoch (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner RetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner OpenRequests (r:0 w:1)
	// Storage: EthereumThresholdSigner Signatures (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner LiveCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	fn claim_all() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(105_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: Environment CurrentSystemState (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Flip Account (r:1 w:0)
	fn claimed() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(28_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: EthereumThresholdSigner Signatures (r:1 w:1)
	// Storage: Staking PendingClaims (r:1 w:1)
	fn post_claim_signature() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking AccountRetired (r:1 w:1)
	fn retire_account() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking AccountRetired (r:1 w:1)
	fn activate_account() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ClaimExpiries (r:1 w:0)
	fn on_initialize_best_case() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(2_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Staking PendingClaims (r:7 w:7)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Flip Account (r:7 w:7)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: System Account (r:7 w:7)
	fn expire_pending_claims_at(b: u32, ) -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(0 as Weight)
			// Standard Error: 140_000
			.saturating_add((32_555_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Staking MinimumStake (r:0 w:1)
	fn update_minimum_stake() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(9_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Environment CurrentSystemState (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:0 w:1)
	// Storage: Staking AccountRetired (r:0 w:1)
	fn staked() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(41_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Validator RotationPhase (r:1 w:0)
	// Storage: Validator CurrentEpochStartedAt (r:1 w:0)
	// Storage: Validator BlocksPerEpoch (r:1 w:0)
	// Storage: Validator ClaimPeriodAsPercentage (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:1 w:0)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Staking MinimumStake (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Staking ClaimTTL (r:1 w:0)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Environment KeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment GlobalSignatureNonce (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: Validator CurrentEpoch (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner RetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner OpenRequests (r:0 w:1)
	// Storage: EthereumThresholdSigner Signatures (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner LiveCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	fn claim() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(100_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(27 as Weight))
			.saturating_add(RocksDbWeight::get().writes(14 as Weight))
	}
	// Storage: Flip Account (r:1 w:1)
	// Storage: Validator RotationPhase (r:1 w:0)
	// Storage: Validator CurrentEpochStartedAt (r:1 w:0)
	// Storage: Validator BlocksPerEpoch (r:1 w:0)
	// Storage: Validator ClaimPeriodAsPercentage (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking WithdrawalAddresses (r:1 w:0)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Staking ClaimTTL (r:1 w:0)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Environment KeyManagerAddress (r:1 w:0)
	// Storage: Environment EthereumChainId (r:1 w:0)
	// Storage: Environment GlobalSignatureNonce (r:1 w:1)
	// Storage: EthereumThresholdSigner ThresholdSignatureRequestIdCounter (r:1 w:1)
	// Storage: Validator CeremonyIdCounter (r:1 w:1)
	// Storage: Validator CurrentEpoch (r:1 w:0)
	// Storage: EthereumVault Vaults (r:1 w:0)
	// Storage: Validator EpochAuthorityCount (r:1 w:0)
	// Storage: Reputation Suspensions (r:3 w:0)
	// Storage: EthereumThresholdSigner ThresholdSignatureResponseTimeout (r:1 w:0)
	// Storage: EthereumThresholdSigner RetryQueues (r:1 w:1)
	// Storage: EthereumThresholdSigner OpenRequests (r:0 w:1)
	// Storage: EthereumThresholdSigner Signatures (r:0 w:1)
	// Storage: EthereumThresholdSigner PendingCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner LiveCeremonies (r:0 w:1)
	// Storage: EthereumThresholdSigner RequestCallback (r:0 w:1)
	fn claim_all() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(105_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(26 as Weight))
			.saturating_add(RocksDbWeight::get().writes(14 as Weight))
	}
	// Storage: Environment CurrentSystemState (r:1 w:0)
	// Storage: Staking PendingClaims (r:1 w:1)
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Flip Account (r:1 w:0)
	fn claimed() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(28_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: EthereumThresholdSigner Signatures (r:1 w:1)
	// Storage: Staking PendingClaims (r:1 w:1)
	fn post_claim_signature() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(27_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Staking AccountRetired (r:1 w:1)
	fn retire_account() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking AccountRetired (r:1 w:1)
	fn activate_account() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ClaimExpiries (r:1 w:0)
	fn on_initialize_best_case() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(2_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Staking ClaimExpiries (r:1 w:1)
	// Storage: Staking PendingClaims (r:7 w:7)
	// Storage: Flip OffchainFunds (r:1 w:1)
	// Storage: Flip Account (r:7 w:7)
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Validator Backups (r:1 w:1)
	// Storage: System Account (r:7 w:7)
	fn expire_pending_claims_at(b: u32, ) -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(0 as Weight)
			// Standard Error: 140_000
			.saturating_add((32_555_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Staking MinimumStake (r:0 w:1)
	fn update_minimum_stake() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(9_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
