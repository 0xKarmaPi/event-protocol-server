mod error;
mod listeners;

use database::{
    native_enums::{self, Context},
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
    let mut opt =
        ConnectOptions::new(std::env::var("DATABASE_URL").expect("missing DATABASE_URL env"));

    opt.sqlx_logging(false);

    let ws_url = std::env::var("WS_URL").expect("missing WS_URL env");

    let db = Database::connect(opt)
        .await
        .expect("fail to connect to datbase");

    tracing_subscriber::fmt().init();

    loop {
        stream(&db, &ws_url)
            .await
            .unwrap_or_else(|error| tracing::error!("stream error {:#?}", error));
    }
}

async fn stream(db: &DatabaseConnection, ws_url: &str) -> Result<(), PubsubClientError> {
    let client = PubsubClient::new(ws_url).await?;

    let filter = RpcTransactionLogsFilter::Mentions(vec![PROGRAM_ID_STR.to_string()]);

    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig::finalized()),
    };

    let (mut notifications, _unsubscribe) = client.logs_subscribe(filter, config).await?;

    tracing::info!("🦀 stream is running");

    while let Some(response) = notifications.next().await {
        let signature = response.value.signature;

        // sometime the websocket sends logs duplicate, skip if resolved
        let snapshot = signature_snapshot::find_by_signature(db, &signature)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("find snapshot failed {:#?}", e);
                None
            });

        if snapshot.is_none() {
            let logs = response.value.logs;

            if let Some(event) = parse_logs(logs) {
                let db_event = native_enums::Event::from_ref(&event);

                let result = match event {
                    Event::DeployEvent(event) => _on_deploy_event(db, event).await,
                    Event::VoteEvent(event) => _on_vote_event(db, event).await,
                    Event::FinishEvent(event) => _on_finish_event(db, event).await,
                    Event::CloseEvent(event) => _on_close_event(db, event).await,
                    Event::ClaimRewards(event) => _on_claim_rewards(db, event).await,
                    Event::Withdraw(event) => _on_withdraw(db, event).await,
                };

                match result {
                    Ok(_) => {
                        signature_snapshot::create(
                            db,
                            signature.clone(),
                            db_event.clone(),
                            Context::Stream,
                        )
                        .await
                        .unwrap_or_else(|e| {
                            tracing::error!("fail to create snapshot {:#?}", e);
                        });

                        tracing::info!("done {} >> {}", db_event, signature)
                    }
                    Err(error) => tracing::error!("error from {} listener {}", db_event, error),
                }
            }
        } else {
            tracing::info!("skip >> {}", signature);
        }
    }

    Ok(())
}
