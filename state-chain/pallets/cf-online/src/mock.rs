use super::*;
use crate as pallet_cf_online;
use frame_support::{construct_runtime, parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use cf_traits::{impl_mock_stake_transfer, Chainflip, Heartbeat, NetworkState};

type ValidatorId = u64;

cf_traits::impl_mock_epoch_info!(ValidatorId, u128, u32);
impl_mock_stake_transfer!(ValidatorId, u128);

thread_local! {
	pub static VALIDATOR_HEARTBEAT: RefCell<ValidatorId> = RefCell::new(0);
	pub static NETWORK_STATE: RefCell<NetworkState<ValidatorId>> = RefCell::new(
		NetworkState {
			awaiting: vec![],
			online: vec![],
			number_of_nodes: 0,
		}
	);
}

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		OnlinePallet: pallet_cf_online::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

// A heartbeat interval in blocks
pub const HEARTBEAT_BLOCK_INTERVAL: u64 = 150;

parameter_types! {
	pub const HeartbeatBlockInterval: u64 = HEARTBEAT_BLOCK_INTERVAL;
}

pub struct MockHeartbeat;
impl Heartbeat for MockHeartbeat {
	type ValidatorId = ValidatorId;

	fn heartbeat_submitted(validator_id: &Self::ValidatorId) -> Weight {
		VALIDATOR_HEARTBEAT.with(|cell| *cell.borrow_mut() = *validator_id);
		0
	}

	fn on_heartbeat_interval(network_state: NetworkState<Self::ValidatorId>) -> Weight {
		NETWORK_STATE.with(|cell| *cell.borrow_mut() = network_state);
		0
	}
}

impl MockHeartbeat {
	pub(crate) fn network_state() -> NetworkState<ValidatorId> {
		NETWORK_STATE.with(|cell| (*cell.borrow()).clone())
	}
}

pub const ALICE: <Test as frame_system::Config>::AccountId = 100u64;
pub const BOB: <Test as frame_system::Config>::AccountId = 200u64;

cf_traits::impl_mock_ensure_witnessed_for_origin!(Origin);

impl Chainflip for Test {
	type KeyId = Vec<u8>;
	type ValidatorId = u64;
	type Amount = u128;
	type Call = Call;
	type EnsureWitnessed = MockEnsureWitnessed;
}

impl Config for Test {
	type Event = Event;
	type HeartbeatBlockInterval = HeartbeatBlockInterval;
	type Heartbeat = MockHeartbeat;
	type EpochInfo = MockEpochInfo;
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let config = GenesisConfig { system: Default::default() };

	let mut ext: sp_io::TestExternalities = config.build_storage().unwrap().into();

	MockEpochInfo::add_validator(ALICE);

	ext.execute_with(|| {
		System::set_block_number(1);
	});

	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		OnlinePallet::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		OnlinePallet::on_initialize(System::block_number());
	}
}