use crate::{mock::*, Error, Members};
use cf_traits::mocks::time_source;
use frame_support::{assert_noop, assert_ok, traits::OnInitialize};
use std::time::Duration;

use crate as pallet_cf_governance;

fn mock_extrinsic() -> Box<Call> {
	let call = Box::new(Call::Governance(
		pallet_cf_governance::Call::<Test>::new_membership_set(vec![EVE, PETER, MAX]),
	));
	call
}

fn next_block() {
	System::set_block_number(System::block_number() + 1);
	<Governance as OnInitialize<u64>>::on_initialize(System::block_number());
}

fn last_event() -> crate::mock::Event {
	frame_system::Pallet::<Test>::events()
		.pop()
		.expect("Event expected")
		.event
}

#[test]
fn genesis_config() {
	new_test_ext().execute_with(|| {
		let genesis_members = Members::<Test>::get();
		assert!(genesis_members.contains(&ALICE));
		assert!(genesis_members.contains(&BOB));
		assert!(genesis_members.contains(&CHARLES));
	});
}

#[test]
fn governance_restriction() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Governance::new_membership_set(Origin::signed(ALICE), vec![EVE, PETER, MAX]),
			frame_support::error::BadOrigin
		);
	});
}

#[test]
fn propose_a_governance_extrinsic_and_expect_execution() {
	new_test_ext().execute_with(|| {
		assert_ok!(Governance::propose_governance_extrinsic(
			Origin::signed(ALICE),
			mock_extrinsic()
		));
		assert_eq!(
			last_event(),
			crate::mock::Event::pallet_cf_governance(crate::Event::Proposed(0)),
		);
		next_block();
		assert_ok!(Governance::approve(Origin::signed(BOB), 0));
		next_block();
		assert_ok!(Governance::approve(Origin::signed(CHARLES), 0));
		assert_eq!(
			last_event(),
			crate::mock::Event::pallet_cf_governance(crate::Event::Approved(0)),
		);
		next_block();
		let genesis_members = Members::<Test>::get();
		assert!(genesis_members.contains(&EVE));
		assert!(genesis_members.contains(&PETER));
		assert!(genesis_members.contains(&MAX));
		assert_eq!(
			last_event(),
			crate::mock::Event::pallet_cf_governance(crate::Event::Executed(0)),
		);
	});
}

#[test]
fn expired_on_approve() {
	new_test_ext().execute_with(|| {
		const START_TIME: Duration = Duration::from_secs(10);
		const END_TIME: Duration = Duration::from_secs(7300);
		time_source::Mock::reset_to(START_TIME);
		assert_ok!(Governance::propose_governance_extrinsic(
			Origin::signed(ALICE),
			mock_extrinsic()
		));
		time_source::Mock::reset_to(END_TIME);
		assert_noop!(
			Governance::approve(Origin::signed(ALICE), 0),
			<Error<Test>>::AlreadyExpired
		);
	});
}

#[test]
fn proposal_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Governance::approve(Origin::signed(ALICE), 200),
			<Error<Test>>::NotFound
		);
	});
}

#[test]
fn propose_a_governance_extrinsic_and_expect_it_to_expire() {
	new_test_ext().execute_with(|| {
		const START_TIME: Duration = Duration::from_secs(10);
		const END_TIME: Duration = Duration::from_secs(7300);
		time_source::Mock::reset_to(START_TIME);
		next_block();
		assert_ok!(Governance::propose_governance_extrinsic(
			Origin::signed(ALICE),
			mock_extrinsic()
		));
		next_block();
		time_source::Mock::reset_to(END_TIME);
		next_block();
		assert_eq!(
			last_event(),
			crate::mock::Event::pallet_cf_governance(crate::Event::Expired(0)),
		);
		assert_noop!(
			Governance::approve(Origin::signed(ALICE), 0),
			<Error<Test>>::AlreadyExpired
		);
	});
}

#[test]
fn several_open_proposals() {
	new_test_ext().execute_with(|| {
		assert_ok!(Governance::propose_governance_extrinsic(
			Origin::signed(ALICE),
			mock_extrinsic()
		));
		assert_ok!(Governance::propose_governance_extrinsic(
			Origin::signed(BOB),
			mock_extrinsic()
		));
	});
}
