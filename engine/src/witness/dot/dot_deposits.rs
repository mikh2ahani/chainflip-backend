use std::collections::BTreeSet;

use cf_primitives::{EpochIndex, PolkadotBlockNumber};
use futures_core::Future;
use pallet_cf_ingress_egress::{DepositChannelDetails, DepositWitness};
use state_chain_runtime::PolkadotInstance;

use super::super::common::chunked_chain_source::chunked_by_vault::{
	builder::ChunkedByVaultBuilder, ChunkedByVault,
};
use crate::witness::{
	common::{
		chunked_chain_source::chunked_by_vault::deposit_addresses::Addresses, RuntimeCallHasChain,
		RuntimeHasChain,
	},
	dot::EventWrapper,
};
use cf_chains::{
	assets::dot::Asset,
	dot::{PolkadotAccountId, PolkadotExtrinsicIndex, PolkadotHash},
	Polkadot,
};
use subxt::events::Phase;

impl<Inner: ChunkedByVault> ChunkedByVaultBuilder<Inner> {
	pub fn dot_deposits<ProcessCall, ProcessingFut>(
		self,
		process_call: ProcessCall,
	) -> ChunkedByVaultBuilder<
		impl ChunkedByVault<
			Index = PolkadotBlockNumber,
			Hash = PolkadotHash,
			Data = (Vec<(Phase, EventWrapper)>, BTreeSet<u32>),
			Chain = Polkadot,
			ExtraInfo = PolkadotAccountId,
			ExtraHistoricInfo = (),
		>,
	>
	where
		Inner: ChunkedByVault<
			Index = PolkadotBlockNumber,
			Hash = PolkadotHash,
			Data = (Vec<(Phase, EventWrapper)>, Addresses<Inner>),
			Chain = Polkadot,
			ExtraInfo = PolkadotAccountId,
			ExtraHistoricInfo = (),
		>,
		ProcessCall: Fn(state_chain_runtime::RuntimeCall, EpochIndex) -> ProcessingFut
			+ Send
			+ Sync
			+ Clone
			+ 'static,
		ProcessingFut: Future<Output = ()> + Send + 'static,
		state_chain_runtime::Runtime: RuntimeHasChain<Inner::Chain>,
		state_chain_runtime::RuntimeCall:
			RuntimeCallHasChain<state_chain_runtime::Runtime, Inner::Chain>,
	{
		self.then(move |epoch, header| {
			let process_call = process_call.clone();
			async move {
				let (events, addresses_and_details) = header.data;

				let addresses = address_and_details_to_addresses(addresses_and_details);

				let (deposit_witnesses, broadcast_indices) =
					deposit_witnesses(header.index, addresses, &events, &epoch.info.1);

				if !deposit_witnesses.is_empty() {
					process_call(
						pallet_cf_ingress_egress::Call::<_, PolkadotInstance>::process_deposits {
							deposit_witnesses,
							block_height: header.index,
						}
						.into(),
						epoch.index,
					)
					.await
				}

				(events, broadcast_indices)
			}
		})
	}
}

fn address_and_details_to_addresses(
	address_and_details: Vec<DepositChannelDetails<state_chain_runtime::Runtime, PolkadotInstance>>,
) -> Vec<PolkadotAccountId> {
	address_and_details
		.into_iter()
		.map(|deposit_channel_details| {
			assert_eq!(deposit_channel_details.deposit_channel.asset, Asset::Dot);
			deposit_channel_details.deposit_channel.address
		})
		.collect()
}

// Return the deposit witnesses and the extrinsic indices of transfers we want
// to confirm the broadcast of.
fn deposit_witnesses(
	block_number: PolkadotBlockNumber,
	monitored_addresses: Vec<PolkadotAccountId>,
	events: &Vec<(Phase, EventWrapper)>,
	our_vault: &PolkadotAccountId,
) -> (Vec<DepositWitness<Polkadot>>, BTreeSet<PolkadotExtrinsicIndex>) {
	let mut deposit_witnesses = vec![];
	let mut extrinsic_indices = BTreeSet::new();
	for (phase, wrapped_event) in events {
		if let Phase::ApplyExtrinsic(extrinsic_index) = phase {
			if let EventWrapper::Transfer { to, amount, from } = wrapped_event {
				let deposit_address = PolkadotAccountId::from_aliased(to.0);
				if monitored_addresses.contains(&deposit_address) {
					deposit_witnesses.push(DepositWitness {
						deposit_address,
						asset: Asset::Dot,
						amount: *amount,
						deposit_details: (),
					});
				}
				// It's possible a transfer to one of the monitored addresses comes from our_vault,
				// so this cannot be an else if
				if &PolkadotAccountId::from_aliased(from.0) == our_vault ||
					&deposit_address == our_vault
				{
					tracing::info!(
						"Interesting transfer at block: {block_number}, extrinsic index: {extrinsic_index} from: {from:?} to: {to:?}", 
					);
					extrinsic_indices.insert(*extrinsic_index);
				}
			}
		}
	}
	(deposit_witnesses, extrinsic_indices)
}

#[cfg(test)]
mod test {
	use cf_chains::dot::PolkadotBalance;

	use crate::witness::dot::test::phase_and_events;

	use super::*;

	fn mock_transfer(
		from: &PolkadotAccountId,
		to: &PolkadotAccountId,
		amount: PolkadotBalance,
	) -> EventWrapper {
		EventWrapper::Transfer {
			from: from.aliased_ref().to_owned().into(),
			to: to.aliased_ref().to_owned().into(),
			amount,
		}
	}

	#[test]
	fn witness_deposits_for_addresses_we_monitor() {
		let our_vault = PolkadotAccountId::from_aliased([0; 32]);

		// we want two monitors, one sent through at start, and one sent through channel
		const TRANSFER_1_INDEX: u32 = 1;
		let transfer_1_deposit_address = PolkadotAccountId::from_aliased([1; 32]);
		const TRANSFER_1_AMOUNT: PolkadotBalance = 10000;

		const TRANSFER_2_INDEX: u32 = 2;
		let transfer_2_deposit_address = PolkadotAccountId::from_aliased([2; 32]);
		const TRANSFER_2_AMOUNT: PolkadotBalance = 20000;

		const TRANSFER_FROM_OUR_VAULT_INDEX: u32 = 7;
		const TRANFER_TO_OUR_VAULT_INDEX: u32 = 8;

		const TRANSFER_TO_SELF_INDEX: u32 = 9;
		const TRANSFER_TO_SELF_AMOUNT: PolkadotBalance = 30000;

		let block_event_details = phase_and_events(vec![
			// we'll be witnessing this from the start
			(
				TRANSFER_1_INDEX,
				mock_transfer(
					&PolkadotAccountId::from_aliased([7; 32]),
					&transfer_1_deposit_address,
					TRANSFER_1_AMOUNT,
				),
			),
			// we'll receive this address from the channel
			(
				TRANSFER_2_INDEX,
				mock_transfer(
					&PolkadotAccountId::from_aliased([7; 32]),
					&transfer_2_deposit_address,
					TRANSFER_2_AMOUNT,
				),
			),
			// this one is not for us
			(
				19,
				mock_transfer(
					&PolkadotAccountId::from_aliased([7; 32]),
					&PolkadotAccountId::from_aliased([9; 32]),
					93232,
				),
			),
			(
				TRANSFER_FROM_OUR_VAULT_INDEX,
				mock_transfer(&our_vault, &PolkadotAccountId::from_aliased([9; 32]), 93232),
			),
			(
				TRANFER_TO_OUR_VAULT_INDEX,
				mock_transfer(&PolkadotAccountId::from_aliased([9; 32]), &our_vault, 93232),
			),
			// Example: Someone generates a DOT -> ETH swap, getting the DOT address that we're now
			// monitoring for inputs. They now generate a BTC -> DOT swap, and set the destination
			// address of the DOT to the address they generated earlier.
			// Now our Polakdot vault is sending to an address we're monitoring for deposits.
			(
				TRANSFER_TO_SELF_INDEX,
				mock_transfer(&our_vault, &transfer_2_deposit_address, TRANSFER_TO_SELF_AMOUNT),
			),
		]);

		let (deposit_witnesses, broadcast_indices) = deposit_witnesses(
			32,
			vec![transfer_1_deposit_address, transfer_2_deposit_address],
			&block_event_details,
			&our_vault,
		);

		assert_eq!(deposit_witnesses.len(), 3);
		assert_eq!(deposit_witnesses.get(0).unwrap().amount, TRANSFER_1_AMOUNT);
		assert_eq!(deposit_witnesses.get(1).unwrap().amount, TRANSFER_2_AMOUNT);
		assert_eq!(deposit_witnesses.get(2).unwrap().amount, TRANSFER_TO_SELF_AMOUNT);

		// Check the egress and ingress fetch
		assert_eq!(broadcast_indices.len(), 3);
		assert!(broadcast_indices.contains(&TRANSFER_FROM_OUR_VAULT_INDEX));
		assert!(broadcast_indices.contains(&TRANFER_TO_OUR_VAULT_INDEX));
		assert!(broadcast_indices.contains(&TRANSFER_TO_SELF_INDEX));
	}
}
