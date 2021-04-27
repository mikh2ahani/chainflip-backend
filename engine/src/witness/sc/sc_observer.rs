use anyhow::Result;
use sp_keyring::AccountKeyring;
use state_chain_runtime::{AccountId, System};
use substrate_subxt::{
    balances::{TransferCallExt, TransferEvent},
    extrinsic::DefaultExtra,
    register_default_type_sizes,
    sp_core::Decode,
    Client, ClientBuilder, EventSubscription,
};

use crate::{
    mq::{nats_client::NatsMQClient, IMQClient, Options, Subject},
    witness::sc::transactions::DataAddedEvent,
};

use super::{runtime::StateChainRuntime, staking::ClaimSigRequested};

pub async fn start() {
    println!("Start the state chain witness with subxt");

    // Not sure if the client should be created here, or above...
    let options = Options {
        url: "http://localhost:9944".to_string(),
    };

    let mq_client = NatsMQClient::connect(options).await.unwrap();

    subscribe_to_events(*mq_client).await.unwrap();
}

// async fn create_subscription<'a, E: substrate_subxt::Event<StateChainRuntime>>(
//     client: Client<StateChainRuntime>,
// ) -> Result<EventSubscription<'a, StateChainRuntime>> {
//     let sub = client.subscribe_finalized_events().await?;
//     let decoder = client.events_decoder();
//     let mut sub = EventSubscription::new(sub, decoder);
//     sub.filter_event::<E>();
//     Ok(sub)
// }

/// Create a substrate subxt client over the StateChainRuntime
async fn create_subxt_client() -> Result<Client<StateChainRuntime>> {
    let client = ClientBuilder::<StateChainRuntime>::new()
        // ideally don't use this, but we currently have a few types that aren't even used, so this is to save
        // defining types for them.
        .skip_type_sizes_check()
        .register_type_size::<AccountId>("AccountId")
        .build()
        .await?;

    Ok(client)
}

async fn subscribe_to_events<M: IMQClient>(mq_client: M) -> Result<()> {
    let client = create_subxt_client().await?;

    // TODO: subscribe_events -> finalised events

    // ===== DataAddedEvents - for easy testing ====
    let sub = client.subscribe_events().await?;
    let decoder = client.events_decoder();
    let mut sub = EventSubscription::new(sub, decoder);
    sub.filter_event::<DataAddedEvent<_>>();

    // SigClaimRequested
    let sig_claim_requested_events = client.subscribe_finalized_events().await?;
    let decoder_more = client.events_decoder();
    let mut sig_claim_requested_events =
        EventSubscription::new(sig_claim_requested_events, decoder_more);
    sig_claim_requested_events.filter_event::<ClaimSigRequested<_>>();

    loop {
        let raw = sub.next().await.unwrap().unwrap();
        println!("Raw event:\n{:#?}", raw);

        let event = DataAddedEvent::<StateChainRuntime>::decode(&mut &raw.data[..]).unwrap();
        println!(
            "The sender of this data is: {} and they sent: '{:?}'",
            event.who,
            String::from_utf8(event.clone().data),
        );

        println!("Adding event: {:#?} to the message queue", event);
        mq_client.publish(Subject::Claim, &event).await.unwrap();

        // Sig claim request
        let raw = sig_claim_requested_events.next().await.unwrap().unwrap();
        println!("the raw event is: {:#?}", raw);
        let event = ClaimSigRequested::<StateChainRuntime>::decode(&mut &raw.data[..]).unwrap();
        println!("The sender is {:#?}", event.who);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn run_test() {
        // let event = substrate_subxt::RawEvent {
        //     module: "Transactions".to_string(),
        //     variant: "DataAdded".to_string(),
        //     data: "Hello".as_bytes().to_owned(),
        // };

        start().await;
    }
}

// RawEvent {
//     module: "Transactions",
//     variant: "DataAdded",
//     data: "8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a480c617364",
// }

// RawEvent {
//     module: "System",
//     variant: "ExtrinsicSuccess",
//     data: "482d7c09000000000200",
// }
// Here's the event to be added: ExtrinsicSuccessEvent {
//     _runtime: PhantomData,
//     info: DispatchInfo {
//         weight: 159133000,
//         class: DispatchClass::Mandatory,
//         pays_fee: Pays::Yes,
//     },
// }
