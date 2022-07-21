//! Autogenerated weights for pallet_cf_reputation
//!
//! THIS FILE WAS AUTO-GENERATED USING CHAINFLIP NODE BENCHMARK CMD VERSION 4.0.0-dev
//! DATE: 2022-07-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/chainflip-node
// benchmark
// pallet
// --extrinsic
// *
// --pallet
// pallet_cf_reputation
// --output
// state-chain/pallets/cf-reputation/src/weights.rs
// --execution=wasm
// --steps=20
// --repeat=10
// --template=state-chain/chainflip-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cf_reputation.
pub trait WeightInfo {
	fn update_accrual_ratio() -> Weight;
	fn set_penalty() -> Weight;
	fn update_missed_heartbeat_penalty() -> Weight;
	fn on_runtime_upgrade() -> Weight;
	fn heartbeat() -> Weight;
	fn submit_network_state() -> Weight;
	fn on_initialize_no_action() -> Weight;
}

/// Weights for pallet_cf_reputation using the Substrate node and recommended hardware.
pub struct PalletWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PalletWeight<T> {
	// Storage: Reputation AccrualRatio (r:0 w:1)
	fn update_accrual_ratio() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Reputation Penalties (r:1 w:1)
	fn set_penalty() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(14_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Reputation Penalties (r:0 w:1)
	fn update_missed_heartbeat_penalty() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x19ebab52d1fc3244db45364979b4af8e4e7b9012096b41c4eb3aaf947f6ea429] (r:1 w:0)
	fn on_runtime_upgrade() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(7_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Reputation LastHeartbeat (r:1 w:1)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation AccrualRatio (r:1 w:0)
	fn heartbeat() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(11_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Reputation LastHeartbeat (r:1 w:0)
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation Suspensions (r:1 w:1)
	// Storage: Flip SlashingRate (r:1 w:0)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Flip TotalIssuance (r:1 w:1)
	// Storage: Validator Backups (r:1 w:0)
	fn submit_network_state() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(45_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_no_action() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(0 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Reputation AccrualRatio (r:0 w:1)
	fn update_accrual_ratio() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Reputation Penalties (r:1 w:1)
	fn set_penalty() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Reputation Penalties (r:0 w:1)
	fn update_missed_heartbeat_penalty() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x19ebab52d1fc3244db45364979b4af8e4e7b9012096b41c4eb3aaf947f6ea429] (r:1 w:0)
	fn on_runtime_upgrade() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(7_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Reputation LastHeartbeat (r:1 w:1)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation AccrualRatio (r:1 w:0)
	fn heartbeat() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(11_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Validator CurrentAuthorities (r:1 w:0)
	// Storage: Reputation LastHeartbeat (r:1 w:0)
	// Storage: Reputation Penalties (r:1 w:0)
	// Storage: Reputation Reputations (r:1 w:1)
	// Storage: Reputation Suspensions (r:1 w:1)
	// Storage: Flip SlashingRate (r:1 w:0)
	// Storage: Flip Account (r:1 w:1)
	// Storage: Flip TotalIssuance (r:1 w:1)
	// Storage: Validator Backups (r:1 w:0)
	fn submit_network_state() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(45_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_no_action() -> Weight {
		#[allow(clippy::unnecessary_cast)]
		(0 as Weight)
	}
}
