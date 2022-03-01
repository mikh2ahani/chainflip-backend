//! This tests integration with the StakeManager contract
//! In order for these tests to work, nats and ganache with the preloaded db
//! in `./eth-db` must be loaded in
use std::str::FromStr;

use chainflip_engine::{
    eth::{
        stake_manager::{StakeManager, StakeManagerEvent},
        EthHttpRpcClient, EthObserver, EthWsRpcClient,
    },
    logging::utils,
    settings::{CommandLineOptions, Settings},
};

use futures::stream::StreamExt;
use sp_core::H160;
use sp_runtime::AccountId32;

use web3::types::U256;

mod common;

#[tokio::test]
pub async fn test_all_stake_manager_events() {
    let root_logger = utils::new_cli_logger();

    let settings =
        Settings::from_default_file("config/Testing.toml", CommandLineOptions::default()).unwrap();

    let eth_ws_rpc_client = EthWsRpcClient::new(&settings.eth, &root_logger)
        .await
        .expect("Couldn't create EthRpcClient");

    let eth_http_rpc_client = EthHttpRpcClient::new(&settings.eth, &root_logger)
        .expect("Couldn't create EthHttpRpcClient");

    // TODO: Get the address from environment variables, so we don't need to start the SC
    let stake_manager = StakeManager::new(H160::default()).unwrap();

    // The stream is infinite unless we stop it after a short time
    // in which it should have already done it's job.
    let sm_events = stake_manager
        .event_stream(&eth_http_rpc_client, &eth_ws_rpc_client, 0, &root_logger)
        .await
        .unwrap()
        .take_until(tokio::time::sleep(std::time::Duration::from_millis(1)))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Vec<_>>();

    assert!(
        !sm_events.is_empty(),
        "{}",
        common::EVENT_STREAM_EMPTY_MESSAGE
    );

    // The following event details correspond to the events in chainflip-eth-contracts/scripts/deploy_and.py
    sm_events
        .iter()
        .find(|event| match &event.event_parameters {
            StakeManagerEvent::Staked {
                account_id,
                amount,
                return_addr,
                ..
            } => {
                assert_eq!(
                    account_id,
                    &AccountId32::from_str("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuziKFgU")
                        .unwrap()
                );
                assert_eq!(amount, &40000000000000000000000);
                assert_eq!(
                    return_addr,
                    &web3::types::H160::from_str("0x0000000000000000000000000000000000000001")
                        .unwrap()
                );
                true
            }
            _ => false,
        })
        .expect("Didn't find the Staked event");

    sm_events
        .iter()
        .find(|event| match &event.event_parameters {
            StakeManagerEvent::ClaimRegistered {
                account_id,
                amount,
                staker,
                ..
            } => {
                assert_eq!(
                    account_id,
                    &AccountId32::from_str("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuziKFgU")
                        .unwrap()
                );
                assert_eq!(
                    amount,
                    &U256::from_dec_str("13333333333333334032384").unwrap()
                );
                assert_eq!(
                    staker,
                    &web3::types::H160::from_str("0x70997970c51812dc3a010c7d01b50e0d17dc79c8")
                        .unwrap()
                );
                true
            }
            _ => false,
        })
        .expect("Didn't find the ClaimRegistered event");

    sm_events
        .iter()
        .find(|event| match &event.event_parameters {
            StakeManagerEvent::ClaimExecuted {
                account_id, amount, ..
            } => {
                assert_eq!(
                    account_id,
                    &AccountId32::from_str("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuziKFgU")
                        .unwrap()
                );
                assert_eq!(amount, &13333333333333334032384);
                true
            }
            _ => false,
        })
        .expect("Didn't find the ClaimExecuted event");

    sm_events
        .iter()
        .find(|event| match event.event_parameters {
            StakeManagerEvent::MinStakeChanged {
                old_min_stake,
                new_min_stake,
                ..
            } => {
                assert_eq!(
                    old_min_stake,
                    U256::from_dec_str("40000000000000000000000").unwrap()
                );
                assert_eq!(
                    new_min_stake,
                    U256::from_dec_str("13333333333333334032384").unwrap()
                );
                true
            }
            _ => false,
        })
        .expect("Didn't find the MinStakeChanged event");

    sm_events
        .iter()
        .find(|event| match event.event_parameters {
            StakeManagerEvent::FlipSupplyUpdated {
                old_supply,
                new_supply,
                ..
            } => {
                assert_eq!(
                    old_supply,
                    U256::from_dec_str("90000000000000000000000000").unwrap()
                );
                assert_eq!(
                    new_supply,
                    U256::from_dec_str("100000000000000000000000000").unwrap()
                );
                true
            }
            _ => false,
        })
        .expect("Didn't find the FlipSupplyUpdated event");
}
