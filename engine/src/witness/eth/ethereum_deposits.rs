use crate::{
	eth::retry_rpc::address_checker::*,
	state_chain_observer::client::{
		chain_api::ChainApi, extrinsic_api::signed::SignedExtrinsicApi, storage_api::StorageApi,
	},
};
use anyhow::ensure;
use cf_chains::Ethereum;
use cf_primitives::chains::assets::eth;
use ethers::types::Bloom;
use pallet_cf_ingress_egress::DepositChannelDetails;
use sp_core::H256;
use state_chain_runtime::EthereumInstance;
use std::sync::Arc;

use crate::witness::eth::vault::VaultEvents;

use std::collections::BTreeMap;

use ethers::prelude::*;
use itertools::Itertools;
use sp_core::U256;

use crate::eth::rpc::address_checker::*;

use super::{contract_common::events_at_block, vault::FetchedNativeFilter};
use crate::witness::common::chain_source::Header;

use super::super::common::{
	chunked_chain_source::chunked_by_vault::{builder::ChunkedByVaultBuilder, ChunkedByVault},
	STATE_CHAIN_CONNECTION,
};
use crate::eth::retry_rpc::EthersRetryRpcApi;

impl<Inner: ChunkedByVault> ChunkedByVaultBuilder<Inner> {
	pub async fn ethereum_deposits<StateChainClient, EthRetryRpcClient>(
		self,
		state_chain_client: Arc<StateChainClient>,
		eth_rpc: EthRetryRpcClient,
	) -> ChunkedByVaultBuilder<impl ChunkedByVault>
	where
		Inner: ChunkedByVault<
			Index = u64,
			Hash = H256,
			Data = (Bloom, Vec<DepositChannelDetails<Ethereum>>),
			Chain = Ethereum,
		>,
		StateChainClient: SignedExtrinsicApi + StorageApi + ChainApi + Send + Sync + 'static,
		EthRetryRpcClient: EthersRetryRpcApi + AddressCheckerRetryRpcApi + Send + Sync + Clone,
	{
		let address_checker_address = state_chain_client
			.storage_value::<pallet_cf_environment::EthereumAddressCheckerAddress<state_chain_runtime::Runtime>>(
				state_chain_client.latest_finalized_hash(),
			)
			.await
			.expect(STATE_CHAIN_CONNECTION)
			.into();

		let vault_address = state_chain_client
			.storage_value::<pallet_cf_environment::EthereumVaultAddress<state_chain_runtime::Runtime>>(
				state_chain_client.latest_finalized_hash(),
			)
			.await
			.expect(STATE_CHAIN_CONNECTION)
			.into();

		self.then(move |epoch, header| {
			let eth_rpc = eth_rpc.clone();
			let state_chain_client = state_chain_client.clone();
			async move {
				let (bloom, deposit_channels) = header.data;

				const NATIVE_ASSET: eth::Asset = eth::Asset::Eth;

				// Genesis block cannot contain any transactions
				if let Some(parent_hash) = header.parent_hash {
					if !deposit_channels.is_empty() {
						let addresses = deposit_channels
							.into_iter()
							.filter(|deposit_channel| {
								deposit_channel.deposit_channel.asset == NATIVE_ASSET
							})
							.map(|deposit_channel| deposit_channel.deposit_channel.address)
							.collect::<Vec<_>>();

						let ingresses = eth_ingresses_at_block(
							address_states(
								&eth_rpc,
								address_checker_address,
								parent_hash,
								header.hash,
								addresses,
							)
							.await?,
							events_at_block::<VaultEvents, _>(
								Header {
									index: header.index,
									hash: header.hash,
									parent_hash: header.parent_hash,
									data: bloom,
								},
								vault_address,
								&eth_rpc,
							)
							.await?
							.into_iter()
							.filter_map(|event| match event.event_parameters {
								VaultEvents::FetchedNativeFilter(event) => Some(event),
								_ => None,
							})
							.collect(),
						)?;

						if !ingresses.is_empty() {
							state_chain_client
								.submit_signed_extrinsic(
									pallet_cf_witnesser::Call::witness_at_epoch {
										call:
											Box::new(
												pallet_cf_ingress_egress::Call::<
													_,
													EthereumInstance,
												>::process_deposits {
													deposit_witnesses: ingresses
														.into_iter()
														.map(|(to_addr, value)| {
															pallet_cf_ingress_egress::DepositWitness {
																deposit_address: to_addr,
																asset: NATIVE_ASSET,
																amount:
																	value
																	.try_into()
																	.expect("Ingress witness transfer value should fit u128"),
																deposit_details: (),
															}
														})
														.collect(),
													block_height: header.index,
												}
												.into(),
											),
										epoch_index: epoch.index,
									},
								)
								.await;
						}
					}
				}

				Ok::<_, anyhow::Error>(())
			}
		})
	}
}

async fn address_states<EthRetryRpcClient>(
	eth_rpc: &EthRetryRpcClient,
	address_checker_address: H160,
	parent_hash: H256,
	hash: H256,
	addresses: Vec<H160>,
) -> Result<impl Iterator<Item = (H160, (AddressState, AddressState))>, anyhow::Error>
where
	EthRetryRpcClient: AddressCheckerRetryRpcApi + Send + Sync + Clone,
{
	let previous_address_states = eth_rpc
		.address_states(parent_hash, address_checker_address, addresses.clone())
		.await;

	let address_states =
		eth_rpc.address_states(hash, address_checker_address, addresses.clone()).await;

	ensure!(
		addresses.len() == previous_address_states.len() &&
			previous_address_states.len() == address_states.len()
	);

	Ok(addresses
		.into_iter()
		.zip(previous_address_states.into_iter().zip(address_states)))
}

pub fn eth_ingresses_at_block<
	Addresses: IntoIterator<Item = (H160, (AddressState, AddressState))>,
>(
	addresses: Addresses,
	native_events: Vec<FetchedNativeFilter>,
) -> Result<Vec<(H160, U256)>, anyhow::Error> {
	let fetched_native_totals: BTreeMap<_, _> = native_events
		.into_iter()
		.into_group_map_by(|f| f.sender)
		.into_iter()
		.map(|(sender, events)| {
			(sender, events.into_iter().fold(U256::from(0), |acc, f| acc.saturating_add(f.amount)))
		})
		.collect();

	addresses
		.into_iter()
		.map(|(address, (previous_address_state, address_state))| {
			Ok((
				address,
				if !address_state.has_contract {
					ensure!(!previous_address_state.has_contract);
					ensure!(fetched_native_totals.get(&address).is_none());

					address_state.balance.saturating_sub(previous_address_state.balance)
				} else {
					let fetched_native_total =
						fetched_native_totals.get(&address).cloned().unwrap_or_default();

					if !previous_address_state.has_contract {
						fetched_native_total.saturating_sub(previous_address_state.balance)
					} else {
						fetched_native_total
					}
				},
			))
		})
		.filter_ok(|(_address, ingress_amount)| !ingress_amount.is_zero())
		.collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
	use crate::{
		eth::{
			retry_rpc::{EthersRetryRpcApi, EthersRetryRpcClient},
			rpc::{EthRpcClient, ReconnectSubscriptionClient},
		},
		settings::Settings,
		witness::common::chain_source::Header,
	};

	use super::{super::contract_common::events_at_block, *};
	use ethers::prelude::U256;
	use futures_util::FutureExt;
	use utilities::task_scope;

	use super::super::vault::VaultEvents;

	#[test]
	fn block_empty_lists() {
		let addresses = [];
		let native_events = Default::default();

		let ingresses = eth_ingresses_at_block(addresses, native_events).unwrap();

		assert!(ingresses.is_empty());
	}

	#[test]
	fn test_eth_ingresses_at_block_no_contract() {
		let address = H160::random();
		let addresses = [(
			address,
			(
				AddressState { balance: U256::from(100), has_contract: false },
				AddressState { balance: U256::from(200), has_contract: false },
			),
		)];

		// some random event should not be ignored
		let native_events =
			vec![FetchedNativeFilter { sender: H160::random(), amount: U256::from(300) }];

		let ingresses = eth_ingresses_at_block(addresses, native_events).unwrap();

		assert!(ingresses.eq(&[(address, U256::from(100))]));
	}

	#[test]
	fn test_eth_ingresses_at_block_with_contract() {
		let before_contract_deployed = U256::from(200);

		let addresses = vec![
			(
				H160::random(),
				(
					AddressState { balance: U256::from(200), has_contract: false },
					AddressState { balance: U256::from(0), has_contract: true },
				),
			),
			(
				H160::random(),
				(
					AddressState { balance: U256::from(0), has_contract: false },
					AddressState { balance: U256::from(0), has_contract: true },
				),
			),
		];

		// There were two events were emitted in the same Ethereum block
		let native_events = vec![
			FetchedNativeFilter { sender: addresses[0].0, amount: before_contract_deployed },
			FetchedNativeFilter { sender: addresses[0].0, amount: U256::from(123) },
			FetchedNativeFilter { sender: addresses[1].0, amount: U256::from(212) },
			// Not in our list of monitored addresses, so we don't witness it.
			FetchedNativeFilter { sender: H160::random(), amount: U256::from(420) },
		];

		let ingresses = eth_ingresses_at_block(addresses.clone(), native_events).unwrap();
		assert!(
			ingresses.eq(&[(addresses[0].0, U256::from(123)), (addresses[1].0, U256::from(212))])
		);
	}

	#[ignore = "requries connection to a node"]
	#[tokio::test]
	async fn test_get_ingress_contract() {
		task_scope::task_scope(|scope| {
			async {
				let vault_address: H160 =
					"B7A5bd0345EF1Cc5E66bf61BdeC17D2461fBd968".parse().unwrap();
				let address_checker_address =
					"e7f1725E7734CE288F8367e1Bb143E90bb3F0512".parse::<Address>().unwrap();

				let settings = Settings::new_test().unwrap();

				let client = EthersRetryRpcClient::new(
					scope,
					EthRpcClient::new(&settings.eth).await.unwrap(),
					ReconnectSubscriptionClient::new(
						settings.eth.ws_node_endpoint,
						web3::types::U256::from(1337),
					),
				);

				let addresses = vec![
					"41aD2bc63A2059f9b623533d87fe99887D794847".parse().unwrap(),
					"c2774b2f1972f50ac6113e81721cc7214388434d".parse().unwrap(),
				];

				let block_number = 138;
				let block = client.block(block_number.into()).await;

				let address_states = address_states(
					&client,
					address_checker_address,
					block.parent_hash,
					block.hash.unwrap(),
					addresses.clone(),
				)
				.await
				.unwrap();

				let fetched_native_events = events_at_block::<VaultEvents, _>(
					Header {
						index: block_number,
						parent_hash: Some(block.parent_hash),
						hash: block.hash.unwrap(),
						data: block.logs_bloom.unwrap(),
					},
					vault_address,
					&client,
				)
				.await
				.unwrap()
				.into_iter()
				.filter_map(|event| match event.event_parameters {
					VaultEvents::FetchedNativeFilter(event) => Some(event),
					_ => None,
				})
				.collect();

				let increases =
					eth_ingresses_at_block(address_states, fetched_native_events).unwrap();

				for (address, increase) in increases {
					println!("{}: {}", address, increase);
				}

				Ok(())
			}
			.boxed()
		})
		.await
		.unwrap();
	}
}