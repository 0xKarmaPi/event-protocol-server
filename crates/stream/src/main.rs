mod error;
mod listeners;

use database::{
    repositories::signature_snapshot,
    sea_orm::{ConnectOptions, Database, DatabaseConnection},
};
use dotenv::dotenv;
use futures::StreamExt;
use listeners::{
    _on_claim_rewards, _on_close_event, _on_deploy_event, _on_finish_event, _on_vote_event,
    _on_withdraw,
};
use program::{
    log::{parse_logs, Event},
    PROGRAM_ID_STR,
};
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    pubsub_client::PubsubClientError,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::commitment_config::CommitmentConfig;

#[tokio::main]
async fn main() {
    dotenv().expect("fail to load env");
    let opt = ConnectOptions::new(std::env::var("DATABASE_URL").expect("missing DATABASE_URL env"));

    let ws_url = std::env::var("WS_URL").expect("missing WS_URL env");

    let db = Database::connect(opt)
        .await
        .expect("fail to connect to datbase");

    loop {
        stream(&db, &ws_url)
            .await
            .unwrap_or_else(|error| eprint!("stream error {:#?}", error));
    }
}

async fn stream(db: &DatabaseConnection, ws_url: &str) -> Result<(), PubsubClientError> {
    let client = PubsubClient::new(ws_url).await?;

    let filter = RpcTransactionLogsFilter::Mentions(vec![PROGRAM_ID_STR.to_string()]);

    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::finalized()),
    };

    let (mut notifications, _unsubscribe) = client.logs_subscribe(filter, config).await?;

    println!("🦀 stream is running");

    while let Some(response) = notifications.next().await {
        let signature = response.value.signature;

        // sometime the websocket sends logs duplicate, skip if resolved
        let snapshot = signature_snapshot::find_by_signature(db, &signature)
            .await
            .unwrap_or_else(|e| {
                eprintln!("find snapshot failed {:#?}", e);
                None
            });

        if snapshot.is_none() {
            let logs = response.value.logs;

            if let Some(event) = parse_logs(logs) {
                match event {
                    Event::DeployEvent(event) => _on_deploy_event(db, event, &signature).await,
                    Event::VoteEvent(event) => _on_vote_event(db, event, &signature).await,
                    Event::FinishEvent(event) => _on_finish_event(db, event, &signature).await,
                    Event::CloseEvent(event) => _on_close_event(db, event, &signature).await,
                    Event::ClaimRewards(event) => _on_claim_rewards(db, event, &signature).await,
                    Event::Withdraw(event) => _on_withdraw(db, event, &signature).await,
                }
                .unwrap_or_else(|e| eprintln!("error from listener {:#?}", e));
            }
        } else {
            println!("skip >> {}", signature);
        }
    }

    Ok(())
}
