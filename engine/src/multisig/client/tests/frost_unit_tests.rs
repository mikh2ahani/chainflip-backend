use crate::{
    logging::{REQUEST_TO_SIGN_EXPIRED, REQUEST_TO_SIGN_IGNORED, SIGNING_CEREMONY_FAILED},
    multisig::{
        self,
        client::{
            self, keygen,
            signing::frost,
            tests::helpers::{
                gen_invalid_local_sig, gen_invalid_signing_comm1, new_nodes,
                new_signing_ceremony_with_keygen, run_stages, split_messages_for, standard_signing,
                KeygenCeremonyRunner, SigningCeremonyRunner, STAGE_FINISHED_OR_NOT_STARTED,
            },
            tests::*,
            KeygenOptions,
        },
        crypto::Rng,
        tests::fixtures::MESSAGE_HASH,
        MultisigInstruction,
    },
    testing::assert_ok,
};
use rand_legacy::SeedableRng;

use itertools::Itertools;

use super::helpers::{self, for_each_stage, run_keygen, standard_signing_coroutine};

// Data for any stage that arrives one stage too early should be properly delayed
// and processed after the stage transition is made
#[tokio::test]
async fn should_delay_stage_data() {
    for_each_stage(
        1..SIGNING_STAGES,
        || Box::pin(async { new_signing_ceremony_with_keygen().await.0 }),
        standard_signing_coroutine,
        |stage_number, mut ceremony, (_, messages, _)| async move {
            let [late_sender, test_account] = ceremony.select_account_ids();

            let get_messages_for_stage = |stage_index: usize| {
                split_messages_for(messages[stage_index].clone(), &test_account, &late_sender)
            };

            // Receive the data of this stage and the next stage from all but one client
            let (late_msg, msgs) = get_messages_for_stage(stage_number - 1);
            ceremony.distribute_messages(msgs);
            let (next_late_msg, next_msgs) = get_messages_for_stage(stage_number);
            ceremony.distribute_messages(next_msgs);

            assert_ok!(ceremony.nodes[&test_account]
                .client
                .ensure_ceremony_at_signing_stage(stage_number, ceremony.ceremony_id));

            // Now receive the final client's data to advance the stage
            ceremony.distribute_messages(late_msg);

            assert_ok!(ceremony.nodes[&test_account]
                .client
                .ensure_ceremony_at_signing_stage(stage_number + 1, ceremony.ceremony_id));

            ceremony.distribute_messages(next_late_msg);

            // Check that the stage correctly advanced or finished
            assert_ok!(ceremony.nodes[&test_account]
                .client
                .ensure_ceremony_at_signing_stage(
                    if stage_number + 2 > SIGNING_STAGES {
                        // The keygen finished
                        STAGE_FINISHED_OR_NOT_STARTED
                    } else {
                        stage_number + 2
                    },
                    ceremony.ceremony_id
                ));
        },
    )
    .await;
}

// If any initial commitments arrive before the request to sign,
// they should be delayed and processed after it arrives
#[tokio::test]
async fn should_delay_comm1_before_rts() {
    let test_id = &ACCOUNT_IDS[0];

    let mut signing_ceremony = new_signing_ceremony_with_keygen().await.0;
    let (_, signing_messages) = standard_signing(&mut signing_ceremony).await;

    let mut signing_ceremony = new_signing_ceremony_with_keygen().await.0;

    // Send comm1 messages from the other clients
    signing_ceremony.distribute_messages(signing_messages.stage_1_messages);

    assert_ok!(signing_ceremony.nodes[test_id]
        .client
        .ensure_ceremony_at_signing_stage(
            STAGE_FINISHED_OR_NOT_STARTED,
            signing_ceremony.ceremony_id
        ));

    // Now we get the request to sign (effectively receiving the request from our StateChain)
    signing_ceremony.request().await;

    // It should advance to stage 2 right away if the comm1's were delayed correctly
    assert_ok!(signing_ceremony.nodes[test_id]
        .client
        .ensure_ceremony_at_signing_stage(2, signing_ceremony.ceremony_id));
}

#[tokio::test]
async fn should_handle_invalid_local_sig() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let messages = signing_ceremony.request().await;
    let mut messages = helpers::run_stages!(
        signing_ceremony,
        messages,
        frost::VerifyComm2,
        frost::LocalSig3
    );

    // This account id will send an invalid signature
    let [bad_account_id] = signing_ceremony.select_account_ids();
    let invalid_sig3 = gen_invalid_local_sig(&mut signing_ceremony.rng);
    for message in messages.get_mut(&bad_account_id).unwrap().values_mut() {
        *message = invalid_sig3.clone();
    }

    let messages = signing_ceremony
        .run_stage::<frost::VerifyLocalSig4, _, _>(messages)
        .await;
    signing_ceremony.distribute_messages(messages);
    signing_ceremony
        .complete_with_error(&[bad_account_id])
        .await;
    assert!(signing_ceremony
        .nodes
        .values()
        .all(|node| node.tag_cache.contains_tag(SIGNING_CEREMONY_FAILED)));
}

#[tokio::test]
async fn should_handle_inconsistent_broadcast_com1() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let mut messages = signing_ceremony.request().await;

    // This account id will send an invalid signature
    let [bad_account_id] = signing_ceremony.select_account_ids();
    for message in messages.get_mut(&bad_account_id).unwrap().values_mut() {
        *message = gen_invalid_signing_comm1(&mut signing_ceremony.rng);
    }

    let messages = signing_ceremony
        .run_stage::<frost::VerifyComm2, _, _>(messages)
        .await;
    signing_ceremony.distribute_messages(messages);
    signing_ceremony
        .complete_with_error(&[bad_account_id])
        .await;
    assert!(signing_ceremony
        .nodes
        .values()
        .all(|node| node.tag_cache.contains_tag(SIGNING_CEREMONY_FAILED)));
}

#[tokio::test]
async fn should_handle_inconsistent_broadcast_sig3() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let messages = signing_ceremony.request().await;

    let mut messages = helpers::run_stages!(
        signing_ceremony,
        messages,
        frost::VerifyComm2,
        frost::LocalSig3
    );

    // This account id will send an invalid signature
    let [bad_account_id] = signing_ceremony.select_account_ids();
    for message in messages.get_mut(&bad_account_id).unwrap().values_mut() {
        *message = gen_invalid_local_sig(&mut signing_ceremony.rng);
    }

    let messages = signing_ceremony
        .run_stage::<frost::VerifyLocalSig4, _, _>(messages)
        .await;
    signing_ceremony.distribute_messages(messages);
    signing_ceremony
        .complete_with_error(&[bad_account_id])
        .await;
    assert!(signing_ceremony
        .nodes
        .values()
        .all(|node| node.tag_cache.contains_tag(SIGNING_CEREMONY_FAILED)));
}

#[tokio::test]
async fn should_ignore_duplicate_rts() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;
    let [test_id] = signing_ceremony.select_account_ids();

    let messages = signing_ceremony.request().await;

    helpers::run_stages!(signing_ceremony, messages, frost::VerifyComm2,);

    assert_ok!(signing_ceremony.nodes[&test_id]
        .client
        .ensure_ceremony_at_signing_stage(2, signing_ceremony.ceremony_id));

    // Send another request to sign with the same ceremony_id and key_id
    signing_ceremony.request_without_gather();

    // The request should have been rejected and the existing ceremony is unchanged
    assert_ok!(signing_ceremony.nodes[&test_id]
        .client
        .ensure_ceremony_at_signing_stage(2, signing_ceremony.ceremony_id));
    assert!(signing_ceremony.nodes[&test_id]
        .tag_cache
        .contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn should_delay_rts_until_key_is_ready() {
    let keygen_ceremony_id = 1;
    let new_keygen_ceremony = || {
        KeygenCeremonyRunner::new(
            new_nodes(ACCOUNT_IDS.clone(), KeygenOptions::allowing_high_pubkey()),
            keygen_ceremony_id,
            Rng::from_seed([8; 32]),
        )
    };

    let test_node_id = &ACCOUNT_IDS[0];

    let keygen_ceremony = new_keygen_ceremony();
    let (key_id, _, _) = helpers::standard_keygen(keygen_ceremony).await;
    let mut keygen_ceremony = new_keygen_ceremony();

    let messages = keygen_ceremony.request().await;
    let messages = run_stages!(
        keygen_ceremony,
        messages,
        keygen::VerifyComm2,
        keygen::SecretShare3,
        keygen::Complaints4,
        keygen::VerifyComplaints5
    );

    let (mut signing_ceremony, non_signing_nodes) =
        SigningCeremonyRunner::new_with_threshold_subset_of_signers(
            keygen_ceremony.nodes,
            1,
            key_id,
            MESSAGE_HASH.clone(),
            Rng::from_seed([4; 32]),
        );

    // Send the request to sign
    signing_ceremony.request_without_gather();

    let mut keygen_ceremony = KeygenCeremonyRunner::new(
        signing_ceremony
            .nodes
            .into_iter()
            .chain(non_signing_nodes)
            .collect(),
        keygen_ceremony_id,
        Rng::from_seed([8; 32]),
    );

    // The request to sign should have been delayed, so the stage is unaffected
    assert_ok!(keygen_ceremony.nodes[test_node_id]
        .client
        .ensure_ceremony_at_signing_stage(
            STAGE_FINISHED_OR_NOT_STARTED,
            signing_ceremony.ceremony_id
        ));

    // Complete the keygen by sending the ver5 from each other client to the test node
    keygen_ceremony.distribute_messages(messages);

    // Now that the keygen completed, the rts should have been processed
    assert_ok!(keygen_ceremony.nodes[test_node_id]
        .client
        .ensure_ceremony_at_signing_stage(1, signing_ceremony.ceremony_id));
}

#[tokio::test]
async fn should_ignore_rts_with_unknown_signer_id() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    // Get an id that was not in the keygen and substitute it in the signer list
    let unknown_signer_id = AccountId::new([0; 32]);
    assert!(!signing_ceremony.nodes.keys().contains(&unknown_signer_id));

    // Send the request to sign with a signer specified that is unknown
    let mut sign_info = signing_ceremony.signing_info();
    sign_info.signers[1] = unknown_signer_id;
    let [test_node_id] = signing_ceremony.select_account_ids();

    let test_node = signing_ceremony.nodes.get_mut(&test_node_id).unwrap();

    test_node.client.process_multisig_instruction(
        multisig::MultisigInstruction::Sign(sign_info),
        &mut signing_ceremony.rng,
    );

    // The request to sign should not have triggered a ceremony
    assert_ok!(test_node.client.ensure_ceremony_at_signing_stage(
        STAGE_FINISHED_OR_NOT_STARTED,
        signing_ceremony.ceremony_id
    ));
    assert!(test_node.tag_cache.contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn should_ignore_rts_if_not_participating() {
    let (mut signing_ceremony, non_signing_nodes) = new_signing_ceremony_with_keygen().await;

    // Get a node that participated in generating this key, but one not selected for this signing
    // ceremony
    let (_, mut non_signing_node) = non_signing_nodes.into_iter().next().unwrap();

    // send the request to sign to the non_signing_node
    let sign_info = signing_ceremony.signing_info();
    non_signing_node.client.process_multisig_instruction(
        multisig::MultisigInstruction::Sign(sign_info),
        &mut signing_ceremony.rng,
    );

    // The request to sign should not have started a ceremony
    assert_ok!(non_signing_node.client.ensure_ceremony_at_signing_stage(
        STAGE_FINISHED_OR_NOT_STARTED,
        signing_ceremony.ceremony_id
    ));
    assert!(non_signing_node
        .tag_cache
        .contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn should_ignore_rts_with_incsufficient_number_of_signers() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let [test_node_id] = signing_ceremony.select_account_ids();

    assert_ok!(signing_ceremony
        .nodes
        .get(&test_node_id)
        .unwrap()
        .client
        .ensure_ceremony_at_signing_stage(
            STAGE_FINISHED_OR_NOT_STARTED,
            signing_ceremony.ceremony_id
        ));

    // Send the request to sign with insufficient signer_ids specified
    let mut sign_info = signing_ceremony.signing_info();
    // the request to sign will have one less signer than necessary
    sign_info.signers.pop();

    let node_0 = signing_ceremony.nodes.get_mut(&test_node_id).unwrap();
    node_0.client.process_multisig_instruction(
        multisig::MultisigInstruction::Sign(sign_info),
        &mut signing_ceremony.rng,
    );

    // The request to sign should not have started a ceremony
    assert_ok!(node_0.client.ensure_ceremony_at_signing_stage(
        STAGE_FINISHED_OR_NOT_STARTED,
        signing_ceremony.ceremony_id
    ));
    assert!(node_0.tag_cache.contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn pending_rts_should_expire() {
    let keygen_ceremony_id = 1;
    let signing_ceremony_id = 1;
    let target_account_id = &ACCOUNT_IDS[0];

    let new_keygen_ceremony = || {
        KeygenCeremonyRunner::new(
            new_nodes(ACCOUNT_IDS.clone(), KeygenOptions::allowing_high_pubkey()),
            keygen_ceremony_id,
            Rng::from_seed([8; 32]),
        )
    };

    // Pre-run a deterministic ceremony to get the key id that will be generated by a successful
    // keygeneration
    let keygen_ceremony = new_keygen_ceremony();
    let (key_id, _, _) = helpers::standard_keygen(keygen_ceremony).await;

    // Start a new ceremony, that will be used to test against and run it up to stage 5
    let mut keygen_ceremony = new_keygen_ceremony();
    let messages = keygen_ceremony.request().await;
    run_stages!(
        keygen_ceremony,
        messages,
        keygen::VerifyComm2,
        keygen::SecretShare3,
        keygen::Complaints4,
        keygen::VerifyComplaints5
    );

    // Send request to sign with the key id currently unknown to the client
    let (mut signing_ceremony, _) = SigningCeremonyRunner::new_with_threshold_subset_of_signers(
        keygen_ceremony.nodes,
        signing_ceremony_id,
        key_id,
        MESSAGE_HASH.clone(),
        Rng::from_seed([4; 32]),
    );
    signing_ceremony.request_without_gather();

    signing_ceremony
        .get_mut_node(target_account_id)
        .client
        .force_stage_timeout();

    // TODO: Complete the keygen ceremony above, to ensure we still don't progress,
    // even after the keygen is completed?

    // Should be no pending request to sign.
    assert_ok!(signing_ceremony.nodes[target_account_id]
        .client
        .ensure_ceremony_at_signing_stage(STAGE_FINISHED_OR_NOT_STARTED, signing_ceremony_id));
    assert!(signing_ceremony.nodes[target_account_id]
        .tag_cache
        .contains_tag(REQUEST_TO_SIGN_EXPIRED));
}

// Ignore unexpected messages at all stages. This includes:
// - Messages with stage data that is not the current stage or the next stage
// - Duplicate messages from the same sender AccountId
// - Messages from unknown AccountId or not in the signing ceremony
#[tokio::test]
async fn should_ignore_unexpected_message_for_stage() {
    for_each_stage(
        1..=SIGNING_STAGES,
        || Box::pin(async { new_signing_ceremony_with_keygen().await.0 }),
        standard_signing_coroutine,
        |stage_number, mut ceremony, (_, messages, _)| async move {
            let previous_stage = stage_number - 1;
            let test_node_id = &ACCOUNT_IDS[0];

            let get_messages_for_stage = |stage_index: usize| {
                split_messages_for(messages[stage_index].clone(), test_node_id, &ACCOUNT_IDS[1])
            };

            // Get the messagess from all but one client for the previous stage
            let (msg_from_1, other_msgs) = get_messages_for_stage(previous_stage);
            ceremony.distribute_messages(other_msgs.clone());

            // Receive messages from all unexpected stages (not the current stage or the next)
            for ignored_stage_index in (0..previous_stage).chain(stage_number + 1..SIGNING_STAGES) {
                let (msg_from_1, _) = get_messages_for_stage(ignored_stage_index);
                ceremony.distribute_messages(msg_from_1);
            }

            // We should not have progressed further when receiving unexpected messages
            assert!(
                ceremony.nodes[test_node_id]
                    .client
                    .ensure_ceremony_at_signing_stage(stage_number, ceremony.ceremony_id)
                    .is_ok(),
                "Failed to ignore a message from an unexpected stage"
            );

            // Receive a duplicate message
            ceremony.distribute_messages(other_msgs);
            assert!(
                ceremony.nodes[test_node_id]
                    .client
                    .ensure_ceremony_at_signing_stage(stage_number, ceremony.ceremony_id)
                    .is_ok(),
                "Failed to ignore a duplicate message"
            );

            // Receive a message from an unknown AccountId
            let unknown_id = AccountId::new([0; 32]);
            assert!(!ACCOUNT_IDS.contains(&unknown_id));
            ceremony.distribute_messages(
                msg_from_1
                    .iter()
                    .map(|(_, message)| (unknown_id.clone(), message.clone()))
                    .collect(),
            );
            assert!(
                ceremony.nodes[test_node_id]
                    .client
                    .ensure_ceremony_at_signing_stage(stage_number, ceremony.ceremony_id)
                    .is_ok(),
                "Failed to ignore a message from an unknown account id"
            );

            // Receive a message from a node that is not in the signing ceremony
            let non_participant_id = ACCOUNT_IDS
                .iter()
                .find(|account_id| !ceremony.nodes.contains_key(*account_id))
                .unwrap();
            ceremony.distribute_messages(
                msg_from_1
                    .iter()
                    .map(|(_, message)| (non_participant_id.clone(), message.clone()))
                    .collect(),
            );
            assert!(
                ceremony.nodes[test_node_id]
                    .client
                    .ensure_ceremony_at_signing_stage(stage_number, ceremony.ceremony_id)
                    .is_ok(),
                "Failed to ignore a message from non-participant account id"
            );

            // Receive the last message and advance the stage
            ceremony.distribute_messages(msg_from_1);
            assert!(
                ceremony.nodes[test_node_id]
                    .client
                    .ensure_ceremony_at_signing_stage(
                        if stage_number + 1 > SIGNING_STAGES {
                            // The keygen finished
                            STAGE_FINISHED_OR_NOT_STARTED
                        } else {
                            stage_number + 1
                        },
                        ceremony.ceremony_id
                    )
                    .is_ok(),
                "Failed to proceed to next stage"
            );
        },
    )
    .await;
}

// If the list of signers in the sign request contains a duplicate id, the request should be ignored
#[tokio::test]
async fn should_ignore_rts_with_duplicate_signer() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let [node_0_id] = signing_ceremony.select_account_ids();

    // Send the request to sign with a duplicate id in the list of signers
    let mut sign_info = signing_ceremony.signing_info();
    sign_info.signers[1] = sign_info.signers[2].clone();
    let client = &mut signing_ceremony.nodes.get_mut(&node_0_id).unwrap().client;
    client.process_multisig_instruction(
        MultisigInstruction::Sign(sign_info),
        &mut signing_ceremony.rng,
    );

    // The rts should not have started a ceremony and we should see an error tag
    assert_ok!(client.ensure_ceremony_at_signing_stage(
        STAGE_FINISHED_OR_NOT_STARTED,
        signing_ceremony.ceremony_id
    ));

    assert!(signing_ceremony
        .get_mut_node(&node_0_id)
        .tag_cache
        .contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn should_ignore_rts_with_used_ceremony_id() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let messages = signing_ceremony.request().await;
    let messages = helpers::run_stages!(
        signing_ceremony,
        messages,
        frost::VerifyComm2,
        frost::LocalSig3,
        frost::VerifyLocalSig4
    );
    // Finish a signing ceremony
    signing_ceremony.distribute_messages(messages);
    signing_ceremony.complete().await;

    // Send an rts with the same ceremony id (the default signing ceremony id for tests)
    let sign_info = signing_ceremony.signing_info();
    let node = signing_ceremony.nodes.values_mut().next().unwrap();
    node.client.process_multisig_instruction(
        MultisigInstruction::Sign(sign_info),
        &mut signing_ceremony.rng,
    );

    // The rts should have been ignored
    assert_ok!(node.client.ensure_ceremony_at_signing_stage(
        STAGE_FINISHED_OR_NOT_STARTED,
        signing_ceremony.ceremony_id
    ));
    assert!(node.tag_cache.contains_tag(REQUEST_TO_SIGN_IGNORED));
}

#[tokio::test]
async fn should_ignore_stage_data_with_used_ceremony_id() {
    let (key_id, _, nodes) = helpers::run_keygen(
        helpers::new_nodes(
            ACCOUNT_IDS.clone(),
            client::KeygenOptions::allowing_high_pubkey(),
        ),
        1,
    )
    .await;

    let signing_ceremony_id = 1;

    let mut signing_ceremony = SigningCeremonyRunner::new_with_threshold_subset_of_signers(
        nodes,
        signing_ceremony_id,
        key_id,
        MESSAGE_HASH.clone(),
        Rng::from_seed([4; 32]),
    )
    .0;

    let [node_0_id, node_1_id] = signing_ceremony.select_account_ids();

    let (_, signing_messages) = helpers::standard_signing(&mut signing_ceremony).await;

    // Receive comm1 from a used ceremony id
    signing_ceremony.distribute_message(
        &node_1_id,
        &node_0_id,
        signing_messages
            .stage_1_messages
            .get(&node_1_id)
            .unwrap()
            .get(&node_0_id)
            .unwrap()
            .clone(),
    );

    // The message should have been ignored and no ceremony was started
    // In this case, the ceremony would be unauthorised, so we must check how many signing states exist
    // to see if a unauthorised state was created.
    assert_ok!(signing_ceremony
        .get_mut_node(&node_0_id)
        .client
        .ensure_ceremony_at_signing_stage(STAGE_FINISHED_OR_NOT_STARTED, signing_ceremony_id));
    assert_eq!(
        signing_ceremony
            .get_mut_node(&node_0_id)
            .client
            .ceremony_manager
            .get_signing_states_len(),
        0
    );
}

#[tokio::test]
async fn should_not_consume_ceremony_id_if_unauthorised() {
    let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

    let [node_0_id, node_1_id] = signing_ceremony.select_account_ids();

    // Receive comm1 messages for an unauthorised signing_ceremony
    let message = gen_invalid_signing_comm1(&mut signing_ceremony.rng);
    signing_ceremony.distribute_message(&node_1_id, &node_0_id, message);

    // Check the unauthorised ceremony was created
    assert_eq!(
        signing_ceremony
            .nodes
            .get(&node_0_id)
            .unwrap()
            .client
            .ceremony_manager
            .get_signing_states_len(),
        1
    );

    // Timeout the unauthorised ceremony
    let node_0 = signing_ceremony.get_mut_node(&node_0_id);
    node_0.client.force_stage_timeout();

    // Do a signing ceremony as normal, using the default signing_ceremony
    let messages = signing_ceremony.request().await;
    let messages = helpers::run_stages!(
        signing_ceremony,
        messages,
        frost::VerifyComm2,
        frost::LocalSig3,
        frost::VerifyLocalSig4
    );
    signing_ceremony.distribute_messages(messages);

    // completes successfully, because the ceremony_id was not consumed prior
    signing_ceremony.complete().await;
}

// TODO: Come back and do this such that it signs with all parties
#[tokio::test]
async fn should_sign_with_all_parties() {
    let (key_id, _messages, nodes) = run_keygen(
        new_nodes(ACCOUNT_IDS.clone(), KeygenOptions::allowing_high_pubkey()),
        1,
    )
    .await;

    let mut signing_ceremony = SigningCeremonyRunner::new_with_all_signers(
        nodes,
        1,
        key_id,
        MESSAGE_HASH.clone(),
        Rng::from_seed([4; 32]),
    );

    let messages = signing_ceremony.request().await;
    let messages = helpers::run_stages!(
        signing_ceremony,
        messages,
        frost::VerifyComm2,
        frost::LocalSig3,
        frost::VerifyLocalSig4
    );
    signing_ceremony.distribute_messages(messages);
    signing_ceremony.complete().await;
}

mod timeout {

    // TODO:
    // - Same as the tests for `offline_party_should_be_reported_*`, but the nodes are reported if the majority can't agree on any one value
    // (even if all values are `Some(...)` such as when a node does an inconsistent broadcast)
    // - If timeout before the key is ready, the ceremony should be ignored, but need to ensure that
    //    we return a response

    use super::*;

    /* TODO: Refactor once feature re-enabled

    // If timeout during an "unauthorised" ceremony, we report the nodes that attempted to start it
    // (i.e. whoever send stage data for the ceremony)
    #[tokio::test]
    #[ignore = "functionality disabled as SC does not expect this response"]
    async fn should_report_on_timeout_before_request_to_sign() {
        let mut ctx = helpers::KeygenContext::new();
        let keygen_states = ctx.generate().await;
        let sign_states = ctx.sign().await;

        let id0 = ctx.get_account_id(0);

        let mut c0 = keygen_states
            .key_ready_data()
            .expect("successful keygen")
            .clients[&id0]
            .clone();

        assert_ok!(c0.ensure_ceremony_at_signing_stage(STAGE_FINISHED_OR_NOT_STARTED));

        let bad_array_ids = [ctx.get_account_id(1), ctx.get_account_id(2)];

        for id in &bad_array_ids {
            c0.receive_signing_stage_data(1, &sign_states, id);
        }

        assert_ok!(c0.ensure_ceremony_at_signing_stage(STAGE_FINISHED_OR_NOT_STARTED));

        c0.force_stage_timeout();

        check_blamed_paries(ctx.outcome_receivers.get_mut(&id0).unwrap(), &bad_array_ids).await;
    }
    */

    mod during_regular_stage {

        use super::*;

        // ======================

        // The following tests cover:
        // If timeout during a regular stage, but the majority of nodes can agree on all values,
        // we proceed with the ceremony and use the data received by the majority. If majority of nodes
        // agree on a party timing out in the following stage (broadcast verification), the party gets reported

        #[tokio::test]
        async fn recover_if_party_appears_offline_to_minority_stage1() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            let mut messages = signing_ceremony.request().await;

            let [non_sending_party_id, timed_out_party_id] = signing_ceremony.select_account_ids();

            messages
                .get_mut(&non_sending_party_id)
                .unwrap()
                .remove(&timed_out_party_id);

            // this node doesn't receive non_sending_party's message, so must timeout
            signing_ceremony
                .nodes
                .get_mut(&timed_out_party_id)
                .unwrap()
                .client
                .force_stage_timeout();

            let messages = helpers::run_stages!(
                signing_ceremony,
                messages,
                frost::VerifyComm2,
                frost::LocalSig3,
                frost::VerifyLocalSig4
            );
            signing_ceremony.distribute_messages(messages);
            signing_ceremony.complete().await;
        }

        #[tokio::test]
        async fn recover_if_party_appears_offline_to_minority_stage3() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            let messages = signing_ceremony.request().await;

            let mut messages = helpers::run_stages!(
                signing_ceremony,
                messages,
                frost::VerifyComm2,
                frost::LocalSig3
            );

            let [non_sending_party_id, timed_out_party_id] = signing_ceremony.select_account_ids();

            messages
                .get_mut(&non_sending_party_id)
                .unwrap()
                .remove(&timed_out_party_id);

            // this node doesn't receive non_sending_party's message, so must timeout
            signing_ceremony
                .nodes
                .get_mut(&timed_out_party_id)
                .unwrap()
                .client
                .force_stage_timeout();

            let messages =
                helpers::run_stages!(signing_ceremony, messages, frost::VerifyLocalSig4,);

            signing_ceremony.distribute_messages(messages);
            signing_ceremony.complete().await;
        }

        // ======================

        // ======================

        // The following tests cover:
        // If timeout during a regular stage, and the majority of nodes didn't receive the data
        // from some nodes (i.e. they vote on value `None`), those nodes are reported

        #[tokio::test]
        async fn offline_party_should_be_reported_stage1() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            let messages = signing_ceremony.request().await;

            let [non_sending_party_id] = signing_ceremony.select_account_ids();

            // non sending party sends to no one
            let messages = signing_ceremony
                .run_stage_with_non_sender::<frost::VerifyComm2, _, _>(
                    messages,
                    &non_sending_party_id,
                )
                .await;
            signing_ceremony.distribute_messages(messages);
            signing_ceremony
                .complete_with_error(&[non_sending_party_id])
                .await;
        }

        #[tokio::test]
        async fn offline_party_should_be_reported_stage3() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            let messages = signing_ceremony.request().await;

            let messages = helpers::run_stages!(
                signing_ceremony,
                messages,
                frost::VerifyComm2,
                frost::LocalSig3
            );

            let [non_sending_party_id] = signing_ceremony.select_account_ids();

            // non sending party sends to no one
            let messages = signing_ceremony
                .run_stage_with_non_sender::<frost::VerifyLocalSig4, _, _>(
                    messages,
                    &non_sending_party_id,
                )
                .await;
            signing_ceremony.distribute_messages(messages);
            signing_ceremony
                .complete_with_error(&[non_sending_party_id])
                .await;
        }

        // ======================
    }

    mod during_broadcast_verification_stage {

        use super::*;

        // ======================

        // The following tests cover:
        // If timeout during a broadcast verification stage, and we have enough data, we can recover

        #[tokio::test]
        async fn recover_if_agree_on_values_stage2() {
            let (mut ceremony, _) = new_signing_ceremony_with_keygen().await;

            let bad_node_id = ceremony.nodes.keys().next().unwrap().clone();

            let messages = ceremony.request().await;
            let messages = ceremony
                .run_stage::<frost::VerifyComm2, _, _>(messages)
                .await;

            let messages = ceremony
                .run_stage_with_non_sender::<frost::LocalSig3, _, _>(messages, &bad_node_id)
                .await;

            let messages = ceremony
                .run_stage::<frost::VerifyLocalSig4, _, _>(messages)
                .await;
            ceremony.distribute_messages(messages);
            ceremony.complete().await;
        }

        #[tokio::test]
        async fn recover_if_agree_on_values_stage4() {
            let (mut ceremony, _) = new_signing_ceremony_with_keygen().await;

            let bad_node_id = ceremony.nodes.keys().next().unwrap().clone();

            let messages = ceremony.request().await;
            let messages = helpers::run_stages!(
                ceremony,
                messages,
                frost::VerifyComm2,
                frost::LocalSig3,
                frost::VerifyLocalSig4
            );

            ceremony.distribute_messages_with_non_sender(messages, &bad_node_id);

            ceremony.complete().await;
        }

        // ======================

        // ======================

        // The following tests cover:
        // If timeout during a broadcast verification stage, and we don't have enough data to
        // recover some of the parties messages, we report those parties (note that we can't report
        // the parties that were responsible for the timeout in the first place as we would need
        // another round of "voting" which can also timeout, and then we are back where we started)

        #[tokio::test]
        async fn report_if_cannot_agree_on_values_stage_2() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            // bad party 1 will timeout during a broadcast verification stage. It should be reported
            // bad party 2 will timeout during a broadcast verification stage. It won't get reported.
            // (Ideally it should be reported, but we can't due to the limitations of the protocol)
            let [non_sending_party_id_1, non_sending_party_id_2] =
                signing_ceremony.select_account_ids();

            let messages = signing_ceremony.request().await;

            // bad party one times out here
            let messages = signing_ceremony
                .run_stage_with_non_sender::<frost::VerifyComm2, _, _>(
                    messages,
                    &non_sending_party_id_1,
                )
                .await;

            // bad party two times out here (NB: They are different parties)
            signing_ceremony.distribute_messages_with_non_sender(messages, &non_sending_party_id_2);

            signing_ceremony
                .complete_with_error(&[non_sending_party_id_1])
                .await
        }

        #[tokio::test]
        async fn report_if_cannot_agree_on_values_stage_4() {
            let (mut signing_ceremony, _) = new_signing_ceremony_with_keygen().await;

            // bad party 1 will timeout during a broadcast verification stage. It should be reported
            // bad party 2 will timeout during a broadcast verification stage. It won't get reported.
            // (Ideally it should be reported, but we can't due to the limitations of the protocol
            let [non_sending_party_id_1, non_sending_party_id_2] =
                signing_ceremony.select_account_ids();

            let messages = signing_ceremony.request().await;

            let messages = helpers::run_stages!(
                signing_ceremony,
                messages,
                frost::VerifyComm2,
                frost::LocalSig3
            );

            // bad party one times out here
            let messages = signing_ceremony
                .run_stage_with_non_sender::<frost::VerifyLocalSig4, _, _>(
                    messages,
                    &non_sending_party_id_1,
                )
                .await;

            // bad party two times out here (NB: They are different parties)
            signing_ceremony.distribute_messages_with_non_sender(messages, &non_sending_party_id_2);

            signing_ceremony
                .complete_with_error(&[non_sending_party_id_1])
                .await
        }

        // ======================
    }
}
